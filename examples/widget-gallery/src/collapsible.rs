use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    prelude::*,
    style::{Background, Foreground, Opacity, Transition},
    taffy::style::Display,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

fn icon(name: &'static str, size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

fn code_line(text: &'static str) -> impl IntoView {
    text.style(|s| {
        s.height(32.0)
            .items_center()
            .padding_horiz(12.0)
            .border(1.0)
            .border_radius(8.0)
            .corner_smoothing(0.6)
            .font_family("monospace".to_owned())
            .font_size(13.0)
            .with_theme(|s, t| {
                s.background(t.muted())
                    .border_color(t.border())
                    .color(t.foreground())
            })
    })
}

fn collapsible_card(
    title: &'static str,
    subtitle: &'static str,
    default_open: bool,
    disabled: bool,
) -> AnyView {
    let open = RwSignal::new(default_open);

    Stack::vertical((
        Stack::horizontal((
            Stack::vertical((
                title.style(|s| {
                    s.font_size(14.0)
                        .font_weight(FontWeight::SEMI_BOLD)
                        .with_theme(|s, t| s.color(t.foreground()))
                }),
                subtitle.style(|s| {
                    s.font_size(13.0)
                        .with_theme(|s, t| s.color(t.muted_foreground()))
                }),
            ))
            .style(|s| s.flex_col().gap(2.0)),
            Button::new(dyn_view(move || {
                if open.get() {
                    icon("chevron-up", 16.0)
                } else {
                    icon("chevron-down", 16.0)
                }
            }))
            .action(move || {
                if !disabled {
                    open.update(|value| *value = !*value)
                }
            })
            .style(move |s| {
                s.size(32.0, 32.0)
                    .padding(0.0)
                    .transition(Background, Transition::linear(100.millis()))
                    .transition(Foreground, Transition::linear(100.millis()))
                    .with_theme(|s, t| {
                        s.background(t.def(|_| floem::peniko::Color::TRANSPARENT))
                            .border_color(t.def(|_| floem::peniko::Color::TRANSPARENT))
                            .color(t.muted_foreground())
                            .hover(|s| s.background(t.muted()).color(t.foreground()))
                            .disabled(|s| s.set(Opacity, 0.5).unset_cursor())
                    })
                    .apply_if(disabled, |s| s.set_disabled(true))
            }),
        ))
        .style(|s| s.items_center().justify_between().gap(16.0)),
        Stack::vertical((
            code_line("@radix-ui/react-collapsible"),
            code_line("@radix-ui/react-tooltip"),
            code_line("@radix-ui/react-dropdown-menu"),
        ))
        .style(move |s| {
            s.flex_col()
                .gap(8.0)
                .padding_top(12.0)
                .apply_if(!open.get(), |s| s.display(Display::None))
        }),
    ))
    .style(move |s| {
        s.width(420.0)
            .flex_col()
            .padding(16.0)
            .border(1.0)
            .border_radius(8.0)
            .corner_smoothing(0.6)
            .with_theme(move |s, t| {
                s.background(t.card())
                    .border_color(t.border())
                    .apply_if(disabled, |s| s.set(Opacity, 0.6))
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

pub fn collapsible_view() -> impl IntoView {
    Stack::vertical((
        "Collapsible".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        section(
            "Basic",
            Stack::horizontal((
                collapsible_card(
                    "Dependencies",
                    "Open to inspect installed packages.",
                    true,
                    false,
                ),
                collapsible_card(
                    "Optional packages",
                    "Starts closed with the same trigger pattern.",
                    false,
                    false,
                ),
                collapsible_card(
                    "Disabled",
                    "Trigger is inert and content remains closed.",
                    false,
                    true,
                ),
            ))
            .style(|s| {
                s.items_start()
                    .gap(24.0)
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
