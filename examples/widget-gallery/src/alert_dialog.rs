use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    peniko::Color,
    prelude::*,
    taffy::{FlexWrap, style::Display},
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

#[derive(Clone, Copy)]
enum AlertSize {
    Default,
    Sm,
}

fn icon(name: &'static str, size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

fn outline_button(label: &'static str) -> Button {
    Button::new(label).style(|s| {
        s.border(1.0).with_theme(|s, t| {
            s.background(t.background())
                .border_color(t.input())
                .color(t.foreground())
                .hover(|s| s.background(t.muted()))
        })
    })
}

fn destructive_button(label: &'static str) -> Button {
    Button::new(label).style(|s| {
        s.with_theme(|s, t| {
            s.background(t.def(|t| t.danger().with_alpha(if t.is_dark { 0.2 } else { 0.1 })))
                .color(t.danger())
                .hover(|s| {
                    s.background(
                        t.def(|t| t.danger().with_alpha(if t.is_dark { 0.3 } else { 0.2 })),
                    )
                })
        })
    })
}

fn media(icon_name: &'static str) -> AnyView {
    Stack::vertical((icon(icon_name, 24.0),))
        .style(|s| {
            s.size(40.0, 40.0)
                .items_center()
                .justify_center()
                .border_radius(8.0)
                .corner_smoothing(0.6)
                .with_theme(|s, t| s.background(t.muted()).color(t.foreground()))
        })
        .into_any()
}

fn header(
    icon_name: Option<&'static str>,
    title: &'static str,
    description: &'static str,
    size: AlertSize,
) -> AnyView {
    let title_text = title.style(|s| {
        s.font_size(16.0)
            .font_weight(FontWeight::MEDIUM)
            .with_theme(|s, t| s.color(t.popover_foreground()))
    });
    let description_text = description.style(|s| {
        s.font_size(14.0)
            .line_height(1.4)
            .with_theme(|s, t| s.color(t.muted_foreground()))
    });

    match (icon_name, size) {
        (Some(name), AlertSize::Default) => Stack::horizontal((
            media(name),
            Stack::vertical((title_text, description_text))
                .style(|s| s.flex_col().gap(6.0).flex_grow(1.0)),
        ))
        .style(|s| s.items_start().gap(14.0))
        .into_any(),
        (Some(name), AlertSize::Sm) => Stack::vertical((media(name), title_text, description_text))
            .style(|s| s.items_center().gap(8.0))
            .into_any(),
        (None, _) => Stack::vertical((title_text, description_text))
            .style(|s| s.flex_col().gap(6.0))
            .into_any(),
    }
}

fn footer(content: impl IntoView + 'static, size: AlertSize) -> AnyView {
    content
        .style(move |s| {
            let s = match size {
                AlertSize::Default => s.items_center().justify_end(),
                AlertSize::Sm => s.items_center(),
            };
            s.gap(8.0)
                .margin_horiz(-16.0)
                .margin_bottom(-16.0)
                .padding(16.0)
                .border_top(1.0)
                .border_bottom_left_radius(12.0)
                .border_bottom_right_radius(12.0)
                .corner_smoothing(0.6)
                .with_theme(|s, t| {
                    s.background(t.def(|t| t.muted().with_alpha(0.5)))
                        .border_color(t.border())
                })
        })
        .into_any()
}

fn alert_surface(
    icon_name: Option<&'static str>,
    title: &'static str,
    description: &'static str,
    actions: impl IntoView + 'static,
    size: AlertSize,
) -> AnyView {
    let width = match size {
        AlertSize::Default => 384.0,
        AlertSize::Sm => 320.0,
    };

    Stack::vertical((
        header(icon_name, title, description, size),
        footer(actions, size),
    ))
    .style(move |s| {
        s.width(width)
            .flex_col()
            .gap(16.0)
            .padding(16.0)
            .border(1.0)
            .border_radius(12.0)
            .corner_smoothing(0.6)
            .box_shadow_blur(18.0)
            .box_shadow_color(Color::from_rgb8(0, 0, 0).with_alpha(0.16))
            .with_theme(|s, t| {
                s.background(t.popover())
                    .color(t.popover_foreground())
                    .border_color(t.def(|t| t.foreground.with_alpha(0.1)))
            })
    })
    .into_any()
}

fn delete_alert(open: RwSignal<bool>) -> AnyView {
    alert_surface(
        Some("triangle-alert"),
        "Delete project?",
        "This action cannot be undone. This will permanently delete the project and remove all data from our servers.",
        Stack::horizontal((
            outline_button("Cancel").action(move || open.set(false)),
            destructive_button("Delete").action(move || open.set(false)),
        )),
        AlertSize::Default,
    )
}

fn compact_alert() -> AnyView {
    alert_surface(
        Some("log-out"),
        "Sign out?",
        "You can sign back in at any time.",
        Stack::horizontal((outline_button("Cancel"), Button::new("Continue"))),
        AlertSize::Sm,
    )
}

fn plain_alert() -> AnyView {
    alert_surface(
        None,
        "Discard draft?",
        "Your unsaved message will be removed from this thread.",
        Stack::horizontal((outline_button("Cancel"), destructive_button("Discard"))),
        AlertSize::Default,
    )
}

fn overlay_frame(open: RwSignal<bool>) -> AnyView {
    Stack::vertical((
        Button::new("Open alert dialog").action(move || open.set(true)),
        Stack::vertical((delete_alert(open)
            .style(move |s| s.apply_if(!open.get(), |s| s.display(Display::None))),))
        .style(move |s| {
            s.width(560.0)
                .height(340.0)
                .items_center()
                .justify_center()
                .border(1.0)
                .border_radius(8.0)
                .corner_smoothing(0.6)
                .background(Color::from_rgb8(0, 0, 0).with_alpha(0.10))
                .apply_if(!open.get(), |s| {
                    s.with_theme(|s, t| s.background(t.muted()))
                })
                .with_theme(|s, t| s.border_color(t.border()))
        }),
    ))
    .style(|s| s.flex_col().items_start().gap(12.0))
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

pub fn alert_dialog_view() -> impl IntoView {
    let open = RwSignal::new(true);

    Stack::vertical((
        "Alert Dialog".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::horizontal((
            section("Overlay + Default", overlay_frame(open)),
            section("Small", compact_alert()),
            section("No Media", plain_alert()),
        ))
        .style(|s| s.items_start().gap(24.0).flex_wrap(FlexWrap::Wrap)),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
