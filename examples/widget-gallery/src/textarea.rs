use floem::{
    AnyView, IntoView, prelude::*, text::FontWeight, theme::StyleThemeExt, views::TextInput,
};

use crate::form::{form, form_item};

fn textarea_surface(placeholder: &'static str, invalid: bool, disabled: bool) -> AnyView {
    let value = RwSignal::new(String::new());
    TextInput::new(value)
        .placeholder(placeholder)
        .style(move |s| {
            s.width(420.0)
                .min_height(64.0)
                .height(84.0)
                .items_start()
                .padding_horiz(10.0)
                .padding_vert(8.0)
                .border_radius(8.0)
                .corner_smoothing(0.6)
                .font_size(14.0)
                .set_disabled(disabled)
                .with_theme(move |s, t| {
                    s.background(t.def(move |t| {
                        if disabled {
                            t.input_disabled_background()
                        } else {
                            t.input_background()
                        }
                    }))
                    .border_color(t.input())
                    .color(t.foreground())
                })
                .focus_visible(|s| {
                    s.with_theme(|s, t| {
                        s.border_color(t.ring())
                            .outline(3.0)
                            .outline_color(t.ring_focus())
                    })
                })
                .apply_if(invalid, |s| {
                    s.with_theme(|s, t| {
                        s.border_color(t.danger()).outline_color(
                            t.def(|t| t.danger().with_alpha(if t.is_dark { 0.4 } else { 0.2 })),
                        )
                    })
                })
        })
        .into_any()
}

fn label(label: &'static str) -> AnyView {
    label
        .style(|s| {
            s.font_size(14.0)
                .font_weight(FontWeight::MEDIUM)
                .with_theme(|s, t| s.color(t.foreground()))
        })
        .into_any()
}

pub fn textarea_view() -> impl IntoView {
    form((
        form_item(
            "Default:",
            Stack::vertical((
                label("Message"),
                textarea_surface("Type your message here.", false, false),
            ))
            .style(|s| s.gap(8.0)),
        ),
        form_item(
            "Invalid:",
            Stack::vertical((
                label("Feedback"),
                textarea_surface("Tell us what happened.", true, false),
                "Feedback is required."
                    .style(|s| s.font_size(14.0).with_theme(|s, t| s.color(t.danger()))),
            ))
            .style(|s| s.gap(8.0)),
        ),
        form_item(
            "Disabled:",
            textarea_surface("Disabled textarea", false, true),
        ),
    ))
    .style(|s| s.with_theme(|s, t| s.background(t.background()).color(t.foreground())))
}
