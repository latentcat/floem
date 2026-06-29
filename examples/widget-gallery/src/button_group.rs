use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    peniko::Color,
    prelude::*,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

use crate::form::{form, form_item};

#[derive(Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    List,
    Grid,
}

fn group_icon(name: &'static str) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| icon.style(|s| s.size(16.0, 16.0)).into_any())
        .unwrap_or_else(|| Empty::new().style(|s| s.size(16.0, 16.0)).into_any())
}

fn outline_group_button(label: impl IntoView + 'static, radius: (f64, f64, f64, f64)) -> Button {
    Button::new(label).style(move |s| {
        s.height(32.0)
            .padding_horiz(10.0)
            .border(1.0)
            .border_top_left_radius(radius.0)
            .border_top_right_radius(radius.1)
            .border_bottom_right_radius(radius.2)
            .border_bottom_left_radius(radius.3)
            .corner_smoothing(0.6)
            .with_theme(|s, t| {
                s.background(t.background())
                    .border_color(t.border())
                    .color(t.foreground())
                    .hover(|s| s.background(t.muted()))
            })
    })
}

fn icon_button(name: &'static str, radius: (f64, f64, f64, f64)) -> Button {
    outline_group_button(group_icon(name), radius).style(|s| s.width(32.0).padding(0.0))
}

fn group_text(label: &'static str) -> AnyView {
    label
        .style(|s| {
            s.height(32.0)
                .items_center()
                .padding_horiz(10.0)
                .border(1.0)
                .border_top_left_radius(8.0)
                .border_bottom_left_radius(8.0)
                .corner_smoothing(0.6)
                .font_size(14.0)
                .font_weight(FontWeight::MEDIUM)
                .with_theme(|s, t| {
                    s.background(t.muted())
                        .color(t.foreground())
                        .border_color(t.border())
                })
        })
        .into_any()
}

fn vertical_separator() -> Empty {
    Empty::new().style(|s| {
        s.width(1.0)
            .height(32.0)
            .with_theme(|s, t| s.background(t.input()))
    })
}

fn view_mode_button(
    icon_name: &'static str,
    label: &'static str,
    value: ViewMode,
    selected: RwSignal<ViewMode>,
    radius: (f64, f64, f64, f64),
) -> Button {
    outline_group_button(
        Stack::horizontal((group_icon(icon_name), label)).style(|s| s.items_center().gap(6.0)),
        radius,
    )
    .action(move || selected.set(value))
    .style(move |s| {
        s.margin_left(-1.0).apply_if(selected.get() == value, |s| {
            s.with_theme(|s, t| {
                s.background(t.muted())
                    .border_color(t.input())
                    .color(t.foreground())
                    .hover(|s| s.background(t.muted()))
            })
        })
    })
}

pub fn button_group_view() -> impl IntoView {
    let view_mode = RwSignal::new(ViewMode::List);

    form((
        form_item(
            "Horizontal:",
            Stack::horizontal((
                icon_button("bold", (8.0, 0.0, 0.0, 8.0)),
                icon_button("italic", (0.0, 0.0, 0.0, 0.0)).style(|s| s.margin_left(-1.0)),
                icon_button("underline", (0.0, 8.0, 8.0, 0.0)).style(|s| s.margin_left(-1.0)),
            ))
            .style(|s| s.items_center()),
        ),
        form_item(
            "With text:",
            Stack::horizontal((
                group_text("View"),
                vertical_separator(),
                view_mode_button(
                    "list",
                    "List",
                    ViewMode::List,
                    view_mode,
                    (0.0, 0.0, 0.0, 0.0),
                ),
                view_mode_button(
                    "grid-2x2",
                    "Grid",
                    ViewMode::Grid,
                    view_mode,
                    (0.0, 8.0, 8.0, 0.0),
                ),
            ))
            .style(|s| s.items_center()),
        ),
        form_item(
            "Vertical:",
            Stack::vertical((
                outline_group_button("Top", (8.0, 8.0, 0.0, 0.0)),
                outline_group_button("Middle", (0.0, 0.0, 0.0, 0.0)).style(|s| s.margin_top(-1.0)),
                outline_group_button("Bottom", (0.0, 0.0, 8.0, 8.0)).style(|s| s.margin_top(-1.0)),
            ))
            .style(|s| s.width(120.0)),
        ),
    ))
    .style(|s| {
        s.with_theme(|s, t| s.background(t.background()).color(t.foreground()))
            .border_color(Color::TRANSPARENT)
    })
}
