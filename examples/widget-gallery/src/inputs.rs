use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    peniko::Color,
    prelude::*,
    style::{Background, Style, Transition},
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Decorators, TextInput},
};

fn icon(name: &'static str, size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

fn input_width() -> Style {
    Style::new().width(320.0).min_width(0.0)
}

fn invalid_style() -> Style {
    Style::new().with_theme(|s, t| {
        s.border_color(t.danger())
            .outline(3.0)
            .outline_color(t.def(|t| t.danger().with_alpha(if t.is_dark { 0.4 } else { 0.2 })))
    })
}

fn readonly_style() -> Style {
    Style::new()
        .with_theme(|s, t| {
            s.background(t.def(|t| {
                if t.is_dark {
                    t.input().with_alpha(0.2)
                } else {
                    t.muted().with_alpha(0.45)
                }
            }))
            .color(t.foreground())
        })
        .unset_cursor()
}

fn shadcn_input(
    value: RwSignal<String>,
    placeholder: &'static str,
    disabled: bool,
    invalid: bool,
    readonly: bool,
) -> TextInput {
    TextInput::new(value)
        .placeholder(placeholder)
        .style(move |s| {
            s.apply(input_width())
                .apply_if(disabled, |s| s.set_disabled(true))
                .apply_if(invalid, |s| s.apply(invalid_style()))
                .apply_if(readonly, |s| s.apply(readonly_style()))
        })
}

fn file_input_shell() -> AnyView {
    Stack::horizontal((
        "Choose File".style(|s| {
            s.height(24.0)
                .items_center()
                .padding_horiz(0.0)
                .font_size(14.0)
                .font_weight(FontWeight::MEDIUM)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        "No file selected".style(|s| {
            s.font_size(14.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
    ))
    .style(|s| {
        s.width(320.0)
            .height(32.0)
            .items_center()
            .gap(10.0)
            .padding_horiz(10.0)
            .border(1.0)
            .border_radius(8.0)
            .corner_smoothing(0.6)
            .with_theme(|s, t| {
                s.background(t.input_background())
                    .border_color(t.input())
                    .color(t.foreground())
                    .hover(|s| s.border_color(t.input()))
            })
    })
    .into_any()
}

fn input_with_icon(value: RwSignal<String>) -> AnyView {
    Stack::horizontal((
        icon("search", 16.0),
        TextInput::new(value).placeholder("Search").style(|s| {
            s.width_full()
                .height(30.0)
                .padding_horiz(0.0)
                .border(0.0)
                .background(Color::TRANSPARENT)
                .focus_visible(|s| s.outline(0.0))
        }),
    ))
    .style(|s| {
        s.width(320.0)
            .height(32.0)
            .items_center()
            .gap(8.0)
            .padding_left(10.0)
            .padding_right(8.0)
            .border(1.0)
            .border_radius(8.0)
            .corner_smoothing(0.6)
            .with_theme(|s, t| {
                s.background(t.input_background())
                    .border_color(t.input())
                    .color(t.foreground())
            })
    })
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

fn row(label: &'static str, content: impl IntoView + 'static) -> AnyView {
    Stack::vertical((
        label.style(|s| {
            s.font_size(13.0)
                .font_weight(FontWeight::MEDIUM)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
        content,
    ))
    .style(|s| s.flex_col().gap(6.0))
    .into_any()
}

pub fn text_input_view() -> impl IntoView {
    let empty = RwSignal::new(String::new());
    let filled = RwSignal::new("mia@example.com".to_string());
    let disabled = RwSignal::new(String::new());
    let invalid = RwSignal::new("not-an-email".to_string());
    let readonly = RwSignal::new("readonly@example.com".to_string());
    let number = RwSignal::new("128".to_string());
    let search = RwSignal::new(String::new());

    Stack::vertical((
        "Input".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        section(
            "States",
            Stack::horizontal((
                row(
                    "Placeholder",
                    shadcn_input(empty, "Email", false, false, false),
                ),
                row("Filled", shadcn_input(filled, "Email", false, false, false)),
                row(
                    "Disabled",
                    shadcn_input(disabled, "Disabled input", true, false, false),
                ),
                row(
                    "Invalid",
                    shadcn_input(invalid, "Email", false, true, false),
                ),
                row(
                    "Read only",
                    shadcn_input(readonly, "Email", false, false, true),
                ),
            ))
            .style(|s| s.gap(16.0).flex_wrap(floem::taffy::FlexWrap::Wrap)),
        ),
        section(
            "Types",
            Stack::horizontal((
                row(
                    "Number",
                    shadcn_input(number, "Amount", false, false, false),
                ),
                row("File", file_input_shell()),
                row("Search", input_with_icon(search)),
            ))
            .style(|s| s.gap(16.0).flex_wrap(floem::taffy::FlexWrap::Wrap)),
        ),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
            .transition(Background, Transition::linear(100.millis()))
    })
}
