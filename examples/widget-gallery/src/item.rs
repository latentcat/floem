use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    peniko::Color,
    prelude::*,
    style::{Background, Style, Transition},
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

#[derive(Clone, Copy)]
enum ItemVariant {
    Default,
    Outline,
    Muted,
}

#[derive(Clone, Copy)]
enum ItemSize {
    Default,
    Sm,
    Xs,
}

impl ItemSize {
    fn padding_h(self) -> f64 {
        match self {
            Self::Default | Self::Sm => 12.0,
            Self::Xs => 10.0,
        }
    }

    fn padding_v(self) -> f64 {
        match self {
            Self::Default | Self::Sm => 10.0,
            Self::Xs => 8.0,
        }
    }

    fn gap(self) -> f64 {
        match self {
            Self::Default | Self::Sm => 10.0,
            Self::Xs => 8.0,
        }
    }
}

fn item_icon(name: &'static str) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(|s| s.size(16.0, 16.0).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(|s| s.size(16.0, 16.0)).into_any())
}

fn item_style(variant: ItemVariant, size: ItemSize) -> Style {
    let style = Style::new()
        .width(520.0)
        .padding_horiz(size.padding_h())
        .padding_vert(size.padding_v())
        .gap(size.gap())
        .items_center()
        .border(1.0)
        .border_radius(8.0)
        .corner_smoothing(0.6)
        .font_size(14.0)
        .transition(Background, Transition::linear(100.millis()))
        .focus_visible(|s| {
            s.with_theme(|s, t| {
                s.border_color(t.ring())
                    .outline(3.0)
                    .outline_color(t.ring_focus())
            })
        })
        .hover(|s| s.with_theme(|s, t| s.background(t.muted())));

    match variant {
        ItemVariant::Default => style
            .background(Color::TRANSPARENT)
            .with_theme(|s, t| s.border_color(t.def(|_| Color::TRANSPARENT))),
        ItemVariant::Outline => style.with_theme(|s, t| s.border_color(t.border())),
        ItemVariant::Muted => style.with_theme(|s, t| {
            s.background(t.def(|t| t.muted().with_alpha(0.5)))
                .border_color(t.def(|_| Color::TRANSPARENT))
        }),
    }
}

fn item(
    icon_name: &'static str,
    title: &'static str,
    description: &'static str,
    variant: ItemVariant,
    size: ItemSize,
) -> AnyView {
    Stack::horizontal((
        item_icon(icon_name),
        Stack::vertical((
            title.style(|s| {
                s.font_size(14.0)
                    .font_weight(FontWeight::MEDIUM)
                    .with_theme(|s, t| s.color(t.foreground()))
            }),
            description.style(move |s| {
                s.font_size(match size {
                    ItemSize::Xs => 12.0,
                    _ => 14.0,
                })
                .with_theme(|s, t| s.color(t.muted_foreground()))
            }),
        ))
        .style(|s| s.flex_col().gap(4.0).flex_grow(1.0)),
        Button::new("Open").style(|s| s.height(28.0).padding_horiz(8.0).font_size(13.0)),
    ))
    .style(move |s| s.apply(item_style(variant, size)))
    .into_any()
}

pub fn item_view() -> impl IntoView {
    Stack::vertical((
        item(
            "folder",
            "Default item",
            "Use items for compact rows with title, description, and actions.",
            ItemVariant::Default,
            ItemSize::Default,
        ),
        item(
            "file-text",
            "Outline item",
            "A bordered variant for grouped settings or lists.",
            ItemVariant::Outline,
            ItemSize::Sm,
        ),
        item(
            "bell",
            "Muted item",
            "A low-emphasis variant with muted background.",
            ItemVariant::Muted,
            ItemSize::Xs,
        ),
    ))
    .style(|s| {
        s.padding(30.0)
            .gap(16.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
