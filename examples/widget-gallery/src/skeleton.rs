use floem::{IntoView, prelude::*, theme::StyleThemeExt};

fn skeleton_block(width: f64, height: f64) -> Empty {
    Empty::new()
        .style(move |s| {
            s.width(width)
                .height(height)
                .border_radius(6.0)
                .corner_smoothing(0.6)
                .with_theme(|s, t| s.background(t.muted()))
        })
        .animation(|a| {
            a.duration(2.seconds())
                .keyframe(0, |f| f.style(|s| s.opacity(1.0)))
                .keyframe(50, |f| f.style(|s| s.opacity(0.5)))
                .keyframe(100, |f| f.style(|s| s.opacity(1.0)))
                .repeat(true)
        })
}

fn skeleton_circle(size: f64) -> Empty {
    Empty::new()
        .style(move |s| {
            s.size(size, size)
                .border_radius(100.0)
                .corner_smoothing(0.6)
                .with_theme(|s, t| s.background(t.muted()))
        })
        .animation(|a| {
            a.duration(2.seconds())
                .keyframe(0, |f| f.style(|s| s.opacity(1.0)))
                .keyframe(50, |f| f.style(|s| s.opacity(0.5)))
                .keyframe(100, |f| f.style(|s| s.opacity(1.0)))
                .repeat(true)
        })
}

pub fn skeleton_view() -> impl IntoView {
    Stack::vertical((
        Stack::horizontal((
            skeleton_circle(40.0),
            Stack::vertical((skeleton_block(160.0, 16.0), skeleton_block(240.0, 14.0)))
                .style(|s| s.gap(8.0)),
        ))
        .style(|s| s.items_center().gap(12.0)),
        Stack::vertical((
            skeleton_block(420.0, 180.0),
            skeleton_block(360.0, 14.0),
            skeleton_block(300.0, 14.0),
        ))
        .style(|s| s.gap(10.0)),
    ))
    .style(|s| {
        s.padding(30.0)
            .gap(24.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
