use floem::{AnyView, IntoView, prelude::*, text::FontWeight, theme::StyleThemeExt};

use crate::form::{form, form_item};

fn ratio_box(label: &'static str, ratio: f32, width: f64) -> AnyView {
    Stack::new((label.style(|s| {
        s.font_size(14.0)
            .font_weight(FontWeight::MEDIUM)
            .with_theme(|s, t| s.color(t.foreground()))
    }),))
    .clip()
    .style(move |s| {
        s.width(width)
            .aspect_ratio(ratio)
            .items_center()
            .justify_center()
            .border_radius(12.0)
            .corner_smoothing(0.6)
            .with_theme(|s, t| {
                s.background(t.muted())
                    .color(t.foreground())
                    .border_color(t.border())
            })
    })
    .into_any()
}

pub fn aspect_ratio_view() -> impl IntoView {
    form((
        form_item("16 / 9:", ratio_box("16:9", 16.0 / 9.0, 360.0)),
        form_item("4 / 3:", ratio_box("4:3", 4.0 / 3.0, 260.0)),
        form_item("1 / 1:", ratio_box("1:1", 1.0, 160.0)),
    ))
    .style(|s| s.with_theme(|s, t| s.background(t.background()).color(t.foreground())))
}
