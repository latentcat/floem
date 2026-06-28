use peniko::kurbo::{BezPath, Point, Rect, RoundedRectRadii};

const MIN_SMOOTHING: f64 = 0.0;
const MAX_SMOOTHING: f64 = 1.0;
const MIN_ARC_SWEEP: f64 = 1e-6;

pub(crate) fn normalized_corner_smoothing(smoothing: f64) -> f64 {
    smoothing.clamp(MIN_SMOOTHING, MAX_SMOOTHING)
}

pub(crate) fn should_use_continuous_corners(radii: RoundedRectRadii, smoothing: f64) -> bool {
    normalized_corner_smoothing(smoothing) > 0.0
        && radii
            .top_left
            .max(radii.top_right)
            .max(radii.bottom_left)
            .max(radii.bottom_right)
            > 0.0
}

pub(crate) fn continuous_rounded_rect_path(
    rect: Rect,
    radii: RoundedRectRadii,
    smoothing: f64,
) -> BezPath {
    let rect = rect.abs();
    let smoothing = normalized_corner_smoothing(smoothing);
    let radii = normalize_radii_for_continuous_corners(rect, radii, smoothing);

    let top_left = ContinuousCorner::new(rect, radii.top_left, smoothing);
    let top_right = ContinuousCorner::new(rect, radii.top_right, smoothing);
    let bottom_right = ContinuousCorner::new(rect, radii.bottom_right, smoothing);
    let bottom_left = ContinuousCorner::new(rect, radii.bottom_left, smoothing);

    let mut path = BezPath::new();
    path.move_to((rect.x0 + top_left.p, rect.y0));
    path.line_to((rect.x1 - top_right.p, rect.y0));
    append_top_right_corner(&mut path, rect, top_right);
    path.line_to((rect.x1, rect.y1 - bottom_right.p));
    append_bottom_right_corner(&mut path, rect, bottom_right);
    path.line_to((rect.x0 + bottom_left.p, rect.y1));
    append_bottom_left_corner(&mut path, rect, bottom_left);
    path.line_to((rect.x0, rect.y0 + top_left.p));
    append_top_left_corner(&mut path, rect, top_left);
    path.close_path();
    path
}

fn normalize_radii_for_continuous_corners(
    rect: Rect,
    radii: RoundedRectRadii,
    smoothing: f64,
) -> RoundedRectRadii {
    let mut radii = radii.abs();
    radii = scale_radii_to_fit_edges(rect, radii, |radius| radius);

    for _ in 0..3 {
        let tl = ContinuousCorner::new(rect, radii.top_left, smoothing).p;
        let tr = ContinuousCorner::new(rect, radii.top_right, smoothing).p;
        let br = ContinuousCorner::new(rect, radii.bottom_right, smoothing).p;
        let bl = ContinuousCorner::new(rect, radii.bottom_left, smoothing).p;

        let scale = [
            edge_scale(rect.width(), tl + tr),
            edge_scale(rect.width(), bl + br),
            edge_scale(rect.height(), tl + bl),
            edge_scale(rect.height(), tr + br),
        ]
        .into_iter()
        .fold(1.0_f64, f64::min);

        if scale >= 0.999 {
            break;
        }
        radii = radii_map(radii, |radius| radius * scale);
    }

    radii
}

fn scale_radii_to_fit_edges(
    rect: Rect,
    radii: RoundedRectRadii,
    extent: impl Fn(f64) -> f64,
) -> RoundedRectRadii {
    let scale = [
        edge_scale(
            rect.width(),
            extent(radii.top_left) + extent(radii.top_right),
        ),
        edge_scale(
            rect.width(),
            extent(radii.bottom_left) + extent(radii.bottom_right),
        ),
        edge_scale(
            rect.height(),
            extent(radii.top_left) + extent(radii.bottom_left),
        ),
        edge_scale(
            rect.height(),
            extent(radii.top_right) + extent(radii.bottom_right),
        ),
    ]
    .into_iter()
    .fold(1.0_f64, f64::min);

    radii_map(radii, |radius| radius * scale)
}

