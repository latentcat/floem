use floem::{
    AnyView, IntoView, prelude::*, taffy::FlexDirection, text::FontWeight, theme::StyleThemeExt,
    views::Decorators,
};

use crate::shadcn_style::{fixed_square, wrap_text};

fn avatar(initials: &'static str) -> AnyView {
    initials
        .style(|s| {
            s.apply(fixed_square(32.0))
                .items_center()
                .justify_center()
                .border_radius(999.0)
                .font_size(12.0)
                .font_weight(FontWeight::MEDIUM)
                .with_theme(|s, t| {
                    s.background(t.muted())
                        .color(t.muted_foreground())
                        .border_color(t.border())
                })
        })
        .into_any()
}

fn bubble(text: &'static str, outgoing: bool) -> AnyView {
    text.style(move |s| {
        s.max_width(340.0)
            .min_width(0.0)
            .padding_horiz(12.0)
            .padding_vert(8.0)
            .border(1.0)
            .border_radius(12.0)
            .corner_smoothing(0.6)
            .font_size(14.0)
            .line_height(1.45)
            .apply(wrap_text())
            .with_theme(move |s, t| {
                if outgoing {
                    s.background(t.primary())
                        .color(t.primary_foreground())
                        .border_color(t.primary())
                } else {
                    s.background(t.muted())
                        .color(t.foreground())
                        .border_color(t.muted())
                }
            })
    })
    .into_any()
}

fn message(
    name: &'static str,
    time: &'static str,
    initials: &'static str,
    text: &'static str,
    outgoing: bool,
    footer: &'static str,
) -> AnyView {
    Stack::horizontal((
        avatar(initials),
        Stack::vertical((
            Stack::horizontal((
                name.style(|s| {
                    s.font_size(12.0)
                        .font_weight(FontWeight::MEDIUM)
                        .apply(wrap_text())
                        .with_theme(|s, t| s.color(t.muted_foreground()))
                }),
                time.style(|s| {
                    s.font_size(12.0)
                        .flex_shrink(0.0)
                        .with_theme(|s, t| s.color(t.muted_foreground()))
                }),
            ))
            .style(move |s| {
                s.items_center()
                    .gap(8.0)
                    .padding_horiz(12.0)
                    .min_width(0.0)
                    .apply_if(outgoing, |s| s.justify_end())
            }),
            bubble(text, outgoing),
            footer.style(|s| {
                s.font_size(12.0)
                    .font_weight(FontWeight::MEDIUM)
                    .padding_horiz(12.0)
                    .apply(wrap_text())
                    .with_theme(|s, t| s.color(t.muted_foreground()))
            }),
        ))
        .style(move |s| {
            s.flex_col()
                .gap(6.0)
                .min_width(0.0)
                .apply_if(outgoing, |s| s.items_end())
        }),
    ))
    .style(move |s| {
        s.width_full()
            .min_width(0.0)
            .gap(8.0)
            .items_end()
            .apply_if(outgoing, |s| s.flex_direction(FlexDirection::RowReverse))
    })
    .into_any()
}

pub fn message_view() -> impl IntoView {
    Stack::vertical((
        "Message".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::vertical((
            message(
                "Mia",
                "10:24 AM",
                "M",
                "Can you send the updated invoice and the exported chart?",
                false,
                "Delivered",
            ),
            message(
                "You",
                "10:25 AM",
                "YC",
                "Uploaded both files. The chart has the new theme colors.",
                true,
                "Seen",
            ),
            message(
                "Mia",
                "10:26 AM",
                "M",
                "Looks good. I will add it to the project notes.",
                false,
                "Now",
            ),
        ))
        .style(|s| {
            s.width(620.0)
                .max_width_full()
                .flex_col()
                .gap(18.0)
                .padding(18.0)
                .border(1.0)
                .border_radius(12.0)
                .corner_smoothing(0.6)
                .with_theme(|s, t| {
                    s.background(t.card())
                        .border_color(t.border())
                        .color(t.card_foreground())
                })
        }),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
