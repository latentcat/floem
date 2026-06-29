use floem::{
    AnyView, IntoView, prelude::*, text::FontWeight, theme::StyleThemeExt, views::Decorators,
};

#[derive(Clone, Copy)]
enum Series {
    Desktop,
    Mobile,
    Other,
}

fn series_color(series: Series, s: floem::style::Style) -> floem::style::Style {
    s.with_theme(move |s, t| match series {
        Series::Desktop => s.background(t.primary()),
        Series::Mobile => s.background(t.success()),
        Series::Other => s.background(t.warning()),
    })
}

fn legend_item(series: Series, label: &'static str) -> AnyView {
    Stack::horizontal((
        Empty::new().style(move |s| {
            series_color(series, s.size(8.0, 8.0).border_radius(2.0).flex_shrink(0.0))
        }),
        label.style(|s| {
            s.font_size(12.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
    ))
    .style(|s| s.items_center().gap(6.0))
    .into_any()
}

fn bar(label: &'static str, desktop: f64, mobile: f64) -> AnyView {
    Stack::vertical((
        Stack::horizontal((
            Empty::new().style(move |s| {
                series_color(
                    Series::Desktop,
                    s.width(18.0)
                        .height(desktop)
                        .border_radius(4.0)
                        .corner_smoothing(0.6),
                )
            }),
            Empty::new().style(move |s| {
                series_color(
                    Series::Mobile,
                    s.width(18.0)
                        .height(mobile)
                        .border_radius(4.0)
                        .corner_smoothing(0.6),
                )
            }),
        ))
        .style(|s| s.height(160.0).items_end().gap(4.0)),
        label.style(|s| {
            s.font_size(12.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
    ))
    .style(|s| s.flex_col().items_center().gap(8.0))
    .into_any()
}

fn tooltip() -> AnyView {
    Stack::vertical((
        "March".style(|s| {
            s.font_size(12.0)
                .font_weight(FontWeight::MEDIUM)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::vertical((
            Stack::horizontal((
                legend_item(Series::Desktop, "Desktop"),
                "1,250".style(|s| {
                    s.font_size(12.0)
                        .font_weight(FontWeight::MEDIUM)
                        .with_theme(|s, t| s.color(t.foreground()))
                }),
            ))
            .style(|s| s.justify_between().gap(24.0)),
            Stack::horizontal((
                legend_item(Series::Mobile, "Mobile"),
                "980".style(|s| {
                    s.font_size(12.0)
                        .font_weight(FontWeight::MEDIUM)
                        .with_theme(|s, t| s.color(t.foreground()))
                }),
            ))
            .style(|s| s.justify_between().gap(24.0)),
        ))
        .style(|s| s.flex_col().gap(6.0)),
    ))
    .style(|s| {
        s.width(148.0)
            .flex_col()
            .gap(8.0)
            .padding_horiz(10.0)
            .padding_vert(8.0)
            .border(1.0)
            .border_radius(8.0)
            .corner_smoothing(0.6)
            .box_shadow_blur(18.0)
            .box_shadow_color(floem::peniko::Color::from_rgb8(0, 0, 0).with_alpha(0.16))
            .with_theme(|s, t| {
                s.background(t.background())
                    .border_color(t.def(|t| t.border().with_alpha(0.5)))
                    .color(t.foreground())
            })
    })
    .into_any()
}

fn chart_surface() -> AnyView {
    Stack::vertical((
        Stack::horizontal((
            bar("Jan", 88.0, 52.0),
            bar("Feb", 112.0, 76.0),
            bar("Mar", 148.0, 118.0),
            bar("Apr", 96.0, 136.0),
            bar("May", 128.0, 86.0),
        ))
        .style(|s| s.items_end().gap(22.0).height(198.0)),
        Stack::horizontal((
            legend_item(Series::Desktop, "Desktop"),
            legend_item(Series::Mobile, "Mobile"),
            legend_item(Series::Other, "Other"),
        ))
        .style(|s| s.items_center().justify_center().gap(16.0)),
    ))
    .style(|s| {
        s.width(520.0)
            .flex_col()
            .items_center()
            .gap(14.0)
            .padding(18.0)
            .border(1.0)
            .border_radius(12.0)
            .corner_smoothing(0.6)
            .with_theme(|s, t| {
                s.background(t.card())
                    .border_color(t.border())
                    .color(t.card_foreground())
            })
    })
    .into_any()
}

pub fn chart_view() -> impl IntoView {
    Stack::vertical((
        "Chart".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::horizontal((chart_surface(), tooltip())).style(|s| {
            s.items_start()
                .gap(18.0)
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
