use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    prelude::*,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators, Empty},
};

fn icon(name: &'static str, size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

fn trigger(label: &'static str, tip: &'static str) -> impl IntoView {
    Button::new(label).tooltip(move || tip)
}

fn rich_trigger(label: &'static str) -> impl IntoView {
    Button::new(label).tooltip(|| {
        Stack::horizontal((icon("sparkles", 14.0), "Generate preview"))
            .style(|s| s.items_center().gap(6.0))
    })
}

fn icon_trigger(icon_name: &'static str, tip: &'static str) -> impl IntoView {
    Button::new(icon(icon_name, 16.0))
        .style(|s| s.size(32.0, 32.0).padding(0.0))
        .tooltip(move || tip)
}

fn kbd(text: &'static str) -> AnyView {
    text.style(|s| {
        s.height(18.0)
            .items_center()
            .padding_horiz(5.0)
            .border_radius(4.0)
            .font_size(11.0)
            .font_weight(FontWeight::MEDIUM)
            .with_theme(|s, t| {
                s.background(t.background())
                    .color(t.foreground())
                    .border_color(t.border())
            })
    })
    .into_any()
}

fn tooltip_arrow() -> AnyView {
    Empty::new()
        .style(|s| {
            s.size(10.0, 10.0)
                .margin_bottom(-5.0)
                .rotate(45.0.deg())
                .border_radius(2.0)
                .with_theme(|s, t| s.background(t.foreground()))
        })
        .into_any()
}

fn tooltip_surface(content: impl IntoView + 'static) -> AnyView {
    Stack::vertical((
        tooltip_arrow(),
        content.style(|s| {
            s.max_width(320.0)
                .items_center()
                .gap(6.0)
                .padding_horiz(12.0)
                .padding_vert(6.0)
                .border_radius(6.0)
                .corner_smoothing(0.6)
                .font_size(12.0)
                .line_height(1.0)
                .with_theme(|s, t| s.background(t.foreground()).color(t.background()))
        }),
    ))
    .style(|s| s.flex_col().items_center().gap(0.0))
    .into_any()
}

fn section(title: &'static str, content: impl IntoView + 'static) -> AnyView {
    Stack::vertical((
        title.style(|s| {
            s.font_size(14.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        content,
    ))
    .style(|s| s.flex_col().gap(10.0))
    .into_any()
}

pub fn tooltip_view() -> impl IntoView {
    Stack::vertical((
        "Tooltip".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        section(
            "Triggers",
            Stack::horizontal((
                trigger("Hover", "Tooltip content"),
                trigger("Quick hint", "Saved changes are synced automatically."),
                trigger("Keyboard", "Command + K"),
                rich_trigger("Rich content"),
            ))
            .style(|s| {
                s.items_center()
                    .gap(10.0)
                    .flex_wrap(floem::taffy::FlexWrap::Wrap)
            }),
        ),
        section(
            "Icon buttons",
            Stack::horizontal((
                icon_trigger("settings", "Settings"),
                icon_trigger("download", "Download"),
                icon_trigger("trash-2", "Delete"),
            ))
            .style(|s| s.items_center().gap(8.0)),
        ),
        section(
            "Surfaces",
            Stack::horizontal((
                tooltip_surface("Tooltip content"),
                tooltip_surface(
                    Stack::horizontal(("Open command palette", kbd("⌘K")))
                        .style(|s| s.items_center().gap(6.0)),
                ),
                tooltip_surface(
                    Stack::horizontal((icon("info", 14.0), "Status is synced"))
                        .style(|s| s.items_center().gap(6.0)),
                ),
            ))
            .style(|s| {
                s.items_start()
                    .gap(18.0)
                    .flex_wrap(floem::taffy::FlexWrap::Wrap)
            }),
        ),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