fn edge_scale(edge: f64, occupied: f64) -> f64 {
    if occupied <= 0.0 {
        1.0
    } else {
        (edge.max(0.0) / occupied).min(1.0)
    }
}

#[derive(Clone, Copy, Debug)]
struct ContinuousCorner {
    radius: f64,
    p: f64,
    angle_bezier: f64,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
}

impl ContinuousCorner {
    fn new(rect: Rect, radius: f64, smoothing: f64) -> Self {
        let radius = radius.max(0.0);
        let shortest_side = rect.width().min(rect.height()).max(0.0);
        let smoothing = normalized_corner_smoothing(smoothing);
        let p = (radius * (1.0 + smoothing)).min(shortest_side * 0.5);
        let quarter_side = shortest_side * 0.25;

        let (angle_circle, angle_bezier) = if radius > quarter_side && quarter_side > 0.0 {
            let change_percentage = ((radius - quarter_side) / quarter_side).clamp(0.0, 1.0);
            (
                90.0 * (1.0 - smoothing * (1.0 - change_percentage)),
                45.0 * smoothing * (1.0 - change_percentage),
            )
        } else {
            (90.0 * (1.0 - smoothing), 45.0 * smoothing)
        };

        let angle_bezier_radians = angle_bezier.to_radians();
        let d_to_c = angle_bezier_radians.tan();
        let longest = radius * (angle_bezier_radians * 0.5).tan();
        let l = (angle_circle.to_radians() * 0.5).sin() * radius * f64::sqrt(2.0);
        let c = longest * angle_bezier_radians.cos();
        let d = c * d_to_c;
        let b = ((p - l) - (1.0 + d_to_c) * c) / 3.0;
        let a = 2.0 * b;

        Self {
            radius,
            p,
            angle_bezier,
            a,
            b,
            c,
            d,
        }
    }
}

fn append_top_right_corner(path: &mut BezPath, rect: Rect, corner: ContinuousCorner) {
    if corner.radius <= 0.0 {
        return;
    }

    let right = rect.x1;
    let top = rect.y0;
    let p = corner.p;
    path.curve_to(
        (right - (p - corner.a), top),
        (right - (p - corner.a - corner.b), top),
        (right - (p - corner.a - corner.b - corner.c), top + corner.d),
    );
    append_arc(
        path,
        Point::new(right - corner.radius, top + corner.radius),
        corner.radius,
        270.0 + corner.angle_bezier,
        90.0 - corner.angle_bezier * 2.0,
    );
    path.curve_to(
        (right, top + (p - corner.a - corner.b)),
        (right, top + (p - corner.a)),
        (right, top + p),
    );
}

fn append_bottom_right_corner(path: &mut BezPath, rect: Rect, corner: ContinuousCorner) {
    if corner.radius <= 0.0 {
        return;
    }

    let right = rect.x1;
    let bottom = rect.y1;
    let p = corner.p;
    path.curve_to(
        (right, bottom - (p - corner.a)),
        (right, bottom - (p - corner.a - corner.b)),
        (
            right - corner.d,
            bottom - (p - corner.a - corner.b - corner.c),
        ),
    );
    append_arc(
        path,
        Point::new(right - corner.radius, bottom - corner.radius),
        corner.radius,
        corner.angle_bezier,
        90.0 - corner.angle_bezier * 2.0,
    );
    path.curve_to(
        (right - (p - corner.a - corner.b), bottom),
        (right - (p - corner.a), bottom),
        (right - p, bottom),
    );
}

fn append_bottom_left_corner(path: &mut BezPath, rect: Rect, corner: ContinuousCorner) {
    if corner.radius <= 0.0 {
        return;
    }

    let left = rect.x0;
    let bottom = rect.y1;
    let p = corner.p;
    path.curve_to(
        (left + (p - corner.a), bottom),
        (left + (p - corner.a - corner.b), bottom),
        (
            left + (p - corner.a - corner.b - corner.c),
            bottom - corner.d,
        ),
    );
    append_arc(
        path,
        Point::new(left + corner.radius, bottom - corner.radius),
        corner.radius,
        90.0 + corner.angle_bezier,
        90.0 - corner.angle_bezier * 2.0,
    );
    path.curve_to(
        (left, bottom - (p - corner.a - corner.b)),
        (left, bottom - (p - corner.a)),
        (left, bottom - p),
    );
}

