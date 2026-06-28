use floem::{
    prelude::*, style::ObjectFit, taffy::style::Overflow, text::FontWeight, unit::UnitExt,
    views::Empty,
};

pub fn background_blur_view() -> impl IntoView {
    let sunflower = include_bytes!("./../assets/sunflower.jpg");

    let background = img(move || sunflower.to_vec()).style(|s| {
        s.absolute()
            .inset(0)
            .size_full()
            .object_fit(ObjectFit::Cover)
    });

    let accent = Empty::new().style(|s| {
        s.size(64, 64)
            .border_radius(100.pct())
            .corner_smoothing(0.6)
            .background(Color::from_rgba8(255, 255, 255, 88))
            .border(1)
            .border_color(Color::from_rgba8(255, 255, 255, 96))
    });

    let card = Stack::vertical((
        Stack::horizontal((
            Stack::vertical((
                "Morning Light".style(|s| {
                    s.font_size(14)
                        .font_weight(FontWeight::MEDIUM)
                        .color(Color::from_rgba8(255, 255, 255, 210))
                }),
                "Sunflower Ridge".style(|s| {
                    s.font_size(34)
                        .font_weight(FontWeight::BOLD)
                        .color(Color::from_rgba8(255, 255, 255, 245))
                }),
            ))
            .style(|s| s.flex_col().gap(4)),
            accent,
        ))
        .style(|s| s.width_full().items_center().justify_between()),
        Stack::horizontal((
            metric("24deg", "Air"),
            metric("62%", "Humidity"),
            metric("8 km/h", "Wind"),
        ))
        .style(|s| s.gap(12).padding_top(16)),
        "Golden hour starts at 18:42".style(|s| {
            s.font_size(15)
                .color(Color::from_rgba8(255, 255, 255, 205))
                .padding_top(18)
        }),
    ))
    .style(|s| {
        s.width(430)
            .max_width(86.pct())
            .padding(28)
            .gap(10)
            .border_radius(30)
            .corner_smoothing(0.6)
            .backdrop_blur(80)
            .background(Color::from_rgba8(255, 255, 255, 42))
            .border(1)
            .border_color(Color::from_rgba8(255, 255, 255, 88))
            .box_shadow_blur(36)
            .box_shadow_color(Color::from_rgba8(0, 0, 0, 76))
            .box_shadow_v_offset(18)
    });

    Stack::new((background, card)).style(|s| {
        s.size_full()
            .min_width(640)
            .min_height(520)
            .items_center()
            .justify_center()
            .overflow_x(Overflow::Clip)
            .overflow_y(Overflow::Clip)
            .background(Color::from_rgb8(30, 38, 30))
    })
}

fn metric(value: &'static str, label: &'static str) -> impl IntoView {
    Stack::vertical((
        value.style(|s| {
            s.font_size(18)
                .font_weight(FontWeight::SEMI_BOLD)
                .color(Color::from_rgba8(255, 255, 255, 238))
        }),
        label.style(|s| s.font_size(12).color(Color::from_rgba8(255, 255, 255, 168))),
    ))
    .style(|s| {
        s.flex_col()
            .gap(3)
            .width(120)
            .padding_vert(12)
            .padding_horiz(14)
            .border_radius(18)
            .corner_smoothing(0.6)
            .background(Color::from_rgba8(255, 255, 255, 32))
            .border(1)
            .border_color(Color::from_rgba8(255, 255, 255, 50))
    })
}
