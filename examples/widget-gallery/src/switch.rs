use floem::{
    AnyView, IntoView,
    prelude::*,
    style::Style,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Decorators, ToggleButton},
};

#[derive(Clone, Copy)]
enum SwitchSize {
    Default,
    Sm,
}

fn size_style(size: SwitchSize) -> Style {
    match size {
        SwitchSize::Default => Style::new().width(32.0).height(18.4),
        SwitchSize::Sm => Style::new().width(24.0).height(14.0),
    }
    .corner_smoothing(0.6)
}

fn invalid_style() -> Style {
    Style::new().with_theme(|s, t| {
        s.border_color(t.danger())
            .outline(3.0)
            .outline_color(t.def(|t| t.danger().with_alpha(if t.is_dark { 0.4 } else { 0.2 })))
    })
}

fn switch_control(
    state: RwSignal<bool>,
    size: SwitchSize,
    disabled: bool,
    invalid: bool,
) -> ToggleButton {
    ToggleButton::new_rw(state)
        .toggle_style(move |s| match size {
            SwitchSize::Default => s
                .circle_rad(8.0)
                .handle_inset(0.0)
                .unchecked_handle_inset(0.0)
                .checked_handle_inset(2.0),
            SwitchSize::Sm => s
                .circle_rad(6.0)
                .handle_inset(0.0)
                .unchecked_handle_inset(0.0)
                .checked_handle_inset(2.0),
        })
        .style(move |s| {
            s.apply(size_style(size))
                .apply_if(disabled, |s| s.set_disabled(true))
                .apply_if(invalid, |s| s.apply(invalid_style()))
        })
}

fn switch_row(
    label: &'static str,
    description: &'static str,
    state: RwSignal<bool>,
    size: SwitchSize,
    disabled: bool,
    invalid: bool,
) -> AnyView {
    Stack::horizontal((
        switch_control(state, size, disabled, invalid),
        Stack::vertical((
            label.style(|s| {
                s.font_size(14.0)
                    .font_weight(FontWeight::MEDIUM)
                    .line_height(1.25)
                    .with_theme(|s, t| s.color(t.foreground()))
            }),
            description.style(|s| {
                s.font_size(13.0)
                    .line_height(1.35)
                    .with_theme(|s, t| s.color(t.muted_foreground()))
            }),
        ))
        .style(|s| s.flex_col().gap(2.0)),
    ))
    .style(|s| s.items_center().gap(10.0))
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

pub fn switch_view() -> impl IntoView {
    let checked = RwSignal::new(true);
    let unchecked = RwSignal::new(false);
    let small_checked = RwSignal::new(true);
    let small_unchecked = RwSignal::new(false);
    let disabled_checked = RwSignal::new(true);
    let disabled_unchecked = RwSignal::new(false);
    let invalid_checked = RwSignal::new(true);
    let invalid_unchecked = RwSignal::new(false);
    let interactive = RwSignal::new(false);

    Stack::vertical((
        "Switch".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        section(
            "Default",
            Stack::vertical((
                switch_row(
                    "Checked",
                    "Primary track with background thumb.",
                    checked,
                    SwitchSize::Default,
                    false,
                    false,
                ),
                switch_row(
                    "Unchecked",
                    "Input track with foreground thumb in dark mode.",
                    unchecked,
                    SwitchSize::Default,
                    false,
                    false,
                ),
            ))
            .style(|s| s.flex_col().gap(12.0)),
        ),
        section(
            "Small",
            Stack::vertical((
                switch_row(
                    "Small checked",
                    "24 x 14 root, 12px thumb.",
                    small_checked,
                    SwitchSize::Sm,
                    false,
                    false,
                ),
                switch_row(
                    "Small unchecked",
                    "Same offset rules as default size.",
                    small_unchecked,
                    SwitchSize::Sm,
                    false,
                    false,
                ),
            ))
            .style(|s| s.flex_col().gap(12.0)),
        ),
        section(
            "States",
            Stack::vertical((
                switch_row(
                    "Disabled checked",
                    "Not allowed cursor and 50% opacity.",
                    disabled_checked,
                    SwitchSize::Default,
                    true,
                    false,
                ),
                switch_row(
                    "Disabled unchecked",
                    "Click events do not change state.",
                    disabled_unchecked,
                    SwitchSize::Default,
                    true,
                    false,
                ),
                switch_row(
                    "Invalid checked",
                    "Destructive border and ring.",
                    invalid_checked,
                    SwitchSize::Default,
                    false,
                    true,
                ),
                switch_row(
                    "Invalid unchecked",
                    "Destructive validation state without changing checked color.",
                    invalid_unchecked,
                    SwitchSize::Default,
                    false,
                    true,
                ),
            ))
            .style(|s| s.flex_col().gap(12.0)),
        ),
        section(
            "Interactive",
            Stack::vertical((
                switch_row(
                    "Notifications",
                    "Click toggles only between checked and unchecked states.",
                    interactive,
                    SwitchSize::Default,
                    false,
                    false,
                ),
                dyn_view(move || {
                    if interactive.get() {
                        "Enabled".to_string()
                    } else {
                        "Disabled".to_string()
                    }
                })
                .style(|s| {
                    s.font_size(13.0)
                        .with_theme(|s, t| s.color(t.muted_foreground()))
                }),
            ))
            .style(|s| {
                s.flex_col()
                    .gap(8.0)
                    .padding(14.0)
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
