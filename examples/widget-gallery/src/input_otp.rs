use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    prelude::*,
    style::{Background, Foreground, Opacity, Transition},
    text::FontWeight,
    theme::StyleThemeExt,
    views::Decorators,
};

fn icon(name: &'static str, size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

fn caret() -> AnyView {
    Empty::new()
        .style(|s| {
            s.width(1.0)
                .height(16.0)
                .with_theme(|s, t| s.background(t.foreground()))
        })
        .animation(|a| {
            a.duration(1.seconds())
                .keyframe(0, |f| f.style(|s| s.opacity(1.0)))
                .keyframe(50, |f| f.style(|s| s.opacity(0.0)))
                .keyframe(100, |f| f.style(|s| s.opacity(1.0)))
                .repeat(true)
        })
        .into_any()
}

#[allow(clippy::too_many_arguments)]
fn slot(
    value: &'static str,
    active: bool,
    invalid: bool,
    disabled: bool,
    first: bool,
    last: bool,
    fake_caret: bool,
) -> AnyView {
    Stack::horizontal((
        value.style(|s| s.font_size(14.0)),
        if fake_caret {
            caret()
        } else {
            Empty::new().into_any()
        },
    ))
    .style(move |s| {
        s.size(32.0, 32.0)
            .items_center()
            .justify_center()
            .border_top(1.0)
            .border_right(1.0)
            .border_bottom(1.0)
            .transition(Background, Transition::linear(100.millis()))
            .transition(Foreground, Transition::linear(100.millis()))
            .with_theme(move |s, t| {
                let border = if invalid {
                    t.danger()
                } else if active {
                    t.ring()
                } else {
                    t.input()
                };
                s.background(t.input_background())
                    .border_color(border)
                    .color(t.foreground())
                    .apply_if(active, |s| {
                        s.outline(3.0)
                            .outline_color(t.ring_focus())
                            .border_color(t.ring())
                    })
                    .apply_if(invalid, |s| {
                        s.outline(3.0).outline_color(
                            t.def(|t| t.danger().with_alpha(if t.is_dark { 0.4 } else { 0.2 })),
                        )
                    })
                    .disabled(|s| s.set(Opacity, 0.5).unset_cursor())
            })
            .apply_if(first, |s| {
                s.border_left(1.0)
                    .border_top_left_radius(8.0)
                    .border_bottom_left_radius(8.0)
                    .corner_smoothing(0.6)
            })
            .apply_if(last, |s| {
                s.border_top_right_radius(8.0)
                    .border_bottom_right_radius(8.0)
                    .corner_smoothing(0.6)
            })
            .apply_if(disabled, |s| s.set_disabled(true))
    })
    .into_any()
}

fn group(
    values: [&'static str; 3],
    active: Option<usize>,
    invalid: bool,
    disabled: bool,
    caret_index: Option<usize>,
) -> AnyView {
    Stack::horizontal((
        slot(
            values[0],
            active == Some(0),
            invalid,
            disabled,
            true,
            false,
            caret_index == Some(0),
        ),
        slot(
            values[1],
            active == Some(1),
            invalid,
            disabled,
            false,
            false,
            caret_index == Some(1),
        )
        .style(|s| s.margin_left(-1.0)),
        slot(
            values[2],
            active == Some(2),
            invalid,
            disabled,
            false,
            true,
            caret_index == Some(2),
        )
        .style(|s| s.margin_left(-1.0)),
    ))
    .style(move |s| {
        s.items_center()
            .border_radius(8.0)
            .corner_smoothing(0.6)
            .apply_if(invalid, |s| {
                s.with_theme(|s, t| {
                    s.outline(3.0).outline_color(
                        t.def(|t| t.danger().with_alpha(if t.is_dark { 0.4 } else { 0.2 })),
                    )
                })
            })
            .apply_if(disabled, |s| s.set(Opacity, 0.5).set_disabled(true))
    })
    .into_any()
}

fn separator() -> AnyView {
    Stack::horizontal((icon("minus", 16.0),))
        .style(|s| {
            s.height(32.0)
                .items_center()
                .with_theme(|s, t| s.color(t.muted_foreground()))
        })
        .into_any()
}

fn otp_row(title: &'static str, content: impl IntoView + 'static) -> AnyView {
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

pub fn input_otp_view() -> impl IntoView {
    Stack::vertical((
        "Input OTP".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::horizontal((
            otp_row(
                "Default",
                Stack::horizontal((
                    group(["1", "2", "3"], Some(1), false, false, None),
                    separator(),
                    group(["4", "", ""], Some(1), false, false, Some(1)),
                ))
                .style(|s| s.items_center().gap(8.0)),
            ),
            otp_row(
                "Filled",
                Stack::horizontal((group(["8", "4", "2"], None, false, false, None),))
                    .style(|s| s.items_center()),
            ),
            otp_row(
                "Fake caret",
                Stack::horizontal((group(["", "", ""], Some(0), false, false, Some(0)),))
                    .style(|s| s.items_center()),
            ),
            otp_row(
                "Invalid",
                Stack::horizontal((group(["8", "4", ""], Some(2), true, false, Some(2)),))
                    .style(|s| s.items_center()),
            ),
            otp_row(
                "Disabled",
                Stack::horizontal((group(["0", "0", "0"], None, false, true, None),))
                    .style(|s| s.items_center()),
            ),
        ))
        .style(|s| {
            s.items_start()
                .gap(24.0)
                .flex_wrap(floem::taffy::FlexWrap::Wrap)
        }),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
