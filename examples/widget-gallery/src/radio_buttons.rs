use std::fmt::Display;

use floem::{AnyView, IntoView, prelude::*, style::Style, text::FontWeight, theme::StyleThemeExt};
use strum::IntoEnumIterator;

#[derive(PartialEq, Eq, Clone, Copy, strum::EnumIter)]
enum Density {
    Default,
    Comfortable,
    Compact,
}

impl Display for Density {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Density::Default => write!(f, "Default"),
            Density::Comfortable => write!(f, "Comfortable"),
            Density::Compact => write!(f, "Compact"),
        }
    }
}

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

fn radio_row(
    control: impl IntoView + 'static,
    label: impl Into<String> + 'static,
    detail: &'static str,
) -> impl IntoView {
    let label = label.into();
    Stack::horizontal((
        control,
        Stack::vertical((
            label.style(|s| {
                s.font_size(14.0)
                    .font_weight(FontWeight::MEDIUM)
                    .with_theme(|s, t| s.color(t.foreground()))
            }),
            description(detail),
        ))
        .style(|s| s.flex_col().gap(3.0)),
    ))
    .style(|s| s.items_start().gap(8.0))
}

fn section(title_text: &'static str, content: impl IntoView + 'static) -> AnyView {
    Stack::vertical((title(title_text), content))
        .style(|s| s.flex_col().gap(12.0))
        .into_any()
}

fn invalid_radio_style() -> Style {
    Style::new().with_theme(|s, t| {
        s.border_color(t.danger())
            .outline(3.0)
            .outline_color(t.def(|t| t.danger().with_alpha(if t.is_dark { 0.4 } else { 0.2 })))
            .selected(|s| s.border_color(t.primary()))
    })
}

pub fn radio_buttons_view() -> impl IntoView {
    let density = RwSignal::new(Density::Default);
    let disabled_density = RwSignal::new(Density::Comfortable);
    let invalid_density = RwSignal::new(Density::Compact);

    let default_group = Stack::vertical_from_iter(Density::iter().map(move |density_option| {
        radio_row(
            RadioButton::new_rw(density_option, density),
            density_option.to_string(),
            match density_option {
                Density::Default => "Standard spacing and control density.",
                Density::Comfortable => "More relaxed rows for touch targets.",
                Density::Compact => "Dense rows for data-heavy screens.",
            },
        )
    }))
    .style(|s| s.flex_col().gap(12.0));

    let disabled_group = Stack::vertical_from_iter(Density::iter().map(move |density_option| {
        radio_row(
            RadioButton::new_get(density_option, disabled_density).style(|s| s.set_disabled(true)),
            density_option.to_string(),
            "Disabled radio item keeps selected state visible.",
        )
    }))
    .style(|s| s.flex_col().gap(12.0).set_disabled(true));

    let invalid_group = Stack::vertical_from_iter(Density::iter().map(move |density_option| {
        radio_row(
            RadioButton::new_rw(density_option, invalid_density)
                .style(|s| s.apply(invalid_radio_style())),
            density_option.to_string(),
            match density_option {
                Density::Default => "Unchecked invalid item uses destructive border and ring.",
                Density::Comfortable => "Validation styling remains visible across the group.",
                Density::Compact => "Checked invalid item keeps primary border and fill.",
            },
        )
    }))
    .style(|s| s.flex_col().gap(12.0));

    let labeled_group = Stack::vertical_from_iter(Density::iter().map(move |density_option| {
        RadioButton::new_labeled_rw(density_option, density, move || density_option)
    }))
    .style(|s| s.flex_col().gap(8.0));

    Stack::vertical((
        "Radio Group".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        section("Default", default_group),
        section("Disabled", disabled_group),
        section("Validation", invalid_group),
        section("Inline Label", labeled_group),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .max_width(720.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
