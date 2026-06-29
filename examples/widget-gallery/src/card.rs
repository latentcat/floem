use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    prelude::*,
    style::Style,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

#[derive(Clone, Copy)]
enum CardSize {
    Default,
    Sm,
}

impl CardSize {
    fn spacing(self) -> f64 {
        match self {
            Self::Default => 16.0,
            Self::Sm => 12.0,
        }
    }

    fn title_size(self) -> f64 {
        match self {
            Self::Default => 16.0,
            Self::Sm => 14.0,
        }
    }
}

fn card_style(size: CardSize) -> Style {
    let spacing = size.spacing();
    Style::new()
        .width(360.0)
        .flex_col()
        .gap(spacing)
        .padding_vert(spacing)
        .border(1.0)
        .border_radius(12.0)
        .corner_smoothing(0.6)
        .font_size(14.0)
        .with_theme(|s, t| {
            s.background(t.card())
                .color(t.card_foreground())
                .border_color(t.def(|t| t.foreground.with_alpha(0.10)))
        })
}

fn card_header(title: &'static str, description: &'static str, size: CardSize) -> AnyView {
    let spacing = size.spacing();
    Stack::vertical((
        title.style(move |s| {
            s.font_size(size.title_size())
                .font_weight(FontWeight::MEDIUM)
                .line_height(1.35)
                .with_theme(|s, t| s.color(t.card_foreground()))
        }),
        description.style(|s| {
            s.font_size(14.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
    ))
    .style(move |s| s.gap(4.0).padding_horiz(spacing))
    .into_any()
}

fn card_content(size: CardSize) -> AnyView {
    let spacing = size.spacing();
    Stack::vertical((
        metric_row("Revenue", "$12,430"),
        metric_row("Conversion", "8.2%"),
        metric_row("Active users", "2,413"),
    ))
    .style(move |s| s.gap(8.0).padding_horiz(spacing))
    .into_any()
}

fn card_footer(size: CardSize) -> AnyView {
    let spacing = size.spacing();
    Stack::horizontal((
        "Updated now".style(|s| {
            s.font_size(13.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
        Button::new(
            Stack::horizontal((card_icon("arrow-up-right"), "Open"))
                .style(|s| s.items_center().gap(6.0)),
        )
        .style(|s| s.height(28.0).padding_horiz(8.0).font_size(13.0)),
    ))
    .style(move |s| {
        s.items_center()
            .justify_between()
            .padding(spacing)
            .border_top(1.0)
            .with_theme(|s, t| {
                s.background(t.def(|t| t.muted().with_alpha(0.5)))
                    .border_color(t.border())
            })
    })
    .into_any()
}

fn metric_row(label: &'static str, value: &'static str) -> AnyView {
    Stack::horizontal((
        label.style(|s| {
            s.font_size(13.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
        value.style(|s| {
            s.font_size(13.0)
                .font_weight(FontWeight::MEDIUM)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
    ))
    .style(|s| s.items_center().justify_between().gap(16.0))
    .into_any()
}

fn card_icon(name: &'static str) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(|s| s.size(14.0, 14.0).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(|s| s.size(14.0, 14.0)).into_any())
}

fn card(title: &'static str, description: &'static str, size: CardSize) -> AnyView {
    Stack::vertical((
        card_header(title, description, size),
        card_content(size),
        card_footer(size),
    ))
    .clip()
    .style(move |s| s.apply(card_style(size)))
    .into_any()
}

pub fn card_view() -> impl IntoView {
    Stack::horizontal((
        card(
            "Team performance",
            "Weekly activity summary",
            CardSize::Default,
        ),
        card("Compact card", "Small spacing variant", CardSize::Sm),
    ))
    .style(|s| {
        s.gap(16.0)
            .padding(30.0)
            .items_start()
            .flex_wrap(floem::taffy::FlexWrap::Wrap)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
