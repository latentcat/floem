use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    prelude::*,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

fn icon(name: &'static str, size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

fn slide(index: usize, width: f64, height: f64) -> AnyView {
    Stack::vertical((
        format!("Slide {index}").style(|s| {
            s.font_size(18.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.card_foreground()))
        }),
        "Carousel item".style(|s| {
            s.font_size(13.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
    ))
    .style(move |s| {
        s.size(width, height)
            .flex_col()
            .items_center()
            .justify_center()
            .gap(6.0)
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

fn nav_button(name: &'static str, disabled: bool) -> Button {
    Button::new(icon(name, 16.0)).style(move |s| {
        s.size(32.0, 32.0)
            .padding(0.0)
            .border_radius(999.0)
            .apply_if(disabled, |s| s.set_disabled(true))
    })
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

pub fn carousel_view() -> impl IntoView {
    let horizontal = Stack::horizontal((
        nav_button("chevron-left", true),
        Stack::horizontal((
            slide(1, 220.0, 140.0),
            slide(2, 220.0, 140.0),
            slide(3, 220.0, 140.0),
        ))
        .clip()
        .style(|s| s.width(456.0).gap(16.0)),
        nav_button("chevron-right", false),
    ))
    .style(|s| s.items_center().gap(12.0));

    let vertical = Stack::horizontal((
        Stack::vertical((
            nav_button("chevron-left", false).style(|s| s.rotate(90.0.deg())),
            Stack::vertical((slide(1, 220.0, 108.0), slide(2, 220.0, 108.0)))
                .clip()
                .style(|s| s.height(232.0).gap(16.0)),
            nav_button("chevron-right", false).style(|s| s.rotate(90.0.deg())),
        ))
        .style(|s| s.flex_col().items_center().gap(12.0)),
        Stack::vertical((
            "Keyboard".style(|s| {
                s.font_size(13.0)
                    .font_weight(FontWeight::MEDIUM)
                    .with_theme(|s, t| s.color(t.foreground()))
            }),
            "Arrow keys move between slides; controls use outline icon buttons.".style(|s| {
                s.width(220.0)
                    .font_size(13.0)
                    .line_height(1.4)
                    .with_theme(|s, t| s.color(t.muted_foreground()))
            }),
        ))
        .style(|s| s.flex_col().gap(4.0)),
    ))
    .style(|s| s.items_center().gap(22.0));

    Stack::vertical((
        "Carousel".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        section("Horizontal", horizontal),
        section("Vertical", vertical),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
