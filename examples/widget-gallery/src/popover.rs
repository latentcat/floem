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

use crate::portal::{PortalPosition, anchored_portal};

fn icon(name: &'static str, size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

fn popover_surface(content: impl IntoView + 'static, width: f64) -> AnyView {
    content
        .style(move |s| {
            s.width(width)
                .flex_col()
                .gap(10.0)
                .padding(10.0)
                .border(1.0)
                .border_radius(8.0)
                .corner_smoothing(0.6)
                .box_shadow_blur(12.0)
                .box_shadow_color(Color::from_rgb8(0, 0, 0).with_alpha(0.16))
                .with_theme(|s, t| {
                    s.background(t.popover())
                        .color(t.popover_foreground())
                        .border_color(t.def(|t| t.foreground.with_alpha(0.10)))
                })
        })
        .into_any()
}

fn popover_header(title: &'static str, description: &'static str) -> AnyView {
    Stack::vertical((
        title.style(|s| {
            s.font_size(14.0)
                .font_weight(FontWeight::MEDIUM)
                .with_theme(|s, t| s.color(t.popover_foreground()))
        }),
        description.style(|s| {
            s.font_size(13.0)
                .line_height(1.35)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
    ))
    .style(|s| s.flex_col().gap(2.0))
    .into_any()
}

fn field(label: &'static str, value: &'static str) -> impl IntoView {
    Stack::horizontal((
        label.style(|s| {
            s.width(76.0)
                .font_size(13.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
        TextInput::new(RwSignal::new(value.to_string())).style(|s| s.height(28.0).flex_grow(1.0)),
    ))
    .style(|s| s.items_center().gap(8.0))
}

fn action_item(icon_name: &'static str, label: &'static str, shortcut: &'static str) -> AnyView {
    Stack::horizontal((
        icon(icon_name, 16.0),
        label.style(|s| s.font_size(14.0)),
        Empty::new().style(|s| s.flex_grow(1.0)),
        shortcut.style(|s| {
            s.font_size(12.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
    ))
    .style(|s| {
        s.min_height(28.0)
            .items_center()
            .gap(8.0)
            .padding_horiz(6.0)
            .padding_vert(4.0)
            .border_radius(6.0)
            .with_theme(|s, t| {
                s.color(t.popover_foreground())
                    .hover(|s| s.background(t.accent()).color(t.accent_foreground()))
            })
    })
    .into_any()
}

fn dimensions_surface() -> AnyView {
    popover_surface(
        Stack::vertical((
            popover_header("Dimensions", "Set the dimensions for the layer."),
            field("Width", "100%"),
            field("Height", "25px"),
        )),
        288.0,
    )
}

fn actions_surface() -> AnyView {
    popover_surface(
        Stack::vertical((
            popover_header("Share document", "Invite collaborators or copy a link."),
            action_item("copy", "Copy link", "Cmd+C"),
            action_item("mail-plus", "Invite by email", "I"),
            action_item("settings", "Permissions", "P"),
        )),
        256.0,
    )
}

fn status_surface() -> AnyView {
    popover_surface(
        Stack::vertical((
            Stack::horizontal((
                icon("circle-check", 18.0).style(|s| s.with_theme(|s, t| s.color(t.primary()))),
                popover_header("Synced", "All local edits are saved."),
            ))
            .style(|s| s.items_start().gap(8.0)),
            Button::new("View activity").style(|s| s.height(28.0).width_full()),
        )),
        240.0,
    )
}

fn side_preview(label: &'static str, content: impl IntoView + 'static) -> AnyView {
    Stack::vertical((
        label.style(|s| {
            s.font_size(12.0)
                .font_weight(FontWeight::MEDIUM)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
        content,
    ))
    .style(|s| s.flex_col().gap(6.0))
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

pub fn popover_view() -> impl IntoView {
    let open = RwSignal::new(false);

    Stack::vertical((
        "Popover".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        section(
            "Controlled",
            anchored_portal(
                Button::new(
                    Stack::horizontal((icon("panel-top-open", 16.0), "Open popover"))
                        .style(|s| s.items_center().gap(6.0)),
                )
                .action(move || open.update(|value| *value = !*value)),
                open,
                PortalPosition::bottom_start(8.0),
                dimensions_surface,
            ),
        ),
        section(
            "Content",
            Stack::horizontal((dimensions_surface(), actions_surface(), status_surface())).style(
                |s| {
                    s.items_start()
                        .gap(24.0)
                        .flex_wrap(FlexWrap::Wrap)
                        .width_full()
                },
            ),
        ),
        section(
            "Placement Surfaces",
            Stack::horizontal((
                side_preview("Top", status_surface()),
                side_preview("Right", actions_surface()),
                side_preview("Bottom", dimensions_surface()),
                side_preview("Left", status_surface()),
            ))
            .style(|s| s.items_start().gap(24.0).flex_wrap(FlexWrap::Wrap)),
        ),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
