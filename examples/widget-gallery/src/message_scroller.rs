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

fn item(index: usize) -> impl IntoView {
    Stack::vertical((
        format!("Message {index}").style(|s| {
            s.font_size(13.0)
                .font_weight(FontWeight::MEDIUM)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        "A virtualized row keeps enough intrinsic size for smooth scrolling.".style(|s| {
            s.font_size(12.0)
                .line_height(1.35)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
    ))
    .style(|s| {
        s.flex_col()
            .gap(3.0)
            .padding(12.0)
            .border(1.0)
            .border_radius(10.0)
            .corner_smoothing(0.6)
            .with_theme(|s, t| {
                s.background(t.card())
                    .border_color(t.border())
                    .color(t.card_foreground())
            })
    })
}

pub fn message_scroller_view() -> impl IntoView {
    let content = Stack::vertical_from_iter((1..=18).map(item)).style(|s| {
        s.flex_col()
            .gap(12.0)
            .padding(14.0)
            .min_height(640.0)
            .justify_end()
    });

    Stack::vertical((
        "Message Scroller".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::vertical((
            content.scroll().style(|s| {
                s.size(460.0, 304.0)
                    .scrollbar_width(10.0)
                    .with_theme(|s, t| s.background(t.background()))
            }),
            Button::new(icon("arrow-down", 16.0))
                .style(|s| {
                    s.size(32.0, 32.0)
                        .padding(0.0)
                        .border_radius(999.0)
                        .box_shadow_blur(10.0)
                        .box_shadow_color(floem::peniko::Color::from_rgb8(0, 0, 0).with_alpha(0.12))
                        .with_theme(|s, t| {
                            s.background(t.background())
                                .color(t.foreground())
                                .border_color(t.border())
                                .hover(|s| s.background(t.muted()))
                        })
                })
                .into_any(),
        ))
        .clip()
        .style(|s| {
            s.size(460.0, 360.0)
                .flex_col()
                .items_center()
                .gap(10.0)
                .border(1.0)
                .border_radius(12.0)
                .corner_smoothing(0.6)
                .padding(12.0)
                .with_theme(|s, t| {
                    s.background(t.background())
                        .border_color(t.border())
                        .color(t.foreground())
                })
        }),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
