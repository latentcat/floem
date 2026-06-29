use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    peniko::Color,
    prelude::*,
    style::Style,
    text::FontWeight,
    theme::StyleThemeExt,
};

use crate::form::{form, form_item};

#[derive(Clone, Copy)]
enum BadgeVariant {
    Default,
    Secondary,
    Destructive,
    Outline,
    Ghost,
    Link,
}

fn badge_base_style() -> Style {
    Style::new()
        .height(20.0)
        .padding_horiz(8.0)
        .padding_vert(2.0)
        .gap(4.0)
        .border(1.0)
        .border_radius(100.0)
        .corner_smoothing(0.6)
        .font_size(12.0)
        .font_weight(FontWeight::MEDIUM)
        .items_center()
        .justify_center()
        .flex_shrink(0.0)
        .border_color(Color::TRANSPARENT)
        .focus_visible(|s| {
            s.with_theme(|s, t| {
                s.border_color(t.ring())
                    .outline(3.0)
                    .outline_color(t.ring_focus())
            })
        })
}

fn badge_variant_style(variant: BadgeVariant) -> Style {
    match variant {
        BadgeVariant::Default => Style::new().with_theme(|s, t| {
            s.background(t.primary())
                .color(t.primary_foreground())
                .hover(|s| s.background(t.primary_muted()))
        }),
        BadgeVariant::Secondary => Style::new().with_theme(|s, t| {
            s.background(t.secondary())
                .color(t.secondary_foreground())
                .hover(|s| s.background(t.def(|t| t.secondary().with_alpha(0.8))))
        }),
        BadgeVariant::Destructive => Style::new().with_theme(|s, t| {
            s.background(t.def(|t| t.danger().with_alpha(if t.is_dark { 0.2 } else { 0.1 })))
                .color(t.danger())
                .hover(|s| {
                    s.background(
                        t.def(|t| t.danger().with_alpha(if t.is_dark { 0.3 } else { 0.2 })),
                    )
                })
                .focus_visible(|s| {
                    s.outline_color(
                        t.def(|t| t.danger().with_alpha(if t.is_dark { 0.4 } else { 0.2 })),
                    )
                    .border_color(t.def(|t| t.danger().with_alpha(0.4)))
                })
        }),
        BadgeVariant::Outline => Style::new()
            .background(Color::TRANSPARENT)
            .with_theme(|s, t| {
                s.border_color(t.border())
                    .color(t.foreground())
                    .hover(|s| s.background(t.muted()).color(t.muted_foreground()))
            }),
        BadgeVariant::Ghost => Style::new()
            .background(Color::TRANSPARENT)
            .with_theme(|s, t| {
                s.color(t.foreground()).hover(|s| {
                    s.background(t.def(|t| {
                        if t.is_dark {
                            t.muted().with_alpha(0.5)
                        } else {
                            t.muted()
                        }
                    }))
                    .color(t.muted_foreground())
                })
            }),
        BadgeVariant::Link => Style::new()
            .background(Color::TRANSPARENT)
            .border_color(Color::TRANSPARENT)
            .padding_horiz(0.0)
            .with_theme(|s, t| s.color(t.primary())),
    }
}

fn icon(name: &'static str) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(|s| s.size(12.0, 12.0).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(|s| s.size(12.0, 12.0)).into_any())
}

fn badge(label: &'static str, variant: BadgeVariant) -> AnyView {
    label
        .style(move |s| {
            s.apply(badge_base_style())
                .apply(badge_variant_style(variant))
        })
        .into_any()
}

fn icon_badge(label: &'static str, icon_name: &'static str, variant: BadgeVariant) -> AnyView {
    Stack::horizontal((icon(icon_name), label))
        .style(move |s| {
            s.apply(badge_base_style())
                .padding_left(6.0)
                .apply(badge_variant_style(variant))
        })
        .into_any()
}

pub fn badge_view() -> impl IntoView {
    form((
        form_item(
            "Variants:",
            Stack::horizontal((
                badge("Default", BadgeVariant::Default),
                badge("Secondary", BadgeVariant::Secondary),
                badge("Destructive", BadgeVariant::Destructive),
                badge("Outline", BadgeVariant::Outline),
                badge("Ghost", BadgeVariant::Ghost),
                badge("Link", BadgeVariant::Link),
            ))
            .style(|s| {
                s.items_center()
                    .gap(8.0)
                    .flex_wrap(floem::taffy::FlexWrap::Wrap)
            }),
        ),
        form_item(
            "Icons:",
            Stack::horizontal((
                icon_badge("Verified", "check", BadgeVariant::Default),
                icon_badge("Syncing", "refresh-cw", BadgeVariant::Secondary),
                icon_badge("Error", "triangle-alert", BadgeVariant::Destructive),
                icon_badge("External", "arrow-up-right", BadgeVariant::Outline),
            ))
            .style(|s| {
                s.items_center()
                    .gap(8.0)
                    .flex_wrap(floem::taffy::FlexWrap::Wrap)
            }),
        ),
    ))
    .style(|s| s.with_theme(|s, t| s.background(t.background()).color(t.foreground())))
}
