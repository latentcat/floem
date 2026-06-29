use floem::{
    AnyView, IntoView, easing,
    icons::{self as icon_library, IconLibrary},
    prelude::*,
    theme::StyleThemeExt,
};

use crate::form::{form, form_item};

fn spinner(size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, "loader-circle")
        .map(|icon| {
            icon.style(move |s| {
                s.size(size, size)
                    .flex_shrink(0.0)
                    .with_theme(|s, t| s.color(t.foreground()))
            })
            .animation(|a| {
                a.duration(1.seconds())
                    .keyframe(0, |f| f.style(|s| s.rotate(0.0.deg())).ease(easing::Linear))
                    .keyframe(25, |f| {
                        f.style(|s| s.rotate(90.0.deg())).ease(easing::Linear)
                    })
                    .keyframe(50, |f| {
                        f.style(|s| s.rotate(180.0.deg())).ease(easing::Linear)
                    })
                    .keyframe(75, |f| {
                        f.style(|s| s.rotate(270.0.deg())).ease(easing::Linear)
                    })
                    .keyframe(100, |f| {
                        f.style(|s| s.rotate(360.0.deg())).ease(easing::Linear)
                    })
                    .repeat(true)
            })
            .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

pub fn spinner_view() -> impl IntoView {
    form((
        form_item(
            "Sizes:",
            Stack::horizontal((spinner(16.0), spinner(20.0), spinner(24.0)))
                .style(|s| s.items_center().gap(14.0)),
        ),
        form_item(
            "Inline:",
            Stack::horizontal((spinner(16.0), "Loading data...")).style(|s| {
                s.items_center()
                    .gap(8.0)
                    .font_size(14.0)
                    .with_theme(|s, t| s.color(t.muted_foreground()))
            }),
        ),
    ))
    .style(|s| s.with_theme(|s, t| s.background(t.background()).color(t.foreground())))
}
