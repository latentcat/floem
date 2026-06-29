use floem::{
    AnyView, IntoView,
    prelude::*,
    taffy::FlexDirection,
    text::FontWeight,
    theme::StyleThemeExt,
    unit::Pct,
    views::{Decorators, resizable::Resizable},
};

fn panel(title: &'static str, detail: &'static str) -> AnyView {
    Stack::vertical((
        title.style(|s| {
            s.font_size(14.0)
                .font_weight(FontWeight::MEDIUM)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        detail.style(|s| {
            s.font_size(13.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
    ))
    .style(|s| {
        s.flex_col()
            .gap(3.0)
            .items_center()
            .justify_center()
            .height_full()
            .min_size(0.0, 0.0)
            .with_theme(|s, t| s.background(t.muted()).color(t.foreground()))
    })
    .into_any()
}

fn handle_preview(horizontal: bool) -> AnyView {
    let handle = Empty::new().style(move |s| {
        s.apply_if(horizontal, |s| s.height(24.0).width(1.0))
            .apply_if(!horizontal, |s| s.width(24.0).height(1.0))
            .border_radius(999.0)
            .with_theme(|s, t| s.background(t.border()))
    });

    let grip = Empty::new().style(move |s| {
        s.apply_if(horizontal, |s| s.height(24.0).width(4.0))
            .apply_if(!horizontal, |s| s.width(24.0).height(4.0))
            .border_radius(999.0)
            .with_theme(|s, t| s.background(t.border()))
    });

    Stack::horizontal((handle, grip))
        .style(|s| s.items_center().gap(12.0))
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

pub fn resizable_view() -> impl IntoView {
    let horizontal = Resizable::new((
        panel("Sidebar", "24%"),
        panel("Content", "52%"),
        panel("Preview", "fill"),
    ))
    .custom_sizes_pct(|| vec![(0, Pct(24.0)), (1, Pct(52.0))])
    .clip()
    .style(|s| {
        s.width(660.0)
            .height(220.0)
            .border(1.0)
            .border_radius(12.0)
            .corner_smoothing(0.6)
            .with_theme(|s, t| s.border_color(t.border()).background(t.background()))
    });

    let vertical = Resizable::new((panel("Header", "32%"), panel("Activity", "fill")))
        .custom_sizes_pct(|| vec![(0, Pct(32.0))])
        .clip()
        .style(|s| {
            s.width(420.0)
                .height(260.0)
                .flex_direction(FlexDirection::Column)
                .border(1.0)
                .border_radius(12.0)
                .corner_smoothing(0.6)
                .with_theme(|s, t| s.border_color(t.border()).background(t.background()))
        });

    Stack::vertical((
        "Resizable".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        section("Panel group", horizontal),
        Stack::horizontal((
            section("Vertical", vertical),
            section(
                "Handle styles",
                Stack::vertical((handle_preview(true), handle_preview(false))).style(|s| {
                    s.flex_col()
                        .gap(18.0)
                        .padding(18.0)
                        .border(1.0)
                        .border_radius(12.0)
                        .corner_smoothing(0.6)
                        .with_theme(|s, t| {
                            s.background(t.card())
                                .border_color(t.border())
                                .color(t.card_foreground())
                        })
                }),
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
