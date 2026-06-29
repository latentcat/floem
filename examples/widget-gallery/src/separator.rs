use floem::{IntoView, prelude::*, theme::StyleThemeExt};

use crate::form::{form, form_item};

fn horizontal_separator() -> Empty {
    Empty::new().style(|s| {
        s.width_full()
            .height(1.0)
            .flex_shrink(0.0)
            .with_theme(|s, t| s.background(t.border()))
    })
}

fn vertical_separator() -> Empty {
    Empty::new().style(|s| {
        s.width(1.0)
            .height(48.0)
            .flex_shrink(0.0)
            .with_theme(|s, t| s.background(t.border()))
    })
}

pub fn separator_view() -> impl IntoView {
    form((
        form_item(
            "Horizontal:",
            Stack::vertical((
                "Account".style(|s| s.font_size(14.0)),
                horizontal_separator(),
                "Billing".style(|s| {
                    s.font_size(14.0)
                        .with_theme(|s, t| s.color(t.muted_foreground()))
                }),
            ))
            .style(|s| s.width(320.0).gap(12.0)),
        ),
        form_item(
            "Vertical:",
            Stack::horizontal((
                "Profile",
                vertical_separator(),
                "Team",
                vertical_separator(),
                "Settings",
            ))
            .style(|s| {
                s.height(48.0)
                    .items_center()
                    .gap(16.0)
                    .font_size(14.0)
                    .with_theme(|s, t| s.color(t.foreground()))
            }),
        ),
    ))
    .style(|s| s.with_theme(|s, t| s.background(t.background()).color(t.foreground())))
}
