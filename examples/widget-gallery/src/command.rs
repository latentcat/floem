use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    peniko::Color,
    prelude::*,
    style::{Background, Foreground, Opacity, Transition},
    taffy::{FlexWrap, Overflow},
    text::FontWeight,
    theme::StyleThemeExt,
    views::Decorators,
};

fn icon(name: &'static str, size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

fn command_input(value: &'static str, disabled: bool) -> AnyView {
    Stack::horizontal((
        TextInput::new(RwSignal::new(value.to_string()))
            .placeholder("Type a command or search...")
            .style(|s| {
                s.flex_grow(1.0)
                    .height(28.0)
                    .border(0.0)
                    .padding_horiz(8.0)
                    .background(Color::TRANSPARENT)
            }),
        icon("search", 16.0).style(|s| s.with_theme(|s, t| s.color(t.muted_foreground()))),
    ))
    .style(move |s| {
        s.height(32.0)
            .items_center()
            .gap(8.0)
            .padding_left(2.0)
            .padding_right(8.0)
            .border(1.0)
            .border_radius(8.0)
            .corner_smoothing(0.6)
            .with_theme(|s, t| {
                s.background(t.def(|t| t.input().with_alpha(0.3)))
                    .border_color(t.def(|t| t.input().with_alpha(0.3)))
                    .disabled(|s| s.set(Opacity, 0.5).unset_cursor())
            })
            .apply_if(disabled, |s| s.set_disabled(true))
    })
    .into_any()
}

fn command_item(
    icon_name: &'static str,
    label: &'static str,
    shortcut: Option<&'static str>,
    selected: bool,
    checked: bool,
    disabled: bool,
) -> AnyView {
    Stack::horizontal((
        icon(icon_name, 16.0),
        label.style(|s| s.font_size(14.0)),
        Empty::new().style(|s| s.flex_grow(1.0)),
        shortcut
            .map(|shortcut| {
                shortcut
                    .style(|s| {
                        s.font_size(12.0)
                            .with_theme(|s, t| s.color(t.muted_foreground()))
                    })
                    .into_any()
            })
            .unwrap_or_else(|| Empty::new().into_any()),
        if checked && shortcut.is_none() {
            icon("check", 16.0)
        } else {
            Empty::new().style(|s| s.size(16.0, 16.0)).into_any()
        },
    ))
    .style(move |s| {
        s.min_height(32.0)
            .items_center()
            .gap(8.0)
            .padding_horiz(8.0)
            .padding_vert(6.0)
            .border_radius(6.0)
            .selectable(false)
            .transition(Background, Transition::linear(100.millis()))
            .transition(Foreground, Transition::linear(100.millis()))
            .with_theme(|s, t| {
                s.color(t.popover_foreground())
                    .hover(|s| s.background(t.muted()).color(t.foreground()))
                    .disabled(|s| s.set(Opacity, 0.5).unset_cursor())
            })
            .apply_if(selected, |s| {
                s.with_theme(|s, t| s.background(t.muted()).color(t.foreground()))
            })
            .apply_if(disabled, |s| s.set_disabled(true))
    })
    .into_any()
}

fn command_group(title: &'static str, items: impl IntoView + 'static) -> AnyView {
    Stack::vertical((
        title.style(|s| {
            s.padding_horiz(8.0)
                .padding_vert(6.0)
                .font_size(12.0)
                .font_weight(FontWeight::MEDIUM)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
        items,
    ))
    .style(|s| s.flex_col().padding(4.0))
    .into_any()
}

fn command_separator() -> AnyView {
    Empty::new()
        .style(|s| {
            s.height(1.0)
                .margin_horiz(-4.0)
                .with_theme(|s, t| s.background(t.border()))
        })
        .into_any()
}

fn command_empty() -> AnyView {
    Stack::vertical((
        icon("search-x", 20.0).style(|s| s.with_theme(|s, t| s.color(t.muted_foreground()))),
        "No results found.".style(|s| {
            s.font_size(14.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
    ))
    .style(|s| s.items_center().gap(8.0).padding_vert(24.0))
    .into_any()
}

fn command_surface(content: impl IntoView + 'static, width: f64) -> AnyView {
    content
        .style(move |s| {
            s.width(width)
                .flex_col()
                .overflow_x(Overflow::Hidden)
                .overflow_y(Overflow::Hidden)
                .padding(4.0)
                .border(1.0)
                .border_radius(12.0)
                .corner_smoothing(0.6)
                .with_theme(|s, t| {
                    s.background(t.popover())
                        .color(t.popover_foreground())
                        .border_color(t.def(|t| t.foreground.with_alpha(0.10)))
                })
        })
        .into_any()
}

fn full_command() -> AnyView {
    command_surface(
        Stack::vertical((
            Stack::vertical((command_input("", false),))
                .style(|s| s.padding(4.0).padding_bottom(0.0)),
            Stack::vertical((
                command_group(
                    "Suggestions",
                    Stack::vertical((
                        command_item("calendar", "Calendar", None, true, false, false),
                        command_item("smile", "Search Emoji", None, false, false, false),
                        command_item("calculator", "Calculator", None, false, false, false),
                    ))
                    .style(|s| s.flex_col()),
                ),
                command_separator(),
                command_group(
                    "Settings",
                    Stack::vertical((
                        command_item("user", "Profile", Some("Cmd P"), false, false, false),
                        command_item("credit-card", "Billing", Some("Cmd B"), false, true, false),
                        command_item("settings", "Settings", Some("Cmd S"), false, false, true),
                    ))
                    .style(|s| s.flex_col()),
                ),
            ))
            .style(|s| s.max_height(288.0).flex_col()),
        )),
        520.0,
    )
}

fn empty_command() -> AnyView {
    command_surface(
        Stack::vertical((
            Stack::vertical((command_input("calendarx", false),))
                .style(|s| s.padding(4.0).padding_bottom(0.0)),
            command_empty(),
        )),
        360.0,
    )
}

fn dialog_command() -> AnyView {
    Stack::vertical((
        Stack::vertical((
            "Command Palette".style(|s| {
                s.font_size(14.0)
                    .font_weight(FontWeight::SEMI_BOLD)
                    .with_theme(|s, t| s.color(t.foreground()))
            }),
            "Search for a command to run...".style(|s| {
                s.font_size(13.0)
                    .with_theme(|s, t| s.color(t.muted_foreground()))
            }),
        ))
        .style(|s| s.flex_col().gap(2.0)),
        command_surface(
            Stack::vertical((
                Stack::vertical((command_input("theme", false),))
                    .style(|s| s.padding(4.0).padding_bottom(0.0)),
                command_group(
                    "Theme",
                    Stack::vertical((
                        command_item("sun", "Light", None, true, false, false),
                        command_item("moon", "Dark", None, false, false, false),
                        command_item("monitor", "System", None, false, true, false),
                    ))
                    .style(|s| s.flex_col()),
                ),
            )),
            420.0,
        ),
    ))
    .style(|s| {
        s.flex_col()
            .gap(10.0)
            .padding(12.0)
            .border(1.0)
            .border_radius(12.0)
            .corner_smoothing(0.6)
            .with_theme(|s, t| s.background(t.card()).border_color(t.border()))
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

pub fn command_view() -> impl IntoView {
    Stack::vertical((
        "Command".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::horizontal((
            section("Surface", full_command()),
            section("Empty", empty_command()),
            section("Dialog", dialog_command()),
        ))
        .style(|s| s.items_start().gap(24.0).flex_wrap(FlexWrap::Wrap)),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
