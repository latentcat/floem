use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    peniko::Color,
    prelude::*,
    taffy::FlexWrap,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

#[derive(Clone, Copy)]
enum SheetSide {
    Right,
    Left,
    Top,
    Bottom,
}

impl SheetSide {
    fn label(self) -> &'static str {
        match self {
            Self::Right => "Right",
            Self::Left => "Left",
            Self::Top => "Top",
            Self::Bottom => "Bottom",
        }
    }

    fn title(self) -> &'static str {
        match self {
            Self::Right => "Right sheet",
            Self::Left => "Left sheet",
            Self::Top => "Top sheet",
            Self::Bottom => "Bottom sheet",
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

fn close_button() -> Button {
    Button::new(icon("x", 14.0)).style(|s| {
        s.size(28.0, 28.0).padding(0.0).with_theme(|s, t| {
            s.background(t.def(|_| Color::TRANSPARENT))
                .border_color(t.def(|_| Color::TRANSPARENT))
                .color(t.muted_foreground())
                .hover(|s| s.background(t.muted()).color(t.foreground()))
        })
    })
}

fn sheet_header(side: SheetSide, show_close: bool) -> AnyView {
    Stack::vertical((
        Stack::horizontal((
            side.title().style(|s| {
                s.font_size(16.0)
                    .font_weight(FontWeight::MEDIUM)
                    .with_theme(|s, t| s.color(t.foreground()))
            }),
            Empty::new().style(|s| s.flex_grow(1.0)),
            if show_close {
                close_button().into_any()
            } else {
                Empty::new().style(|s| s.size(28.0, 28.0)).into_any()
            },
        ))
        .style(|s| s.items_center().gap(8.0)),
        "Make changes and save when you're done.".style(|s| {
            s.font_size(14.0)
                .line_height(1.35)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
    ))
    .style(|s| s.flex_col().gap(6.0).padding(16.0))
    .into_any()
}

fn sheet_body() -> AnyView {
    Stack::vertical((
        Stack::horizontal((
            "Name".style(|s| {
                s.width(72.0)
                    .font_size(14.0)
                    .with_theme(|s, t| s.color(t.foreground()))
            }),
            TextInput::new(RwSignal::new("Jane Doe".to_string())).style(|s| s.flex_grow(1.0)),
        ))
        .style(|s| s.items_center().gap(10.0)),
        Stack::horizontal((
            "Email".style(|s| {
                s.width(72.0)
                    .font_size(14.0)
                    .with_theme(|s, t| s.color(t.foreground()))
            }),
            TextInput::new(RwSignal::new("jane@example.com".to_string()))
                .style(|s| s.flex_grow(1.0)),
        ))
        .style(|s| s.items_center().gap(10.0)),
    ))
    .style(|s| s.flex_col().gap(10.0).padding_horiz(16.0))
    .into_any()
}

fn sheet_footer() -> AnyView {
    Stack::vertical((Button::new("Save changes"), outline_button("Cancel")))
        .style(|s| s.flex_col().gap(8.0).padding(16.0))
        .into_any()
}

fn panel(side: SheetSide, show_close: bool) -> AnyView {
    Stack::vertical((
        sheet_header(side, show_close),
        sheet_body(),
        Empty::new().style(|s| s.flex_grow(1.0)),
        sheet_footer(),
    ))
    .style(move |s| {
        let s = match side {
            SheetSide::Right => s.width(150.0).height_full().border_left(1.0),
            SheetSide::Left => s.width(150.0).height_full().border_right(1.0),
            SheetSide::Top => s.width_full().height(140.0).border_bottom(1.0),
            SheetSide::Bottom => s.width_full().height(140.0).border_top(1.0),
        };
        s.flex_col()
            .box_shadow_blur(14.0)
            .box_shadow_color(Color::from_rgb8(0, 0, 0).with_alpha(0.16))
            .with_theme(|s, t| {
                s.background(t.popover())
                    .color(t.popover_foreground())
                    .border_color(t.border())
            })
    })
    .into_any()
}

fn preview(side: SheetSide, show_close: bool) -> AnyView {
    let content = match side {
        SheetSide::Right => Stack::horizontal((
            Empty::new().style(|s| s.flex_grow(1.0)),
            panel(side, show_close),
        ))
        .style(|s| s.size_full())
        .into_any(),
        SheetSide::Left => Stack::horizontal((
            panel(side, show_close),
            Empty::new().style(|s| s.flex_grow(1.0)),
        ))
        .style(|s| s.size_full())
        .into_any(),
        SheetSide::Top => Stack::vertical((
            panel(side, show_close),
            Empty::new().style(|s| s.flex_grow(1.0)),
        ))
        .style(|s| s.size_full())
        .into_any(),
        SheetSide::Bottom => Stack::vertical((
            Empty::new().style(|s| s.flex_grow(1.0)),
            panel(side, show_close),
        ))
        .style(|s| s.size_full())
        .into_any(),
    };

    Stack::vertical((
        Stack::horizontal((
            side.label().style(|s| {
                s.font_size(14.0)
                    .font_weight(FontWeight::SEMI_BOLD)
                    .with_theme(|s, t| s.color(t.foreground()))
            }),
            Empty::new().style(|s| s.flex_grow(1.0)),
            if show_close {
                "close"
                    .style(|s| {
                        s.font_size(12.0)
                            .with_theme(|s, t| s.color(t.muted_foreground()))
                    })
                    .into_any()
            } else {
                "no close"
                    .style(|s| {
                        s.font_size(12.0)
                            .with_theme(|s, t| s.color(t.muted_foreground()))
                    })
                    .into_any()
            },
        ))
        .style(|s| s.items_center()),
        content.clip().style(|s| {
            s.width(320.0)
                .height(220.0)
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

pub fn sheet_view() -> impl IntoView {
    Stack::vertical((
        "Sheet".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::horizontal((Button::new("Open sheet"), outline_button("Close sheet")))
            .style(|s| s.items_center().gap(8.0)),
        Stack::horizontal((
            section("Right", preview(SheetSide::Right, true)),
            section("Left", preview(SheetSide::Left, true)),
            section("Top", preview(SheetSide::Top, true)),
            section("Bottom", preview(SheetSide::Bottom, false)),
        ))
        .style(|s| s.items_start().gap(24.0).flex_wrap(FlexWrap::Wrap)),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
