use floem::{
    AnyView, IntoView,
    prelude::*,
    style::Opacity,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{CheckboxClass, Decorators},
};

fn label_text(text: &'static str) -> impl IntoView {
    text.style(|s| {
        s.font_size(14.0)
            .line_height(1.0)
            .font_weight(FontWeight::MEDIUM)
            .selectable(false)
            .with_theme(|s, t| s.color(t.foreground()))
    })
}

fn helper_text(text: &'static str) -> impl IntoView {
    text.style(|s| {
        s.font_size(13.0)
            .line_height(1.35)
            .with_theme(|s, t| s.color(t.muted_foreground()))
    })
}

fn field(label: impl IntoView + 'static, control: impl IntoView + 'static) -> AnyView {
    Stack::vertical((label, control))
        .style(|s| s.flex_col().gap(8.0).width(300.0))
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
    .style(|s| s.flex_col().gap(12.0))
    .into_any()
}

pub fn label_view() -> impl IntoView {
    let checked = RwSignal::new(true);
    let disabled = RwSignal::new(false);
    let name = RwSignal::new(String::new());

    Stack::vertical((
        "Label".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        section(
            "Form fields",
            Stack::horizontal((
                field(
                    label_text("Display name"),
                    TextInput::new(name)
                        .placeholder("Jane Doe")
                        .style(|s| s.width_full()),
                ),
                field(
                    Stack::horizontal((
                        label_text("Workspace"),
                        "*".style(|s| s.with_theme(|s, t| s.color(t.danger()))),
                    ))
                    .style(|s| s.items_center().gap(4.0)),
                    TextInput::new(name)
                        .placeholder("Acme Inc.")
                        .style(|s| s.width_full()),
                ),
            ))
            .style(|s| {
                s.items_start()
                    .gap(24.0)
                    .flex_wrap(floem::taffy::FlexWrap::Wrap)
            }),
        ),
        section(
            "With controls",
            Stack::vertical((
                Stack::horizontal((
                    Checkbox::new_rw(checked),
                    Stack::vertical((
                        label_text("Enable telemetry"),
                        helper_text("Anonymous diagnostics help improve product quality."),
                    ))
                    .style(|s| s.flex_col().gap(4.0)),
                ))
                .style(|s| {
                    s.items_start()
                        .gap(8.0)
                        .class(CheckboxClass, |s| s.flex_shrink(0.0))
                }),
                Stack::horizontal((
                    Checkbox::new_rw(disabled).style(|s| s.set_disabled(true)),
                    Stack::vertical((
                        label_text("Disabled label").style(|s| s.set(Opacity, 0.5)),
                        helper_text("Peer-disabled labels reduce opacity."),
                    ))
                    .style(|s| s.flex_col().gap(4.0).set(Opacity, 0.5)),
                ))
                .style(|s| s.items_start().gap(8.0).set_disabled(true)),
            ))
            .style(|s| s.flex_col().gap(14.0)),
        ),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
