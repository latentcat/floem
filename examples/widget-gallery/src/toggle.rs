use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    peniko::Color,
    prelude::*,
    style::{Background, Style, Transition},
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

use crate::form::{form, form_item};

#[derive(Clone, Copy)]
enum ToggleVariant {
    Default,
    Outline,
}

#[derive(Clone, Copy)]
enum ToggleSize {
    Sm,
    Default,
    Lg,
}

impl ToggleSize {
    fn height(self) -> f64 {
        match self {
            Self::Sm => 28.0,
            Self::Default => 32.0,
            Self::Lg => 36.0,
        }
    }

    fn min_width(self) -> f64 {
        match self {
            Self::Sm => 28.0,
            Self::Default => 32.0,
            Self::Lg => 36.0,
        }
    }

    fn font_size(self) -> f64 {
        match self {
            Self::Sm => 12.8,
            Self::Default | Self::Lg => 14.0,
        }
    }
}

fn toggle_icon(name: &'static str, size: ToggleSize) -> AnyView {
    let icon_size = match size {
        ToggleSize::Sm => 14.0,
        ToggleSize::Default | ToggleSize::Lg => 16.0,
    };

    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| icon.style(move |s| s.size(icon_size, icon_size)).into_any())
        .unwrap_or_else(|| {
            Empty::new()
                .style(move |s| s.size(icon_size, icon_size))
                .into_any()
        })
}

fn toggle_style(variant: ToggleVariant, size: ToggleSize, pressed: bool) -> Style {
    let style = Style::new()
        .height(size.height())
        .min_width(size.min_width())
        .padding_horiz(10.0)
        .gap(4.0)
        .border_radius(8.0)
        .corner_smoothing(0.6)
        .font_size(size.font_size())
        .font_weight(FontWeight::MEDIUM)
        .border(1.0)
        .border_color(Color::TRANSPARENT)
        .background(Color::TRANSPARENT)
        .transition(Background, Transition::linear(100.millis()))
        .with_theme(|s, t| {
            s.color(t.foreground())
                .hover(|s| s.background(t.muted()).color(t.foreground()))
        })
        .focus_visible(|s| {
            s.with_theme(|s, t| {
                s.border_color(t.ring())
                    .outline(3.0)
                    .outline_color(t.ring_focus())
            })
        })
        .apply_if(pressed, |s| s.with_theme(|s, t| s.background(t.muted())));

    match variant {
        ToggleVariant::Default => style,
        ToggleVariant::Outline => style.with_theme(|s, t| {
            s.border_color(t.input())
                .background(t.input_background())
                .hover(|s| s.background(t.muted()))
        }),
    }
}

fn toggle_button(
    icon_name: &'static str,
    label: Option<&'static str>,
    variant: ToggleVariant,
    size: ToggleSize,
    initial: bool,
) -> Button {
    let pressed = RwSignal::new(initial);
    let content = match label {
        Some(label) => Stack::horizontal((toggle_icon(icon_name, size), label))
            .style(|s| s.items_center().gap(6.0))
            .into_any(),
        None => toggle_icon(icon_name, size),
    };

    Button::new(content)
        .action(move || pressed.update(|value| *value = !*value))
        .style(move |s| s.apply(toggle_style(variant, size, pressed.get())))
}

pub fn toggle_view() -> impl IntoView {
    form((
        form_item(
            "Default:",
            Stack::horizontal((
                toggle_button(
                    "bold",
                    None,
                    ToggleVariant::Default,
                    ToggleSize::Default,
                    false,
                ),
                toggle_button(
                    "italic",
                    None,
                    ToggleVariant::Default,
                    ToggleSize::Default,
                    true,
                ),
                toggle_button(
                    "underline",
                    Some("Underline"),
                    ToggleVariant::Default,
                    ToggleSize::Default,
                    false,
                ),
            ))
            .style(|s| s.items_center().gap(8.0)),
        ),
        form_item(
            "Outline:",
            Stack::horizontal((
                toggle_button(
                    "align-left",
                    None,
                    ToggleVariant::Outline,
                    ToggleSize::Default,
                    false,
                ),
                toggle_button(
                    "align-center",
                    None,
                    ToggleVariant::Outline,
                    ToggleSize::Default,
                    true,
                ),
                toggle_button(
                    "align-right",
                    None,
                    ToggleVariant::Outline,
                    ToggleSize::Default,
                    false,
                ),
            ))
            .style(|s| s.items_center().gap(8.0)),
        ),
        form_item(
            "Sizes:",
            Stack::horizontal((
                toggle_button(
                    "star",
                    Some("SM"),
                    ToggleVariant::Outline,
                    ToggleSize::Sm,
                    false,
                ),
                toggle_button(
                    "star",
                    Some("Default"),
                    ToggleVariant::Outline,
                    ToggleSize::Default,
                    false,
                ),
                toggle_button(
                    "star",
                    Some("LG"),
                    ToggleVariant::Outline,
                    ToggleSize::Lg,
                    false,
                ),
            ))
            .style(|s| s.items_center().gap(8.0)),
        ),
    ))
    .style(|s| s.with_theme(|s, t| s.background(t.background()).color(t.foreground())))
}
