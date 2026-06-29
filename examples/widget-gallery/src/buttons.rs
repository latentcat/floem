use floem::{
    IntoView,
    peniko::Color,
    prelude::*,
    style::{Background, Style, Transition},
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

use crate::form::{form, form_item};

fn secondary_button_style() -> Style {
    Style::new().with_theme(|s, t| {
        s.background(t.secondary())
            .color(t.secondary_foreground())
            .hover(|s| s.background(t.button_secondary_hover()))
    })
}

fn outline_button_style() -> Style {
    Style::new().with_theme(|s, t| {
        s.background(t.def(|t| {
            if t.is_dark {
                Color::from_rgb8(255, 255, 255).with_alpha(0.045)
            } else {
                t.background
            }
        }))
        .border_color(t.def(|t| if t.is_dark { t.input() } else { t.border() }))
        .color(t.foreground())
        .hover(|s| {
            s.background(t.def(|t| {
                if t.is_dark {
                    Color::from_rgb8(255, 255, 255).with_alpha(0.075)
                } else {
                    t.muted()
                }
            }))
            .color(t.foreground())
        })
    })
}

fn ghost_button_style() -> Style {
    Style::new()
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
                .color(t.foreground())
            })
        })
}

fn destructive_button_style() -> Style {
    Style::new().with_theme(|s, t| {
        s.background(t.def(|t| t.danger().with_alpha(if t.is_dark { 0.2 } else { 0.1 })))
            .color(t.danger())
            .hover(|s| {
                s.background(t.def(|t| t.danger().with_alpha(if t.is_dark { 0.3 } else { 0.2 })))
            })
            .focus_visible(|s| {
                s.border_color(t.def(|t| t.danger().with_alpha(0.4)))
                    .outline_color(
                        t.def(|t| t.danger().with_alpha(if t.is_dark { 0.4 } else { 0.2 })),
                    )
            })
    })
}

fn link_button_style() -> Style {
    Style::new()
        .padding_horiz(0.0)
        .background(Color::TRANSPARENT)
        .with_theme(|s, t| {
            s.color(t.primary())
                .hover(move |s| s.background(t.def(|_| Color::TRANSPARENT)))
        })
}

fn button_xs_style() -> Style {
    Style::new()
        .height(24.0)
        .padding_horiz(8.0)
        .gap(4.0)
        .border_radius(8.0)
        .corner_smoothing(0.6)
        .font_size(12.0)
}

fn button_sm_style() -> Style {
    Style::new()
        .height(28.0)
        .padding_horiz(10.0)
        .gap(4.0)
        .border_radius(8.0)
        .corner_smoothing(0.6)
        .font_size(12.8)
}

fn button_lg_style() -> Style {
    Style::new().height(36.0).padding_horiz(10.0).gap(6.0)
}

fn icon_button_style(size: f64, radius: f64) -> Style {
    Style::new()
        .size(size, size)
        .padding(0.0)
        .border_radius(radius)
        .corner_smoothing(0.6)
}

pub fn button_view() -> impl IntoView {
    form((
        form_item(
            "Default:",
            Button::new("Button").action(move || println!("Default button clicked")),
        ),
        form_item(
            "Secondary:",
            Button::new("Secondary")
                .action(|| println!("Secondary button clicked"))
                .style(|s| s.apply(secondary_button_style())),
        ),
        form_item(
            "Outline:",
            Button::new("Outline")
                .action(|| println!("Outline button clicked"))
                .style(|s| s.apply(outline_button_style())),
        ),
        form_item(
            "Ghost:",
            Button::new("Ghost")
                .action(|| println!("Ghost button clicked"))
                .style(|s| s.apply(ghost_button_style())),
        ),
        form_item(
            "Destructive:",
            Button::new("Destructive")
                .action(|| println!("Destructive button clicked"))
                .style(|s| s.apply(destructive_button_style())),
        ),
        form_item(
            "Link:",
            Button::new("Link")
                .action(|| println!("Link button clicked"))
                .style(|s| s.apply(link_button_style())),
        ),
        form_item(
            "Disabled:",
            Button::new("Disabled")
                .style(|s| s.set_disabled(true))
                .action(|| println!("Disabled button clicked")),
        ),
        form_item(
            "Sizes:",
            Stack::horizontal((
                Button::new("XS").style(|s| s.apply(button_xs_style())),
                Button::new("SM").style(|s| s.apply(button_sm_style())),
                Button::new("Default"),
                Button::new("LG").style(|s| s.apply(button_lg_style())),
            ))
            .style(|s| s.items_center().gap(8.0)),
        ),
        form_item(
            "Icon sizes:",
            Stack::horizontal((
                Button::new("+").style(|s| s.apply(icon_button_style(24.0, 8.0))),
                Button::new("+").style(|s| s.apply(icon_button_style(28.0, 8.0))),
                Button::new("+").style(|s| s.apply(icon_button_style(32.0, 10.0))),
                Button::new("+").style(|s| s.apply(icon_button_style(36.0, 10.0))),
            ))
            .style(|s| s.items_center().gap(8.0)),
        ),
    ))
    .style(|s| {
        s.with_theme(|s, t| s.background(t.background()).color(t.foreground()))
            .transition(Background, Transition::linear(100.millis()))
    })
}
