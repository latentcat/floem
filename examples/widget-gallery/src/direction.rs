use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    prelude::*,
    taffy::FlexDirection,
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

fn field(label: &'static str, value: &'static str, rtl: bool) -> AnyView {
    Stack::horizontal((
        label.style(|s| {
            s.font_size(13.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
        value.style(|s| {
            s.font_size(14.0)
                .font_weight(FontWeight::MEDIUM)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
    ))
    .style(move |s| {
        s.items_center()
            .justify_between()
            .gap(16.0)
            .padding_horiz(10.0)
            .height(36.0)
            .border(1.0)
            .border_radius(8.0)
            .corner_smoothing(0.6)
            .apply_if(rtl, |s| s.flex_direction(FlexDirection::RowReverse))
            .with_theme(|s, t| {
                s.background(t.input())
                    .border_color(t.border())
                    .color(t.foreground())
            })
    })
    .into_any()
}

fn sample_card(title: &'static str, description: &'static str, rtl: bool) -> AnyView {
    let direction = if rtl { "RTL" } else { "LTR" };
    let icon_name = if rtl { "align-right" } else { "align-left" };

    Stack::vertical((
        Stack::horizontal((
            icon(icon_name, 16.0),
            Stack::vertical((
                title.style(|s| {
                    s.font_size(15.0)
                        .font_weight(FontWeight::MEDIUM)
                        .with_theme(|s, t| s.color(t.card_foreground()))
                }),
                description.style(|s| {
                    s.font_size(13.0)
                        .with_theme(|s, t| s.color(t.muted_foreground()))
                }),
            ))
            .style(|s| s.flex_col().gap(2.0)),
            Empty::new().style(|s| s.flex_grow(1.0)),
            direction.style(|s| {
                s.font_size(12.0)
                    .font_weight(FontWeight::MEDIUM)
                    .padding_horiz(8.0)
                    .height(24.0)
                    .items_center()
                    .border(1.0)
                    .border_radius(6.0)
                    .with_theme(|s, t| {
                        s.background(t.secondary())
                            .color(t.secondary_foreground())
                            .border_color(t.border())
                    })
            }),
        ))
        .style(move |s| {
            s.items_center()
                .gap(10.0)
                .apply_if(rtl, |s| s.flex_direction(FlexDirection::RowReverse))
        }),
        Stack::horizontal((
            "Dashboard".style(|s| s.font_size(13.0)),
            icon(if rtl { "chevron-left" } else { "chevron-right" }, 14.0),
            "Settings".style(|s| s.font_size(13.0)),
            icon(if rtl { "chevron-left" } else { "chevron-right" }, 14.0),
            "Members".style(|s| {
                s.font_size(13.0)
                    .font_weight(FontWeight::MEDIUM)
                    .with_theme(|s, t| s.color(t.foreground()))
            }),
        ))
        .style(move |s| {
            s.items_center()
                .gap(6.0)
                .apply_if(rtl, |s| s.flex_direction(FlexDirection::RowReverse))
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
        Stack::vertical((field("Name", "Ava Chen", rtl), field("Role", "Admin", rtl)))
            .style(|s| s.flex_col().gap(8.0)),
    ))
    .style(move |s| {
        s.width(392.0)
            .flex_col()
            .gap(14.0)
            .padding(16.0)
            .border(1.0)
            .border_radius(12.0)
            .corner_smoothing(0.6)
            .apply_if(rtl, |s| s.flex_direction(FlexDirection::Column))
            .with_theme(|s, t| {
                s.background(t.card())
                    .color(t.card_foreground())
                    .border_color(t.def(|t| t.foreground.with_alpha(0.1)))
            })
    })
    .into_any()
}

pub fn direction_view() -> impl IntoView {
    Stack::vertical((
        "Direction".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::horizontal((
            sample_card("Left-to-right", "Default interface flow", false),
            sample_card("Right-to-left", "Mirrored layout flow", true),
        ))
        .style(|s| {
            s.items_start()
                .gap(20.0)
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
