use floem::{
    AnyView, IntoView,
    prelude::*,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Decorators, TextInput},
};

use crate::shadcn_style::{text_column, wrap_text};

fn label_text(text: &'static str) -> AnyView {
    text.style(|s| {
        s.font_size(14.0)
            .font_weight(FontWeight::MEDIUM)
            .with_theme(|s, t| s.color(t.foreground()))
    })
    .into_any()
}

fn description(text: &'static str) -> AnyView {
    text.style(|s| {
        s.font_size(14.0)
            .line_height(1.45)
            .apply(wrap_text())
            .with_theme(|s, t| s.color(t.muted_foreground()))
    })
    .into_any()
}

fn error_text(text: &'static str) -> AnyView {
    text.style(|s| s.font_size(14.0).with_theme(|s, t| s.color(t.danger())))
        .into_any()
}

fn field_input(placeholder: &'static str, invalid: bool) -> TextInput {
    let value = RwSignal::new(String::new());
    TextInput::new(value)
        .placeholder(placeholder)
        .style(move |s| {
            s.width(320.0).apply_if(invalid, |s| {
                s.with_theme(|s, t| {
                    s.border_color(t.danger()).outline(3.0).outline_color(
                        t.def(|t| t.danger().with_alpha(if t.is_dark { 0.4 } else { 0.2 })),
                    )
                })
            })
        })
}

fn vertical_field() -> AnyView {
    Stack::vertical((
        label_text("Email"),
        field_input("name@example.com", false),
        description("We will only use this email for account notifications."),
    ))
    .style(|s| s.gap(8.0))
    .into_any()
}

fn horizontal_field() -> AnyView {
    Stack::horizontal((
        Stack::vertical((
            label_text("Workspace"),
            description("Shown in your organization switcher."),
        ))
        .style(|s| s.width(180.0).apply(text_column()).gap(4.0)),
        field_input("Acme Inc.", false),
    ))
    .style(|s| s.items_start().gap(16.0).min_width(0.0))
    .into_any()
}

fn invalid_field() -> AnyView {
    Stack::vertical((
        label_text("Username"),
        field_input("username", true),
        error_text("Username must be at least 3 characters."),
    ))
    .style(|s| s.gap(8.0))
    .into_any()
}

fn separator(label: &'static str) -> AnyView {
    Stack::horizontal((
        Empty::new().style(|s| {
            s.height(1.0)
                .flex_grow(1.0)
                .with_theme(|s, t| s.background(t.border()))
        }),
        label.style(|s| {
            s.padding_horiz(8.0)
                .font_size(14.0)
                .flex_shrink(0.0)
                .with_theme(|s, t| s.background(t.background()).color(t.muted_foreground()))
        }),
        Empty::new().style(|s| {
            s.height(1.0)
                .flex_grow(1.0)
                .with_theme(|s, t| s.background(t.border()))
        }),
    ))
    .style(|s| s.width(520.0).items_center())
    .into_any()
}

pub fn field_view() -> impl IntoView {
    Stack::vertical((
        "Profile".style(|s| {
            s.font_size(16.0)
                .font_weight(FontWeight::MEDIUM)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        description("Field groups organize labels, descriptions, controls, and errors."),
        vertical_field(),
        separator("Team"),
        horizontal_field(),
        invalid_field(),
    ))
    .style(|s| {
        s.padding(30.0)
            .gap(20.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
