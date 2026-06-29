use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    prelude::*,
    theme::StyleThemeExt,
};

use crate::form::{form, form_item};

#[derive(Clone, Copy)]
enum MarkerVariant {
    Default,
    Separator,
    Border,
}

fn marker_icon(name: &'static str) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(|s| s.size(16.0, 16.0).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(|s| s.size(16.0, 16.0)).into_any())
}

fn line() -> Empty {
    Empty::new().style(|s| {
        s.height(1.0)
            .flex_grow(1.0)
            .with_theme(|s, t| s.background(t.border()))
    })
}

fn marker(label: &'static str, icon_name: &'static str, variant: MarkerVariant) -> AnyView {
    let content = Stack::horizontal((marker_icon(icon_name), label))
        .style(|s| s.items_center().gap(8.0).font_size(14.0));

    match variant {
        MarkerVariant::Default => content
            .style(|s| {
                s.min_height(16.0)
                    .with_theme(|s, t| s.color(t.muted_foreground()))
            })
            .into_any(),
        MarkerVariant::Separator => Stack::horizontal((line(), content, line()))
            .style(|s| {
                s.width(420.0)
                    .items_center()
                    .gap(8.0)
                    .with_theme(|s, t| s.color(t.muted_foreground()))
            })
            .into_any(),
        MarkerVariant::Border => content
            .style(|s| {
                s.width(420.0)
                    .padding_bottom(8.0)
                    .border_bottom(1.0)
                    .with_theme(|s, t| s.color(t.muted_foreground()).border_color(t.border()))
            })
            .into_any(),
    }
}

pub fn marker_view() -> impl IntoView {
    form((
        form_item(
            "Default:",
            marker("Last synced 2 minutes ago", "clock", MarkerVariant::Default),
        ),
        form_item(
            "Separator:",
            marker("Today", "calendar", MarkerVariant::Separator),
        ),
        form_item(
            "Border:",
            marker("Activity", "activity", MarkerVariant::Border),
        ),
    ))
    .style(|s| s.with_theme(|s, t| s.background(t.background()).color(t.foreground())))
}
