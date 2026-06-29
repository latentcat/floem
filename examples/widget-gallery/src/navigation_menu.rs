use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    peniko::Color,
    prelude::*,
    style::{Background, Foreground, Opacity, Transition},
    taffy::FlexWrap,
    text::FontWeight,
    theme::StyleThemeExt,
    views::Decorators,
};

use crate::shadcn_style::{text_column, wrap_text};

fn icon(name: &'static str, size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

fn trigger(label: &'static str, open: bool, disabled: bool) -> AnyView {
    Stack::horizontal((
        label.style(|s| s.font_size(14.0).font_weight(FontWeight::MEDIUM)),
        icon("chevron-down", 12.0).style(move |s| {
            s.margin_left(4.0)
                .apply_if(open, |s| s.rotate(180.0.deg()))
                .transition_rotate(Transition::linear(300.millis()))
        }),
    ))
    .style(move |s| {
        s.height(36.0)
            .items_center()
            .justify_center()
            .padding_horiz(10.0)
            .padding_vert(6.0)
            .border_radius(8.0)
            .corner_smoothing(0.6)
            .selectable(false)
            .transition(Background, Transition::linear(100.millis()))
            .transition(Foreground, Transition::linear(100.millis()))
            .with_theme(move |s, t| {
                s.color(t.foreground())
                    .hover(|s| s.background(t.muted()))
                    .focus_visible(|s| {
                        s.outline(3.0)
                            .outline_color(t.ring_focus())
                            .background(t.muted())
                    })
                    .apply_if(open, |s| s.background(t.def(|t| t.muted().with_alpha(0.5))))
                    .disabled(|s| s.set(Opacity, 0.5).unset_cursor())
            })
            .apply_if(disabled, |s| s.set_disabled(true))
    })
    .into_any()
}

fn nav_list(open: &'static str) -> AnyView {
    Stack::horizontal((
        trigger("Getting started", open == "Getting started", false),
        trigger("Components", open == "Components", false),
        trigger("Docs", open == "Docs", false),
        trigger("Disabled", false, true),
    ))
    .style(|s| s.items_center().justify_center().gap(0.0))
    .into_any()
}

fn nav_link(
    title: &'static str,
    description: &'static str,
    icon_name: &'static str,
    active: bool,
) -> AnyView {
    Stack::horizontal((
        icon(icon_name, 16.0),
        Stack::vertical((
            title.style(|s| {
                s.font_size(14.0)
                    .font_weight(FontWeight::MEDIUM)
                    .line_height(1.3)
                    .apply(wrap_text())
            }),
            description.style(|s| {
                s.font_size(13.0)
                    .line_height(1.35)
                    .apply(wrap_text())
                    .with_theme(|s, t| s.color(t.muted_foreground()))
            }),
        ))
        .style(|s| s.apply(text_column()).gap(2.0)),
    ))
    .style(move |s| {
        s.width(236.0)
            .min_width(0.0)
            .items_start()
            .gap(10.0)
            .padding(8.0)
            .border_radius(8.0)
            .corner_smoothing(0.6)
            .selectable(false)
            .transition(Background, Transition::linear(100.millis()))
            .transition(Foreground, Transition::linear(100.millis()))
            .with_theme(move |s, t| {
                s.color(t.popover_foreground())
                    .hover(|s| s.background(t.muted()))
                    .focus_visible(|s| {
                        s.outline(3.0)
                            .outline_color(t.ring_focus())
                            .background(t.muted())
                    })
                    .apply_if(active, |s| {
                        s.background(t.def(|t| t.muted().with_alpha(0.5)))
                    })
            })
    })
    .into_any()
}

fn feature_card() -> AnyView {
    Stack::vertical((
        icon("blocks", 24.0),
        "shadcn/ui".style(|s| {
            s.font_size(16.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .apply(wrap_text())
                .with_theme(|s, t| s.color(t.primary_foreground()))
        }),
        "Copy-paste components built with accessible primitives.".style(|s| {
            s.font_size(13.0)
                .line_height(1.35)
                .apply(wrap_text())
                .with_theme(|s, t| s.color(t.primary_foreground()))
        }),
    ))
    .style(|s| {
        s.width(192.0)
            .height(176.0)
            .flex_col()
            .justify_end()
            .gap(8.0)
            .padding(16.0)
            .border_radius(8.0)
            .corner_smoothing(0.6)
            .with_theme(|s, t| s.background(t.primary()).color(t.primary_foreground()))
    })
    .into_any()
}

fn components_content() -> AnyView {
    Stack::horizontal((
        feature_card(),
        Stack::vertical((
            Stack::horizontal((
                nav_link("Accordion", "Disclosure sections.", "list-collapse", true),
                nav_link(
                    "Button",
                    "Variants and sizes.",
                    "square-mouse-pointer",
                    false,
                ),
            ))
            .style(|s| s.gap(4.0)),
            Stack::horizontal((
                nav_link("Dialog", "Modal surfaces.", "panel-top-open", false),
                nav_link("Tooltip", "Hover hints.", "message-circle-more", false),
            ))
            .style(|s| s.gap(4.0)),
        ))
        .style(|s| s.flex_col().gap(4.0)),
    ))
    .style(|s| s.items_stretch().gap(4.0).padding(4.0))
    .into_any()
}

fn docs_content() -> AnyView {
    Stack::vertical((
        Stack::horizontal((
            nav_link(
                "Installation",
                "Install with pnpm and presets.",
                "terminal",
                true,
            ),
            nav_link("Theming", "Light and dark design tokens.", "palette", false),
        ))
        .style(|s| s.gap(4.0)),
        Stack::horizontal((
            nav_link("Typography", "Type scale and weights.", "type", false),
            nav_link("Motion", "State transitions.", "sparkles", false),
        ))
        .style(|s| s.gap(4.0)),
    ))
    .style(|s| s.flex_col().gap(4.0).padding(4.0))
    .into_any()
}

fn viewport(content: impl IntoView + 'static, width: f64) -> AnyView {
    content
        .style(move |s| {
            s.width(width)
                .flex_col()
                .border(1.0)
                .border_radius(8.0)
                .corner_smoothing(0.6)
                .box_shadow_blur(12.0)
                .box_shadow_color(Color::from_rgb8(0, 0, 0).with_alpha(0.14))
                .with_theme(|s, t| {
                    s.background(t.popover())
                        .color(t.popover_foreground())
                        .border_color(t.def(|t| t.foreground.with_alpha(0.1)))
                })
        })
        .into_any()
}

fn indicator() -> AnyView {
    Stack::vertical((Empty::new().style(|s| {
        s.size(8.0, 8.0)
            .margin_top(-2.0)
            .rotate(45.0.deg())
            .border_top_left_radius(2.0)
            .box_shadow_blur(8.0)
            .box_shadow_color(Color::from_rgb8(0, 0, 0).with_alpha(0.12))
            .with_theme(|s, t| s.background(t.border()))
    }),))
    .style(|s| s.height(6.0).items_center().justify_end())
    .into_any()
}

fn menu_with_viewport(open: &'static str, content: AnyView, width: f64) -> AnyView {
    Stack::vertical((nav_list(open), indicator(), viewport(content, width)))
        .style(|s| s.flex_col().items_center().gap(4.0))
        .into_any()
}

fn no_viewport_content() -> AnyView {
    viewport(
        Stack::vertical((
            nav_link(
                "Overview",
                "Navigation without shared viewport.",
                "map",
                true,
            ),
            nav_link(
                "API Reference",
                "Slots and component props.",
                "braces",
                false,
            ),
            nav_link(
                "Examples",
                "Composed usage patterns.",
                "layout-template",
                false,
            ),
        ))
        .style(|s| s.flex_col().gap(2.0).padding(4.0)),
        280.0,
    )
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

pub fn navigation_menu_view() -> impl IntoView {
    Stack::vertical((
        "Navigation Menu".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::horizontal((
            section(
                "Viewport",
                menu_with_viewport("Components", components_content(), 680.0),
            ),
            section(
                "Docs Content",
                menu_with_viewport("Docs", docs_content(), 500.0),
            ),
            section(
                "Viewport Disabled",
                Stack::vertical((nav_list("Getting started"), no_viewport_content()))
                    .style(|s| s.flex_col().items_center().gap(6.0)),
            ),
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
