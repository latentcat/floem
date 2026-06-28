use floem::{
    prelude::*,
    reactive::RwSignal,
    text::FontWeight,
    unit::{Pct, UnitExt},
    views::{Empty, slider},
};

const PREVIEW_RADIUS: f64 = 72.0;

pub fn corner_smoothing_view() -> impl IntoView {
    let smoothing_pct = RwSignal::new(Pct(60.0));

    let preview = Stack::new((
        Empty::new().style(move |s| {
            s.absolute()
                .inset(0)
                .border_radius(PREVIEW_RADIUS)
                .background(Color::from_rgba8(70, 92, 255, 150))
        }),
        Empty::new().style(move |s| {
            let smoothing = smoothing_pct.get().0 / 100.0;
            s.absolute()
                .inset(0)
                .border_radius(PREVIEW_RADIUS)
                .corner_smoothing(smoothing)
                .background(Color::from_rgba8(255, 122, 54, 150))
        }),
        Stack::horizontal((
            label_pill("circular", Color::from_rgba8(70, 92, 255, 232)),
            label_pill("continuous", Color::from_rgba8(255, 122, 54, 232)),
        ))
        .style(|s| {
            s.absolute()
                .inset_left(18)
                .inset_bottom(18)
                .gap(8)
                .items_center()
        }),
    ))
    .style(|s| {
        s.width(420)
            .height(280)
            .max_width(86.pct())
            .background(Color::from_rgba8(248, 250, 252, 255))
            .border_radius(24)
            .box_shadow_blur(28)
            .box_shadow_color(Color::from_rgba8(15, 23, 42, 46))
            .box_shadow_v_offset(12)
    });

    let controls = control_row(
        "Smoothing",
        slider::Slider::new_rw(smoothing_pct)
            .step(1.0)
            .slider_style(slider_style)
            .style(|s| s.width(280).height(28)),
        Label::derived(move || format!("{:.2}", smoothing_pct.get().0 / 100.0)),
    )
    .style(|s| {
        s.padding(18)
            .width(420)
            .max_width(86.pct())
            .border_radius(18)
            .corner_smoothing(0.6)
            .background(Color::from_rgba8(255, 255, 255, 235))
            .border(1)
            .border_color(Color::from_rgba8(148, 163, 184, 96))
    });

    Stack::vertical((
        Stack::vertical((
            "Continuous Corners".style(|s| {
                s.font_size(24)
                    .font_weight(FontWeight::BOLD)
                    .color(Color::from_rgb8(15, 23, 42))
            }),
            "Figma-style smoothing parameter".style(|s| {
                s.font_size(14)
                    .font_weight(FontWeight::MEDIUM)
                    .color(Color::from_rgb8(71, 85, 105))
            }),
        ))
        .style(|s| s.flex_col().gap(4).items_center()),
        preview,
        controls,
    ))
    .style(|s| {
        s.size_full()
            .min_width(640)
            .min_height(560)
            .flex_col()
            .items_center()
            .justify_center()
            .gap(22)
            .padding(28)
            .background(Color::from_rgb8(241, 245, 249))
    })
}

fn label_pill(label: &'static str, color: Color) -> impl IntoView {
    label.style(move |s| {
        s.font_size(12)
            .font_weight(FontWeight::SEMI_BOLD)
            .color(Color::WHITE)
            .padding_horiz(10)
            .padding_vert(5)
            .border_radius(999)
            .background(color)
    })
}

fn slider_style(s: slider::SliderCustomStyle) -> slider::SliderCustomStyle {
    s.bar_height(6)
        .accent_bar_height(6)
        .bar_radius(999)
        .accent_bar_radius(999)
        .handle_radius(9)
        .bar_color(Color::from_rgba8(203, 213, 225, 255))
        .accent_bar_color(Color::from_rgba8(27, 196, 125, 255))
        .handle_color(Color::from_rgba8(15, 23, 42, 255))
}

fn control_row(
    label: &'static str,
    slider: impl IntoView + 'static,
    value: impl IntoView + 'static,
) -> impl IntoView {
    Stack::horizontal((
        label.style(|s| {
            s.width(82)
                .font_size(13)
                .font_weight(FontWeight::SEMI_BOLD)
                .color(Color::from_rgb8(51, 65, 85))
        }),
        slider,
        value.style(|s| {
            s.width(54)
                .font_size(13)
                .font_weight(FontWeight::MEDIUM)
                .color(Color::from_rgb8(71, 85, 105))
        }),
    ))
    .style(|s| s.gap(12).items_center().justify_between())
}
