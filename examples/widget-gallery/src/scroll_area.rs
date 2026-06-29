use floem::{
    AnyView, IntoView,
    peniko::Color,
    prelude::*,
    style::{Background, CustomStylable, Foreground, Transition},
    taffy::FlexWrap,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Decorators, scroll},
};

fn row(index: usize) -> impl IntoView {
    Stack::vertical((
        format!("v{index}.0.0").style(|s| {
            s.font_size(14.0)
                .font_weight(FontWeight::MEDIUM)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        "Release notes and package metadata".style(|s| {
            s.font_size(12.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
    ))
    .style(|s| {
        s.flex_col()
            .gap(2.0)
            .padding_vert(10.0)
            .border_bottom(1.0)
            .with_theme(|s, t| s.border_color(t.border()))
    })
}

fn token_tile(index: usize) -> impl IntoView {
    Stack::vertical((
        format!("Token {index}").style(|s| {
            s.font_size(13.0)
                .font_weight(FontWeight::MEDIUM)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        "128px".style(|s| {
            s.font_size(12.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
    ))
    .style(|s| {
        s.width(128.0)
            .height(84.0)
            .flex_col()
            .justify_center()
            .gap(4.0)
            .padding(12.0)
            .border(1.0)
            .border_radius(8.0)
            .corner_smoothing(0.6)
            .with_theme(|s, t| s.background(t.card()).border_color(t.border()))
    })
}

fn shadcn_scroll(content: impl IntoView + 'static, width: f64, height: f64) -> impl IntoView {
    content
        .scroll()
        .custom_style(|s| {
            s.shrink_to_fit()
                .track_background(Color::TRANSPARENT)
                .track_border_radius(0.0)
                .track_rounded(false)
                .handle_border_radius(100.pct())
                .handle_rounded(false)
                .vertical_track_inset(1.0)
                .horizontal_track_inset(1.0)
        })
        .style(move |s| {
            s.size(width, height)
                .scrollbar_width(10.0)
                .border(1.0)
                .border_radius(8.0)
                .corner_smoothing(0.6)
                .keyboard_navigable()
                .transition(Background, Transition::linear(100.millis()))
                .transition(Foreground, Transition::linear(100.millis()))
                .with_theme(|s, t| {
                    s.background(t.background())
                        .border_color(t.border())
                        .class(scroll::Handle, |s| s.background(t.border()))
                        .class(scroll::Track, |s| {
                            s.background(t.def(|_| Color::TRANSPARENT))
                        })
                        .focus_visible(|s| {
                            s.outline(3.0)
                                .outline_color(t.ring_focus())
                                .border_color(t.ring())
                        })
                })
        })
}

fn both_axes_content() -> impl IntoView {
    Stack::vertical_from_iter((1..=7).map(|row| {
        Stack::horizontal_from_iter((1..=8).map(move |column| token_tile((row - 1) * 8 + column)))
            .style(|s| s.items_center().gap(10.0))
    }))
    .style(|s| s.flex_col().gap(10.0).padding(14.0).min_size(1120.0, 660.0))
}

fn anatomy_surface() -> AnyView {
    Stack::horizontal((
        Stack::vertical((
            "Viewport".style(|s| {
                s.font_size(13.0)
                    .font_weight(FontWeight::MEDIUM)
                    .with_theme(|s, t| s.color(t.foreground()))
            }),
            "Rounded inherit, focus ring, hidden overflow.".style(|s| {
                s.font_size(12.0)
                    .with_theme(|s, t| s.color(t.muted_foreground()))
            }),
            Empty::new().style(|s| s.flex_grow(1.0)),
            Stack::horizontal((
                Empty::new().style(|s| {
                    s.height(10.0)
                        .flex_grow(1.0)
                        .border_top(1.0)
                        .with_theme(|s, t| s.border_color(t.def(|_| Color::TRANSPARENT)))
                }),
                Empty::new().style(|s| {
                    s.size(10.0, 10.0)
                        .with_theme(|s, t| s.background(t.border()))
                }),
            ))
            .style(|s| s.items_end().gap(0.0)),
        ))
        .style(|s| {
            s.width(250.0)
                .height(150.0)
                .padding(14.0)
                .border(1.0)
                .border_radius(8.0)
                .corner_smoothing(0.6)
                .with_theme(|s, t| s.background(t.background()).border_color(t.border()))
        }),
        Stack::vertical((
            Empty::new().style(|s| {
                s.width(10.0)
                    .height(118.0)
                    .border_left(1.0)
                    .with_theme(|s, t| s.border_color(t.def(|_| Color::TRANSPARENT)))
            }),
            Empty::new().style(|s| {
                s.width(10.0)
                    .height(32.0)
                    .border_radius(100.0)
                    .with_theme(|s, t| s.background(t.border()))
            }),
        ))
        .style(|s| s.width(10.0).height(150.0).items_center()),
    ))
    .style(|s| s.items_start().gap(1.0))
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

pub fn scroll_area_view() -> impl IntoView {
    let vertical_content =
        Stack::vertical_from_iter((1..=18).map(row)).style(|s| s.flex_col().padding_horiz(14.0));

    let horizontal_content = Stack::horizontal_from_iter((1..=12).map(token_tile))
        .style(|s| s.items_center().gap(10.0).padding(14.0).min_width(1700.0));

    Stack::vertical((
        "Scroll Area".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::horizontal((
            section("Vertical", shadcn_scroll(vertical_content, 360.0, 240.0)),
            section(
                "Horizontal",
                shadcn_scroll(horizontal_content, 420.0, 140.0),
            ),
            section(
                "Both axes",
                shadcn_scroll(both_axes_content(), 420.0, 220.0),
            ),
            section("Anatomy", anatomy_surface()),
        ))
        .style(|s| s.gap(24.0).flex_wrap(FlexWrap::Wrap)),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
