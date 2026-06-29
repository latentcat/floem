use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    peniko::Color,
    prelude::*,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

use crate::shadcn_style::wrap_text;

#[derive(Clone, Copy)]
enum DrawerSide {
    Bottom,
    Top,
    Left,
    Right,
}

impl DrawerSide {
    fn label(self) -> &'static str {
        match self {
            Self::Bottom => "Bottom",
            Self::Top => "Top",
            Self::Left => "Left",
            Self::Right => "Right",
        }
    }

    fn title(self) -> &'static str {
        match self {
            Self::Bottom => "Bottom drawer",
            Self::Top => "Top drawer",
            Self::Left => "Left drawer",
            Self::Right => "Right drawer",
        }
    }
}

fn icon(name: &'static str, size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

fn outline_button(label: &'static str) -> Button {
    Button::new(label).style(|s| {
        s.border(1.0).with_theme(|s, t| {
            s.background(t.background())
                .border_color(t.input())
                .color(t.foreground())
                .hover(|s| s.background(t.muted()))
        })
    })
}

fn handle() -> AnyView {
    Empty::new()
        .style(|s| {
            s.width(100.0)
                .height(4.0)
                .margin_top(16.0)
                .border_radius(100.0)
                .with_theme(|s, t| s.background(t.muted()))
        })
        .into_any()
}

fn header(side: DrawerSide) -> AnyView {
    Stack::vertical((
        side.title().style(|s| {
            s.font_size(16.0)
                .font_weight(FontWeight::MEDIUM)
                .apply(wrap_text())
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        "Drawer content uses the popover surface with directional borders and rounded corners."
            .style(|s| {
                s.font_size(14.0)
                    .line_height(1.35)
                    .apply(wrap_text())
                    .with_theme(|s, t| s.color(t.muted_foreground()))
            }),
    ))
    .style(move |s| {
        s.flex_col()
            .gap(4.0)
            .padding(16.0)
            .apply_if(matches!(side, DrawerSide::Bottom | DrawerSide::Top), |s| {
                s.items_center()
            })
    })
    .into_any()
}

fn body() -> AnyView {
    Stack::vertical((
        Stack::horizontal((
            icon("calendar-days", 16.0),
            "Schedule maintenance window".style(|s| {
                s.font_size(14.0)
                    .apply(wrap_text())
                    .with_theme(|s, t| s.color(t.foreground()))
            }),
        ))
        .style(|s| s.items_center().gap(8.0).min_width(0.0)),
        Stack::horizontal((
            icon("bell", 16.0),
            "Notify all project members".style(|s| {
                s.font_size(14.0)
                    .apply(wrap_text())
                    .with_theme(|s, t| s.color(t.foreground()))
            }),
        ))
        .style(|s| s.items_center().gap(8.0).min_width(0.0)),
    ))
    .style(|s| s.flex_col().gap(10.0).padding_horiz(16.0))
    .into_any()
}

fn footer() -> AnyView {
    Stack::vertical((Button::new("Submit"), outline_button("Cancel")))
        .style(|s| s.flex_col().gap(8.0).padding(16.0))
        .into_any()
}

fn content(side: DrawerSide) -> AnyView {
    let inner = if matches!(side, DrawerSide::Bottom) {
        Stack::vertical((
            handle(),
            header(side),
            body(),
            Empty::new().style(|s| s.flex_grow(1.0)),
            footer(),
        ))
        .style(|s| s.flex_col())
        .into_any()
    } else {
        Stack::vertical((
            header(side),
            body(),
            Empty::new().style(|s| s.flex_grow(1.0)),
            footer(),
        ))
        .style(|s| s.flex_col())
        .into_any()
    };

    inner
        .style(move |s| {
            let s = match side {
                DrawerSide::Bottom => s.width_full().height(230.0).border_top(1.0),
                DrawerSide::Top => s.width_full().height(220.0).border_bottom(1.0),
                DrawerSide::Left => s.width(280.0).height_full().border_right(1.0),
                DrawerSide::Right => s.width(280.0).height_full().border_left(1.0),
            };
            s.with_theme(|s, t| {
                s.background(t.popover())
                    .color(t.popover_foreground())
                    .border_color(t.border())
            })
            .apply_if(matches!(side, DrawerSide::Bottom), |s| {
                s.border_top_left_radius(12.0)
                    .border_top_right_radius(12.0)
                    .corner_smoothing(0.6)
            })
            .apply_if(matches!(side, DrawerSide::Top), |s| {
                s.border_bottom_left_radius(12.0)
                    .border_bottom_right_radius(12.0)
                    .corner_smoothing(0.6)
            })
            .apply_if(matches!(side, DrawerSide::Left), |s| {
                s.border_top_right_radius(12.0)
                    .border_bottom_right_radius(12.0)
                    .corner_smoothing(0.6)
            })
            .apply_if(matches!(side, DrawerSide::Right), |s| {
                s.border_top_left_radius(12.0)
                    .border_bottom_left_radius(12.0)
                    .corner_smoothing(0.6)
            })
        })
        .into_any()
}

fn preview(side: DrawerSide) -> AnyView {
    let content = match side {
        DrawerSide::Bottom => {
            Stack::vertical((Empty::new().style(|s| s.flex_grow(1.0)), content(side)))
                .style(|s| s.size_full())
                .into_any()
        }
        DrawerSide::Top => {
            Stack::vertical((content(side), Empty::new().style(|s| s.flex_grow(1.0))))
                .style(|s| s.size_full())
                .into_any()
        }
        DrawerSide::Left => {
            Stack::horizontal((content(side), Empty::new().style(|s| s.flex_grow(1.0))))
                .style(|s| s.size_full())
                .into_any()
        }
        DrawerSide::Right => {
            Stack::horizontal((Empty::new().style(|s| s.flex_grow(1.0)), content(side)))
                .style(|s| s.size_full())
                .into_any()
        }
    };

    Stack::vertical((
        side.label().style(|s| {
            s.font_size(14.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        content.clip().style(|s| {
            s.width(560.0)
                .height(340.0)
                .border(1.0)
                .border_radius(8.0)
                .corner_smoothing(0.6)
                .background(Color::from_rgb8(0, 0, 0).with_alpha(0.10))
                .with_theme(|s, t| s.border_color(t.border()))
        }),
    ))
    .style(|s| s.flex_col().gap(10.0))
    .into_any()
}

pub fn drawer_view() -> impl IntoView {
    Stack::vertical((
        "Drawer".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::horizontal((Button::new("Open drawer"), outline_button("Close drawer")))
            .style(|s| s.items_center().gap(8.0)),
        Stack::vertical((
            preview(DrawerSide::Bottom),
            preview(DrawerSide::Top),
            preview(DrawerSide::Left),
            preview(DrawerSide::Right),
        ))
        .style(|s| s.items_start().gap(24.0)),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
