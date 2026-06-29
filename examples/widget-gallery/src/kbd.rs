use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    prelude::*,
    text::FontWeight,
    theme::StyleThemeExt,
};

use crate::form::{form, form_item};

fn kbd(label: &'static str) -> AnyView {
    label
        .style(|s| {
            s.height(20.0)
                .min_width(20.0)
                .padding_horiz(4.0)
                .items_center()
                .justify_center()
                .border_radius(2.0)
                .corner_smoothing(0.6)
                .font_size(12.0)
                .font_weight(FontWeight::MEDIUM)
                .selectable(false)
                .with_theme(|s, t| s.background(t.muted()).color(t.muted_foreground()))
        })
        .into_any()
}

fn kbd_icon(name: &'static str) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(|s| {
                s.height(20.0)
                    .min_width(20.0)
                    .padding_horiz(4.0)
                    .border_radius(2.0)
                    .corner_smoothing(0.6)
                    .with_theme(|s, t| s.background(t.muted()).color(t.muted_foreground()))
                    .class(SvgClass, |s| s.size(12.0, 12.0))
            })
            .into_any()
        })
        .unwrap_or_else(|| kbd("?"))
}

fn plus() -> AnyView {
    "+".style(|s| s.font_size(12.0).with_theme(|s, t| s.color(t.muted_foreground())))
        .into_any()
}

pub fn kbd_view() -> impl IntoView {
    form((
        form_item(
            "Keys:",
            Stack::horizontal((kbd("⌘"), kbd("K"), kbd("Esc"), kbd("Tab"), kbd("Shift")))
                .style(|s| s.items_center().gap(6.0)),
        ),
        form_item(
            "Group:",
            Stack::horizontal((kbd("⌘"), plus(), kbd("Shift"), plus(), kbd("P")))
                .style(|s| s.items_center().gap(4.0)),
        ),
        form_item(
            "Icon:",
            Stack::horizontal((kbd_icon("arrow-up"), plus(), kbd("K")))
                .style(|s| s.items_center().gap(4.0)),
        ),
    ))
    .style(|s| s.with_theme(|s, t| s.background(t.background()).color(t.foreground())))
}
