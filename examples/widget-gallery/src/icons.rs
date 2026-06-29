use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    prelude::*,
    style::Style,
    taffy::FlexWrap,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

fn icon_view(library: IconLibrary, name: &'static str, size: f64) -> AnyView {
    icon_library::icon(library, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| {
            Empty::new()
                .style(move |s| {
                    s.size(size, size)
                        .border(1.0)
                        .with_theme(|s, t| s.border_color(t.border()))
                })
                .into_any()
        })
}

fn icon_tile(library: IconLibrary, name: &'static str) -> AnyView {
    Stack::vertical((
        icon_view(library, name, 22.0),
        name.to_owned().style(|s| {
            s.font_size(12.0)
                .width(96.0)
                .text_ellipsis()
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
    ))
    .style(|s| {
        s.width(112.0)
            .height(72.0)
            .items_center()
            .justify_center()
            .gap(8.0)
            .border(1.0)
            .border_radius(8.0)
            .corner_smoothing(0.6)
            .with_theme(|s, t| {
                s.background(t.card())
                    .border_color(t.border())
                    .color(t.card_foreground())
            })
    })
    .into_any()
}

fn icon_button_style() -> Style {
    Style::new()
        .size(32.0, 32.0)
        .padding(0.0)
        .border_radius(10.0)
        .corner_smoothing(0.6)
}

fn icon_text_button(library: IconLibrary, name: &'static str, label: &'static str) -> Button {
    Button::new(
        Stack::horizontal((icon_view(library, name, 16.0), label))
            .style(|s| s.items_center().gap(6.0)),
    )
    .action(move || println!("{library} {name} clicked"))
}

fn outline_button_style() -> Style {
    Style::new()
        .with_theme(|s, t| {
            s.background(t.background())
                .border_color(t.border())
                .color(t.foreground())
                .hover(|s| s.background(t.muted()).color(t.foreground()))
        })
        .border(1.0)
}

fn ghost_button_style() -> Style {
    Style::new()
        .background(palette::css::TRANSPARENT)
        .border_color(palette::css::TRANSPARENT)
        .with_theme(|s, t| {
            s.color(t.foreground())
                .hover(|s| s.background(t.muted()).color(t.foreground()))
        })
}

fn library_count(library: IconLibrary) -> AnyView {
    Stack::vertical((
        library.as_str().to_owned().style(|s| {
            s.font_size(12.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
        icon_library::count(library).to_string().style(|s| {
            s.font_size(20.0)
                .font_weight(floem::text::FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
    ))
    .style(|s| {
        s.width(132.0)
            .gap(2.0)
            .padding(12.0)
            .border(1.0)
            .border_radius(8.0)
            .corner_smoothing(0.6)
            .with_theme(|s, t| s.background(t.card()).border_color(t.border()))
    })
    .into_any()
}

fn section(title: &'static str, content: impl IntoView + 'static) -> AnyView {
    Stack::vertical((
        title.style(|s| {
            s.font_size(14.0)
                .font_weight(floem::text::FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        content,
    ))
    .style(|s| s.flex_col().gap(10.0))
    .into_any()
}

pub fn icons_view() -> impl IntoView {
    let lucide_grid = Stack::horizontal((
        icon_tile(IconLibrary::Lucide, "search"),
        icon_tile(IconLibrary::Lucide, "settings"),
        icon_tile(IconLibrary::Lucide, "download"),
        icon_tile(IconLibrary::Lucide, "trash-2"),
        icon_tile(IconLibrary::Lucide, "plus"),
    ))
    .style(|s| s.flex_wrap(FlexWrap::Wrap).gap(10.0));

    let hugeicons_grid = Stack::horizontal((
        icon_tile(IconLibrary::Hugeicons, "searching"),
        icon_tile(IconLibrary::Hugeicons, "settings-01"),
        icon_tile(IconLibrary::Hugeicons, "download-01"),
        icon_tile(IconLibrary::Hugeicons, "delete-02"),
        icon_tile(IconLibrary::Hugeicons, "add-01"),
    ))
    .style(|s| s.flex_wrap(FlexWrap::Wrap).gap(10.0));

    let icon_buttons = Stack::horizontal((
        Button::new(icon_view(IconLibrary::Lucide, "search", 16.0))
            .action(|| println!("search clicked"))
            .style(|s| s.apply(icon_button_style())),
        Button::new(icon_view(IconLibrary::Lucide, "settings", 16.0))
            .action(|| println!("settings clicked"))
            .style(|s| s.apply(icon_button_style()).apply(outline_button_style())),
        Button::new(icon_view(IconLibrary::Hugeicons, "download-01", 16.0))
            .action(|| println!("download clicked"))
            .style(|s| s.apply(icon_button_style()).apply(ghost_button_style())),
        Button::new(icon_view(IconLibrary::Hugeicons, "delete-02", 16.0))
            .action(|| println!("delete clicked"))
            .style(|s| {
                s.apply(icon_button_style()).with_theme(|s, t| {
                    s.background(
                        t.def(|t| t.danger().with_alpha(if t.is_dark { 0.2 } else { 0.1 })),
                    )
                    .color(t.danger())
                    .hover(|s| {
                        s.background(
                            t.def(|t| t.danger().with_alpha(if t.is_dark { 0.3 } else { 0.2 })),
                        )
                    })
                })
            }),
    ))
    .style(|s| s.items_center().gap(8.0));

    let icon_text_buttons = Stack::horizontal((
        icon_text_button(IconLibrary::Lucide, "download", "Download"),
        icon_text_button(IconLibrary::Lucide, "settings", "Settings")
            .style(|s| s.apply(outline_button_style())),
        icon_text_button(IconLibrary::Hugeicons, "add-01", "Create"),
        icon_text_button(IconLibrary::Hugeicons, "delete-02", "Delete").style(|s| {
            s.with_theme(|s, t| {
                s.background(t.def(|t| t.danger().with_alpha(if t.is_dark { 0.2 } else { 0.1 })))
                    .color(t.danger())
                    .hover(|s| {
                        s.background(
                            t.def(|t| t.danger().with_alpha(if t.is_dark { 0.3 } else { 0.2 })),
                        )
                    })
            })
        }),
    ))
    .style(|s| s.items_center().gap(8.0).flex_wrap(FlexWrap::Wrap));

    Stack::vertical((
        "Icons".style(|s| {
            s.font_size(20.0)
                .font_weight(floem::text::FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::horizontal((
            library_count(IconLibrary::Lucide),
            library_count(IconLibrary::Hugeicons),
        ))
        .style(|s| s.gap(10.0).items_center().flex_wrap(FlexWrap::Wrap)),
        section("Lucide", lucide_grid),
        section("Hugeicons", hugeicons_grid),
        section("Icon buttons", icon_buttons),
        section("Icon text buttons", icon_text_buttons),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
