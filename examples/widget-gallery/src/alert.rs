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

use crate::form::{form, form_item};

#[derive(Clone, Copy)]
enum AlertVariant {
    Default,
    Destructive,
}

fn alert_icon(name: &'static str) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| icon.style(|s| s.size(16.0, 16.0).flex_shrink(0.0)).into_any())
        .unwrap_or_else(|| Empty::new().style(|s| s.size(16.0, 16.0)).into_any())
}

fn alert_style(variant: AlertVariant) -> Style {
    let style = Style::new()
        .width(520.0)
        .padding_horiz(10.0)
        .padding_vert(8.0)
        .gap(8.0)
        .border(1.0)
        .border_radius(8.0)
        .corner_smoothing(0.6)
        .font_size(14.0)
        .items_start()
        .with_theme(|s, t| {
            s.background(t.card())
                .color(t.card_foreground())
                .border_color(t.border())
        });

    match variant {
        AlertVariant::Default => style,
        AlertVariant::Destructive => style.with_theme(|s, t| {
            s.color(t.danger())
                .class(LabelClass, |s| s.color(t.danger()))
        }),
    }
}

fn alert_description_style(variant: AlertVariant) -> Style {
    match variant {
        AlertVariant::Default => Style::new().with_theme(|s, t| s.color(t.muted_foreground())),
        AlertVariant::Destructive => Style::new().with_theme(|s, t| {
            s.color(t.def(|t| t.danger().with_alpha(if t.is_dark { 0.9 } else { 0.88 })))
        }),
    }
}

fn alert(
    icon_name: &'static str,
    title: &'static str,
    description: &'static str,
    variant: AlertVariant,
    action: Option<&'static str>,
) -> AnyView {
    let content = Stack::vertical((
        title.style(|s| s.font_weight(FontWeight::MEDIUM)),
        description.style(move |s| s.font_size(14.0).apply(alert_description_style(variant))),
    ))
    .style(|s| s.flex_col().gap(2.0).flex_grow(1.0));

    let action_view = action
        .map(|label| {
            Button::new(label)
                .style(|s| s.height(28.0).padding_horiz(8.0).font_size(13.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().into_any());

    Stack::horizontal((alert_icon(icon_name), content, action_view))
        .style(move |s| {
            s.apply(alert_style(variant))
                .transition(Background, Transition::linear(100.millis()))
        })
        .into_any()
}

pub fn alert_view() -> impl IntoView {
    form((
        form_item(
            "Default:",
            alert(
                "info",
                "Heads up",
                "You can add components to your app using the registry.",
                AlertVariant::Default,
                None,
            ),
        ),
        form_item(
            "Destructive:",
            alert(
                "triangle-alert",
                "Unable to save changes",
                "Your session expired. Please sign in again before continuing.",
                AlertVariant::Destructive,
                Some("Retry"),
            ),
        ),
    ))
    .style(|s| {
        s.with_theme(|s, t| s.background(t.background()).color(t.foreground()))
            .border_color(Color::TRANSPARENT)
    })
}
