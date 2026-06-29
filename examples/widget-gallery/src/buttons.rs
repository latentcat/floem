use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    peniko::Color,
    prelude::*,
    style::{Background, Opacity, Style, Transition},
    taffy::FlexWrap,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

#[derive(Clone, Copy)]
enum ButtonVariant {
    Default,
    Secondary,
    Outline,
    Ghost,
    Destructive,
    Link,
}

#[derive(Clone, Copy)]
enum ButtonSize {
    Xs,
    Sm,
    Default,
    Lg,
    IconXs,
    IconSm,
    Icon,
    IconLg,
}

impl ButtonSize {
    fn icon_size(self) -> f64 {
        match self {
            Self::Xs | Self::IconXs => 12.0,
            Self::Sm | Self::IconSm => 14.0,
            _ => 16.0,
        }
    }
}

fn icon(name: &'static str, size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

fn variant_style(variant: ButtonVariant) -> Style {
    match variant {
        ButtonVariant::Default => Style::new(),
        ButtonVariant::Secondary => Style::new().with_theme(|s, t| {
            s.background(t.secondary())
                .color(t.secondary_foreground())
                .hover(|s| s.background(t.button_secondary_hover()))
        }),
        ButtonVariant::Outline => Style::new().with_theme(|s, t| {
            s.background(t.def(|t| {
                if t.is_dark {
                    t.input().with_alpha(0.3)
                } else {
                    t.background
                }
            }))
            .border_color(t.def(|t| if t.is_dark { t.input() } else { t.border() }))
            .color(t.foreground())
            .hover(|s| {
                s.background(t.def(|t| {
                    if t.is_dark {
                        t.input().with_alpha(0.5)
                    } else {
                        t.muted()
                    }
                }))
                .color(t.foreground())
            })
        }),
        ButtonVariant::Ghost => Style::new()
            .background(Color::TRANSPARENT)
            .border_color(Color::TRANSPARENT)
            .with_theme(|s, t| {
                s.color(t.foreground()).hover(|s| {
                    s.background(t.def(|t| {
                        if t.is_dark {
                            t.muted().with_alpha(0.5)
                        } else {
                            t.muted()
                        }
                    }))
                    .color(t.foreground())
                })
            }),
        ButtonVariant::Destructive => Style::new().with_theme(|s, t| {
            s.background(t.def(|t| t.danger().with_alpha(if t.is_dark { 0.2 } else { 0.1 })))
                .color(t.danger())
                .hover(|s| {
                    s.background(
                        t.def(|t| t.danger().with_alpha(if t.is_dark { 0.3 } else { 0.2 })),
                    )
                })
                .focus_visible(|s| {
                    s.border_color(t.def(|t| t.danger().with_alpha(0.4)))
                        .outline_color(
                            t.def(|t| t.danger().with_alpha(if t.is_dark { 0.4 } else { 0.2 })),
                        )
                })
        }),
        ButtonVariant::Link => Style::new()
            .padding_horiz(0.0)
            .background(Color::TRANSPARENT)
            .border_color(Color::TRANSPARENT)
            .with_theme(|s, t| {
                s.color(t.primary())
                    .hover(move |s| s.background(t.def(|_| Color::TRANSPARENT)))
            }),
    }
}

fn size_style(size: ButtonSize) -> Style {
    match size {
        ButtonSize::Xs => Style::new()
            .height(24.0)
            .gap(4.0)
            .padding_horiz(8.0)
            .border_radius(8.0)
            .font_size(12.0),
        ButtonSize::Sm => Style::new()
            .height(28.0)
            .gap(4.0)
            .padding_horiz(10.0)
            .border_radius(8.0)
            .font_size(12.8),
        ButtonSize::Default => Style::new().height(32.0).gap(6.0).padding_horiz(10.0),
        ButtonSize::Lg => Style::new().height(36.0).gap(6.0).padding_horiz(10.0),
        ButtonSize::IconXs => icon_size_style(24.0),
        ButtonSize::IconSm => icon_size_style(28.0),
        ButtonSize::Icon => icon_size_style(32.0),
        ButtonSize::IconLg => icon_size_style(36.0),
    }
    .corner_smoothing(0.6)
}

fn icon_size_style(size: f64) -> Style {
    Style::new()
        .size(size, size)
        .padding(0.0)
        .border_radius(8.0)
}

fn invalid_style() -> Style {
    Style::new().with_theme(|s, t| {
        s.border_color(t.danger())
            .outline(3.0)
            .outline_color(t.def(|t| t.danger().with_alpha(if t.is_dark { 0.4 } else { 0.2 })))
    })
}

fn expanded_style() -> Style {
    Style::new().with_theme(|s, t| {
        s.background(t.muted())
            .color(t.foreground())
            .hover(|s| s.background(t.muted()).color(t.foreground()))
    })
}

fn shadcn_button(label: &'static str, variant: ButtonVariant, size: ButtonSize) -> Button {
    Button::new(label)
        .action(move || println!("{label} clicked"))
        .style(move |s| s.apply(variant_style(variant)).apply(size_style(size)))
}

fn icon_text_button(
    label: &'static str,
    icon_name: &'static str,
    icon_end: bool,
    variant: ButtonVariant,
    size: ButtonSize,
) -> Button {
    let icon_size = size.icon_size();
    let content = if icon_end {
        Stack::horizontal((label.into_any(), icon(icon_name, icon_size)))
    } else {
        Stack::horizontal((icon(icon_name, icon_size), label.into_any()))
    };

    Button::new(content.style(|s| s.items_center().gap(6.0)))
        .action(move || println!("{label} clicked"))
        .style(move |s| s.apply(variant_style(variant)).apply(size_style(size)))
}

fn icon_button(icon_name: &'static str, variant: ButtonVariant, size: ButtonSize) -> Button {
    Button::new(icon(icon_name, size.icon_size()))
        .action(move || println!("{icon_name} clicked"))
        .style(move |s| s.apply(variant_style(variant)).apply(size_style(size)))
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

fn row(content: impl IntoView + 'static) -> AnyView {
    content
        .style(|s| s.items_center().gap(8.0).flex_wrap(FlexWrap::Wrap))
        .into_any()
}

pub fn button_view() -> impl IntoView {
    let variants = row(Stack::horizontal((
        shadcn_button("Default", ButtonVariant::Default, ButtonSize::Default),
        shadcn_button("Secondary", ButtonVariant::Secondary, ButtonSize::Default),
        shadcn_button("Outline", ButtonVariant::Outline, ButtonSize::Default),
        shadcn_button("Ghost", ButtonVariant::Ghost, ButtonSize::Default),
        shadcn_button(
            "Destructive",
            ButtonVariant::Destructive,
            ButtonSize::Default,
        ),
        shadcn_button("Link", ButtonVariant::Link, ButtonSize::Default),
    )));

    let sizes = row(Stack::horizontal((
        shadcn_button("XS", ButtonVariant::Default, ButtonSize::Xs),
        shadcn_button("SM", ButtonVariant::Default, ButtonSize::Sm),
        shadcn_button("Default", ButtonVariant::Default, ButtonSize::Default),
        shadcn_button("LG", ButtonVariant::Default, ButtonSize::Lg),
    )));

    let icons = row(Stack::horizontal((
        icon_button("plus", ButtonVariant::Default, ButtonSize::IconXs),
        icon_button("search", ButtonVariant::Outline, ButtonSize::IconSm),
        icon_button("settings", ButtonVariant::Secondary, ButtonSize::Icon),
        icon_button("trash-2", ButtonVariant::Destructive, ButtonSize::IconLg),
    )));

    let inline_icons = row(Stack::horizontal((
        icon_text_button(
            "Download",
            "download",
            false,
            ButtonVariant::Default,
            ButtonSize::Default,
        ),
        icon_text_button(
            "Open",
            "arrow-up-right",
            true,
            ButtonVariant::Outline,
            ButtonSize::Sm,
        ),
        icon_text_button(
            "Create",
            "plus",
            false,
            ButtonVariant::Secondary,
            ButtonSize::Xs,
        ),
        icon_text_button(
            "Delete",
            "trash-2",
            false,
            ButtonVariant::Destructive,
            ButtonSize::Default,
        ),
    )));

    let states = row(Stack::horizontal((
        shadcn_button("Disabled", ButtonVariant::Default, ButtonSize::Default)
            .style(|s| s.set_disabled(true).set(Opacity, 0.5).unset_cursor()),
        shadcn_button("Invalid", ButtonVariant::Outline, ButtonSize::Default)
            .style(|s| s.apply(invalid_style())),
        shadcn_button("Expanded", ButtonVariant::Outline, ButtonSize::Default)
            .style(|s| s.apply(expanded_style())),
        shadcn_button("Ghost expanded", ButtonVariant::Ghost, ButtonSize::Default)
            .style(|s| s.apply(expanded_style())),
    )));

    Stack::vertical((
        "Button".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        section("Variants", variants),
        section("Sizes", sizes),
        section("Icon sizes", icons),
        section("Inline icons", inline_icons),
        section("States", states),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
            .transition(Background, Transition::linear(100.millis()))
    })
}
