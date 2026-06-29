use floem::{
    AnyView, IntoView, prelude::*, taffy::FlexWrap, text::FontWeight, theme::StyleThemeExt,
    views::Decorators,
};

use crate::shadcn_style::wrap_text;

#[derive(Clone, Copy)]
enum BubbleVariant {
    Default,
    Secondary,
    Muted,
    Tinted,
    Outline,
    Ghost,
    Destructive,
}

impl BubbleVariant {
    fn label(self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::Secondary => "Secondary",
            Self::Muted => "Muted",
            Self::Tinted => "Tinted",
            Self::Outline => "Outline",
            Self::Ghost => "Ghost",
            Self::Destructive => "Destructive",
        }
    }
}

fn bubble(variant: BubbleVariant, align_end: bool, reactions: Option<&'static str>) -> AnyView {
    let content = Stack::vertical((
        format!("{} bubble", variant.label()).style(|s| {
            s.font_size(14.0)
                .line_height(1.45)
                .max_width(296.0)
                .apply(wrap_text())
        }),
        "Message content wraps inside a rounded shadcn surface.".style(move |s| {
            s.font_size(13.0)
                .line_height(1.4)
                .max_width(296.0)
                .apply(wrap_text())
                .with_theme(move |s, t| {
                    if matches!(variant, BubbleVariant::Default) {
                        s.color(t.primary_foreground())
                    } else {
                        s.color(t.muted_foreground())
                    }
                })
        }),
    ))
    .style(move |s| {
        s.flex_col()
            .gap(2.0)
            .max_width(320.0)
            .min_width(0.0)
            .padding_horiz(12.0)
            .padding_vert(8.0)
            .border(1.0)
            .border_radius(12.0)
            .corner_smoothing(0.6)
            .apply_if(matches!(variant, BubbleVariant::Ghost), |s| s.padding(0.0))
            .with_theme(move |s, t| match variant {
                BubbleVariant::Default => s
                    .background(t.primary())
                    .color(t.primary_foreground())
                    .border_color(t.primary()),
                BubbleVariant::Secondary => s
                    .background(t.secondary())
                    .color(t.secondary_foreground())
                    .border_color(t.secondary()),
                BubbleVariant::Muted => s
                    .background(t.muted())
                    .color(t.foreground())
                    .border_color(t.muted()),
                BubbleVariant::Tinted => s
                    .background(
                        t.def(|t| t.primary().with_alpha(if t.is_dark { 0.24 } else { 0.12 })),
                    )
                    .color(t.foreground())
                    .border_color(t.def(|t| t.primary().with_alpha(0.16))),
                BubbleVariant::Outline => s
                    .background(t.background())
                    .color(t.foreground())
                    .border_color(t.border()),
                BubbleVariant::Ghost => s
                    .border_color(t.def(|_| floem::peniko::Color::TRANSPARENT))
                    .background(t.def(|_| floem::peniko::Color::TRANSPARENT))
                    .color(t.foreground()),
                BubbleVariant::Destructive => s
                    .background(t.def(|t| t.danger().with_alpha(if t.is_dark { 0.2 } else { 0.1 })))
                    .color(t.danger())
                    .border_color(t.def(|t| t.danger().with_alpha(0.16))),
            })
    });

    Stack::vertical((
        content,
        reactions
            .map(|value| {
                value
                    .style(|s| {
                        s.height(22.0)
                            .items_center()
                            .padding_horiz(8.0)
                            .border_radius(999.0)
                            .font_size(13.0)
                            .with_theme(|s, t| {
                                s.background(t.muted())
                                    .color(t.foreground())
                                    .border_color(t.card())
                            })
                    })
                    .into_any()
            })
            .unwrap_or_else(|| Empty::new().into_any()),
    ))
    .style(move |s| {
        s.flex_col()
            .gap(4.0)
            .max_width(360.0)
            .min_width(0.0)
            .apply_if(align_end, |s| s.items_end())
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

pub fn bubble_view() -> impl IntoView {
    Stack::vertical((
        "Bubble".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        section(
            "Variants",
            Stack::horizontal((
                bubble(BubbleVariant::Default, false, Some("2")),
                bubble(BubbleVariant::Secondary, true, None),
                bubble(BubbleVariant::Muted, false, None),
                bubble(BubbleVariant::Tinted, true, Some("ok")),
                bubble(BubbleVariant::Outline, false, None),
                bubble(BubbleVariant::Ghost, true, None),
                bubble(BubbleVariant::Destructive, false, Some("!")),
            ))
            .style(|s| s.gap(14.0).items_start().flex_wrap(FlexWrap::Wrap)),
        ),
        section(
            "Group",
            Stack::vertical((
                bubble(BubbleVariant::Muted, false, None),
                bubble(BubbleVariant::Default, true, None),
                bubble(BubbleVariant::Secondary, true, Some("1")),
            ))
            .style(|s| {
                s.width(520.0)
                    .flex_col()
                    .gap(8.0)
                    .padding(16.0)
                    .border(1.0)
                    .border_radius(12.0)
                    .corner_smoothing(0.6)
                    .with_theme(|s, t| {
                        s.background(t.card())
                            .border_color(t.border())
                            .color(t.card_foreground())
                    })
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