fn append_top_left_corner(path: &mut BezPath, rect: Rect, corner: ContinuousCorner) {
    if corner.radius <= 0.0 {
        return;
    }

    let left = rect.x0;
    let top = rect.y0;
    let p = corner.p;
    path.curve_to(
        (left, top + (p - corner.a)),
        (left, top + (p - corner.a - corner.b)),
        (left + corner.d, top + (p - corner.a - corner.b - corner.c)),
    );
    append_arc(
        path,
        Point::new(left + corner.radius, top + corner.radius),
        corner.radius,
        180.0 + corner.angle_bezier,
        90.0 - corner.angle_bezier * 2.0,
    );
    path.curve_to(
        (left + (p - corner.a - corner.b), top),
        (left + (p - corner.a), top),
        (left + p, top),
    );
}

fn append_arc(
    path: &mut BezPath,
    center: Point,
    radius: f64,
    start_degrees: f64,
    sweep_degrees: f64,
) {
    if sweep_degrees.abs() <= MIN_ARC_SWEEP || radius <= 0.0 {
        return;
    }

    let segments = (sweep_degrees.abs() / 90.0).ceil().max(1.0) as usize;
    let delta = sweep_degrees.to_radians() / segments as f64;
    let mut start = start_degrees.to_radians();

    for _ in 0..segments {
        let end = start + delta;
        let k = (4.0 / 3.0) * (delta / 4.0).tan();
        let p0 = arc_point(center, radius, start);
        let p3 = arc_point(center, radius, end);
        let c1 = Point::new(
            p0.x - start.sin() * radius * k,
            p0.y + start.cos() * radius * k,
        );
        let c2 = Point::new(p3.x + end.sin() * radius * k, p3.y - end.cos() * radius * k);
        path.curve_to(c1, c2, p3);
        start = end;
    }
}

fn arc_point(center: Point, radius: f64, radians: f64) -> Point {
    Point::new(
        center.x + radians.cos() * radius,
        center.y + radians.sin() * radius,
    )
}

fn radii_map(radii: RoundedRectRadii, f: impl Fn(f64) -> f64) -> RoundedRectRadii {
    RoundedRectRadii {
        top_left: f(radii.top_left),
        top_right: f(radii.top_right),
        bottom_left: f(radii.bottom_left),
        bottom_right: f(radii.bottom_right),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use peniko::kurbo::PathEl;

    #[test]
    fn continuous_path_closes_and_starts_on_top_edge() {
        let path = continuous_rounded_rect_path(
            Rect::new(0.0, 0.0, 200.0, 120.0),
            RoundedRectRadii::from_single_radius(32.0),
            0.6,
        );
        let elements: Vec<_> = path.elements().iter().collect();
        assert!(matches!(elements.first(), Some(PathEl::MoveTo(point)) if point.y == 0.0));
        assert!(matches!(elements.last(), Some(PathEl::ClosePath)));
    }

    #[test]
    fn smoothing_expands_corner_transition_beyond_radius() {
        let rect = Rect::new(0.0, 0.0, 200.0, 120.0);
        let corner = ContinuousCorner::new(rect, 32.0, 0.6);
        assert!(corner.p > 32.0);
        assert!(corner.p <= rect.height() * 0.5);
    }

    #[test]
    fn oversized_continuous_corners_are_scaled_to_fit_edges() {
        let rect = Rect::new(0.0, 0.0, 120.0, 80.0);
        let radii = normalize_radii_for_continuous_corners(
            rect,
            RoundedRectRadii::from_single_radius(60.0),
            0.8,
        );
        let tl = ContinuousCorner::new(rect, radii.top_left, 0.8).p;
        let tr = ContinuousCorner::new(rect, radii.top_right, 0.8).p;
        let bl = ContinuousCorner::new(rect, radii.bottom_left, 0.8).p;
        assert!(tl + tr <= rect.width() + 0.01);
        assert!(tl + bl <= rect.height() + 0.01);
    }
}
