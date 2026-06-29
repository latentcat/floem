use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    prelude::*,
    style::{Background, Foreground, Transition},
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

#[derive(Clone, Copy)]
enum DayState {
    Default,
    Outside,
    Today,
    Selected,
    RangeStart,
    RangeMiddle,
    RangeEnd,
    Focused,
    Disabled,
    Hidden,
}

fn icon(name: &'static str, size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

fn weekday(label: &'static str) -> AnyView {
    label
        .style(|s| {
            s.size(32.0, 24.0)
                .items_center()
                .justify_center()
                .font_size(12.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        })
        .into_any()
}

fn day(label: &'static str, state: DayState) -> AnyView {
    label
        .style(move |s| {
            s.size(32.0, 32.0)
                .items_center()
                .justify_center()
                .font_size(13.0)
                .border(1.0)
                .border_radius(8.0)
                .corner_smoothing(0.6)
                .transition(Background, Transition::linear(100.millis()))
                .transition(Foreground, Transition::linear(100.millis()))
                .with_theme(move |s, t| match state {
                    DayState::Default => s
                        .background(t.def(|_| floem::peniko::Color::TRANSPARENT))
                        .border_color(t.def(|_| floem::peniko::Color::TRANSPARENT))
                        .color(t.foreground())
                        .hover(|s| s.background(t.muted())),
                    DayState::Outside => s
                        .background(t.def(|_| floem::peniko::Color::TRANSPARENT))
                        .border_color(t.def(|_| floem::peniko::Color::TRANSPARENT))
                        .color(t.muted_foreground()),
                    DayState::Today => s
                        .background(t.muted())
                        .border_color(t.muted())
                        .color(t.foreground()),
                    DayState::Selected | DayState::RangeStart | DayState::RangeEnd => s
                        .background(t.primary())
                        .border_color(t.primary())
                        .color(t.primary_foreground()),
                    DayState::RangeMiddle => s
                        .background(t.muted())
                        .border_color(t.muted())
                        .color(t.foreground())
                        .border_radius(t.def(|_| 0.0)),
                    DayState::Focused => s
                        .background(t.def(|_| floem::peniko::Color::TRANSPARENT))
                        .border_color(t.ring())
                        .color(t.foreground())
                        .outline(3.0)
                        .outline_color(t.ring_focus()),
                    DayState::Disabled => s
                        .background(t.def(|_| floem::peniko::Color::TRANSPARENT))
                        .border_color(t.def(|_| floem::peniko::Color::TRANSPARENT))
                        .color(t.muted_foreground())
                        .set(floem::style::Opacity, 0.5),
                    DayState::Hidden => s
                        .background(t.def(|_| floem::peniko::Color::TRANSPARENT))
                        .border_color(t.def(|_| floem::peniko::Color::TRANSPARENT))
                        .color(t.def(|_| floem::peniko::Color::TRANSPARENT)),
                })
        })
        .into_any()
}

fn calendar_surface(title: &'static str, caption_dropdown: bool) -> AnyView {
    let weeks = [
        [
            ("29", DayState::Outside),
            ("30", DayState::Outside),
            ("1", DayState::Default),
            ("2", DayState::Default),
            ("3", DayState::Default),
            ("4", DayState::Disabled),
            ("5", DayState::Default),
        ],
        [
            ("6", DayState::Default),
            ("7", DayState::Today),
            ("8", DayState::Default),
            ("9", DayState::RangeStart),
            ("10", DayState::RangeMiddle),
            ("11", DayState::RangeEnd),
            ("12", DayState::Default),
        ],
        [
            ("13", DayState::Default),
            ("14", DayState::Selected),
            ("15", DayState::Default),
            ("16", DayState::Default),
            ("17", DayState::Default),
            ("18", DayState::Default),
            ("19", DayState::Default),
        ],
        [
            ("20", DayState::Default),
            ("21", DayState::Default),
            ("22", DayState::Default),
            ("23", DayState::Default),
            ("24", DayState::Default),
            ("25", DayState::Default),
            ("26", DayState::Default),
        ],
        [
            ("27", DayState::Default),
            ("28", DayState::Default),
            ("29", DayState::Default),
            ("30", DayState::Default),
            ("31", DayState::Default),
            ("1", DayState::Outside),
            ("2", DayState::Outside),
        ],
        [
            ("", DayState::Hidden),
            ("", DayState::Hidden),
            ("", DayState::Hidden),
            ("", DayState::Hidden),
            ("", DayState::Hidden),
            ("", DayState::Hidden),
            ("", DayState::Hidden),
        ],
    ];

    Stack::vertical((
        Stack::horizontal((
            Button::new(icon("chevron-left", 16.0)).style(|s| s.size(32.0, 32.0).padding(0.0)),
            Stack::horizontal((
                title.style(|s| {
                    s.font_size(14.0)
                        .font_weight(FontWeight::MEDIUM)
                        .with_theme(|s, t| s.color(t.foreground()))
                }),
                if caption_dropdown {
                    icon("chevron-down", 14.0)
                } else {
                    Empty::new().into_any()
                },
            ))
            .style(|s| {
                s.items_center()
                    .gap(4.0)
                    .padding_horiz(8.0)
                    .border_radius(8.0)
                    .with_theme(|s, t| s.hover(|s| s.background(t.muted())).color(t.foreground()))
            }),
            Button::new(icon("chevron-right", 16.0)).style(|s| s.size(32.0, 32.0).padding(0.0)),
        ))
        .style(|s| s.items_center().justify_between().height(32.0)),
        Stack::horizontal_from_iter(["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"].map(weekday))
            .style(|s| s.gap(2.0)),
        Stack::vertical_from_iter(weeks.map(|week| {
            Stack::horizontal_from_iter(week.map(|(label, state)| day(label, state)))
                .style(|s| s.gap(2.0))
        }))
        .style(|s| s.flex_col().gap(2.0)),
    ))
    .style(|s| {
        s.width(252.0)
            .flex_col()
            .gap(8.0)
            .padding(8.0)
            .border_radius(12.0)
            .corner_smoothing(0.6)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
    .into_any()
}

pub fn calendar_view() -> impl IntoView {
    Stack::vertical((
        "Calendar".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::horizontal((
            calendar_surface("July 2026", true),
            Stack::vertical((
                "States".style(|s| {
                    s.font_size(14.0)
                        .font_weight(FontWeight::SEMI_BOLD)
                        .with_theme(|s, t| s.color(t.foreground()))
                }),
                Stack::horizontal((
                    day("7", DayState::Today),
                    day("14", DayState::Selected),
                    day("9", DayState::RangeStart),
                    day("10", DayState::RangeMiddle),
                    day("11", DayState::RangeEnd),
                    day("8", DayState::Focused),
                    day("4", DayState::Disabled),
                ))
                .style(|s| s.gap(8.0).items_center()),
            ))
            .style(|s| {
                s.flex_col()
                    .gap(10.0)
                    .padding(16.0)
                    .border(1.0)
                    .border_radius(12.0)
                    .corner_smoothing(0.6)
                    .with_theme(|s, t| {
                        s.background(t.card())
                            .border_color(t.border())
                            .color(t.card_foreground())
                    })
            }),
            Stack::vertical((
                "Multi Month".style(|s| {
                    s.font_size(14.0)
                        .font_weight(FontWeight::SEMI_BOLD)
                        .with_theme(|s, t| s.color(t.foreground()))
                }),
                Stack::horizontal((
                    calendar_surface("July 2026", false),
                    calendar_surface("August 2026", false),
                ))
                .style(|s| s.items_start().gap(12.0)),
            ))
            .style(|s| {
                s.flex_col()
                    .gap(10.0)
                    .padding(16.0)
                    .border(1.0)
                    .border_radius(12.0)
                    .corner_smoothing(0.6)
                    .with_theme(|s, t| {
                        s.background(t.card())
                            .border_color(t.border())
                            .color(t.card_foreground())
                    })
            }),
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
