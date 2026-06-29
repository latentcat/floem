use floem::{
    AnyView, IntoView,
    prelude::*,
    style::Style,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{CheckboxClass, Decorators},
};

// Kept public because the Lists gallery uses it for a custom checkbox example.
pub const CROSS_SVG: &str = r##"
<svg width="800px" height="800px" viewBox="0 0 24 24" fill="none" xmlns="http://www.w3.org/2000/svg">
<path d="M19 5L5 19M5.00001 5L19 19" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
</svg>
"##;

fn title(text: &'static str) -> impl IntoView {
    text.style(|s| {
        s.font_size(14.0)
            .font_weight(FontWeight::SEMI_BOLD)
            .with_theme(|s, t| s.color(t.foreground()))
    })
}

fn description(text: &'static str) -> impl IntoView {
    text.style(|s| {
        s.font_size(13.0)
            .line_height(1.35)
            .with_theme(|s, t| s.color(t.muted_foreground()))
    })
}

fn field(control: impl IntoView + 'static, label: &'static str, detail: &'static str) -> AnyView {
    Stack::horizontal((
        control,
        Stack::vertical((title(label), description(detail))).style(|s| s.flex_col().gap(3.0)),
    ))
    .style(|s| s.items_start().gap(8.0))
    .into_any()
}

fn section(title_text: &'static str, content: impl IntoView + 'static) -> AnyView {
    Stack::vertical((title(title_text), content))
        .style(|s| s.flex_col().gap(12.0))
        .into_any()
}

fn invalid_checkbox_style() -> Style {
    Style::new().with_theme(|s, t| {
        s.border_color(t.danger())
            .outline(3.0)
            .outline_color(t.def(|t| t.danger().with_alpha(if t.is_dark { 0.4 } else { 0.2 })))
            .selected(|s| s.border_color(t.primary()))
    })
}

pub fn checkbox_view() -> impl IntoView {
    let checked = RwSignal::new(true);
    let unchecked = RwSignal::new(false);
    let notifications = RwSignal::new(true);
    let invalid_unchecked = RwSignal::new(false);
    let invalid_checked = RwSignal::new(true);

    let invalid_unchecked_control =
        Checkbox::new_rw(invalid_unchecked).style(|s| s.apply(invalid_checkbox_style()));
    let invalid_checked_control =
        Checkbox::new_rw(invalid_checked).style(|s| s.apply(invalid_checkbox_style()));

    Stack::vertical((
        "Checkbox".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        section(
            "States",
            Stack::horizontal((
                field(
                    Checkbox::new_rw(unchecked),
                    "Unchecked",
                    "Default empty state with input border.",
                ),
                field(
                    Checkbox::new_rw(checked),
                    "Checked",
                    "Primary background and foreground check.",
                ),
                field(
                    checkbox(|| false).style(|s| s.set_disabled(true)),
                    "Disabled",
                    "Reduced opacity and disabled cursor.",
                ),
                field(
                    checkbox(|| true).style(|s| s.set_disabled(true)),
                    "Disabled checked",
                    "Checked state keeps primary fill while disabled.",
                ),
            ))
            .style(|s| s.flex_col().gap(14.0)),
        ),
        section(
            "Validation",
            Stack::vertical((
                field(
                    invalid_unchecked_control,
                    "Invalid unchecked",
                    "Destructive border and ring on an unchecked control.",
                ),
                field(
                    invalid_checked_control,
                    "Invalid checked",
                    "Checked invalid keeps the primary border and fill.",
                ),
            ))
            .style(|s| {
                s.flex_col()
                    .gap(14.0)
                    .class(CheckboxClass, |s| s.flex_shrink(0.0))
            }),
        ),
        section(
            "Field",
            Stack::vertical((field(
                Checkbox::labeled_rw(notifications, || "Enable notifications"),
                "Label composition",
                "The label row uses shadcn spacing without a separate card surface.",
            ),))
            .style(|s| {
                s.flex_col()
                    .gap(14.0)
                    .class(CheckboxClass, |s| s.flex_shrink(0.0))
            }),
        ),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .max_width(720.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
