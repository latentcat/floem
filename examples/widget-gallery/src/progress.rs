use floem::{AnyView, IntoView, prelude::*, style::Background, theme::StyleThemeExt};

use crate::form::{form, form_item};

fn progress(value: f64, width: f64) -> AnyView {
    let fill_width = (width * value.clamp(0.0, 100.0) / 100.0).round();
    Stack::horizontal((Empty::new().style(move |s| {
        s.width(fill_width)
            .height(4.0)
            .border_radius(100.0)
            .with_theme(|s, t| s.background(t.primary()))
    }),))
    .clip()
    .style(move |s| {
        s.width(width)
            .height(4.0)
            .items_center()
            .border_radius(100.0)
            .corner_smoothing(0.6)
            .with_theme(|s, t| s.background(t.muted()))
            .transition(Background, floem::style::Transition::linear(150.millis()))
    })
    .into_any()
}

fn labeled_progress(label: &'static str, value: f64) -> AnyView {
    Stack::vertical((
        Stack::horizontal((
            label.style(|s| s.font_size(14.0)),
            format!("{value:.0}%").style(|s| {
                s.font_size(13.0)
                    .with_theme(|s, t| s.color(t.muted_foreground()))
            }),
        ))
        .style(|s| s.width(320.0).items_center().justify_between()),
        progress(value, 320.0),
    ))
    .style(|s| s.gap(8.0))
    .into_any()
}

pub fn progress_view() -> impl IntoView {
    form((
        form_item("Default:", labeled_progress("Upload", 68.0)),
        form_item(
            "States:",
            Stack::vertical((
                labeled_progress("Queued", 0.0),
                labeled_progress("Processing", 35.0),
                labeled_progress("Complete", 100.0),
            ))
            .style(|s| s.gap(16.0)),
        ),
    ))
    .style(|s| s.with_theme(|s, t| s.background(t.background()).color(t.foreground())))
}
