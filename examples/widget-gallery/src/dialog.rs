use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    peniko::Color,
    prelude::*,
    taffy::FlexWrap,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

use crate::{
    portal::modal_portal,
    shadcn_style::{text_column, wrap_text},
};

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

fn close_button(open: RwSignal<bool>) -> Button {
    Button::new(icon("x", 16.0))
        .action(move || open.set(false))
        .style(|s| {
            s.size(28.0, 28.0).padding(0.0).with_theme(|s, t| {
                s.background(t.def(|_| Color::TRANSPARENT))
                    .border_color(t.def(|_| Color::TRANSPARENT))
                    .color(t.muted_foreground())
                    .hover(|s| s.background(t.muted()).color(t.foreground()))
            })
        })
}

fn header(title: &'static str, description: &'static str) -> AnyView {
    Stack::vertical((
        title.style(|s| {
            s.font_size(16.0)
                .line_height(1.0)
                .font_weight(FontWeight::MEDIUM)
                .apply(wrap_text())
                .with_theme(|s, t| s.color(t.popover_foreground()))
        }),
        description.style(|s| {
            s.font_size(14.0)
                .line_height(1.4)
                .apply(wrap_text())
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
    ))
    .style(|s| s.apply(text_column()).gap(8.0))
    .into_any()
}

fn footer(content: impl IntoView + 'static) -> AnyView {
    content
        .style(|s| {
            s.items_center()
                .justify_end()
                .gap(8.0)
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

fn field(label: &'static str, value: &'static str) -> AnyView {
    Stack::horizontal((
        label.style(|s| {
            s.width(76.0)
                .font_size(14.0)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        TextInput::new(RwSignal::new(value.to_owned())).style(|s| s.flex_grow(1.0)),
    ))
    .style(|s| s.items_center().gap(10.0).min_width(0.0))
    .into_any()
}

fn dialog_surface(
    title: &'static str,
    description: &'static str,
    body: impl IntoView + 'static,
    actions: impl IntoView + 'static,
    open: Option<RwSignal<bool>>,
    show_close: bool,
) -> AnyView {
    Stack::vertical((
        Stack::horizontal((
            header(title, description),
            Empty::new().style(|s| s.flex_grow(1.0)),
            if show_close {
                close_button(open.unwrap_or_else(|| RwSignal::new(true))).into_any()
            } else {
                Empty::new().style(|s| s.size(28.0, 28.0)).into_any()
            },
        ))
        .style(|s| s.items_start().gap(12.0).min_width(0.0)),
        body,
        footer(actions),
    ))
    .style(|s| {
        s.width(384.0)
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

fn profile_dialog(open: RwSignal<bool>) -> AnyView {
    dialog_surface(
        "Edit profile",
        "Make changes to your profile here. Click save when you're done.",
        Stack::vertical((field("Name", "Jane Doe"), field("Username", "@janedoe")))
            .style(|s| s.flex_col().gap(10.0)),
        Stack::horizontal((
            outline_button("Close").action(move || open.set(false)),
            Button::new("Save changes").action(move || open.set(false)),
        )),
        Some(open),
        true,
    )
}

fn compact_dialog() -> AnyView {
    dialog_surface(
        "Invite team",
        "Send an invitation to a collaborator.",
        Stack::vertical((field("Email", "jane@example.com"),)).style(|s| s.flex_col().gap(10.0)),
        Stack::horizontal((outline_button("Cancel"), Button::new("Send invite"))),
        None,
        true,
    )
}

fn no_close_dialog() -> AnyView {
    dialog_surface(
        "Session expired",
        "Sign in again to continue working in this workspace.",
        Stack::horizontal((
            icon("shield-alert", 18.0),
            "Your draft is saved locally.".style(|s| {
                s.font_size(14.0)
                    .apply(wrap_text())
                    .with_theme(|s, t| s.color(t.muted_foreground()))
            }),
        ))
        .style(|s| s.items_center().gap(8.0).min_width(0.0)),
        Stack::horizontal((Button::new("Sign in"),)),
        None,
        false,
    )
}

fn overlay_frame(open: RwSignal<bool>) -> AnyView {
    Stack::vertical((
        Button::new("Open dialog").action(move || open.set(true)),
        modal_portal(open, move || profile_dialog(open)),
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

pub fn dialog_view() -> impl IntoView {
    let open = RwSignal::new(false);

    Stack::vertical((
        "Dialog".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::horizontal((
            section("Overlay + Content", overlay_frame(open)),
            section("Compact", compact_dialog()),
            section("No Close Button", no_close_dialog()),
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
