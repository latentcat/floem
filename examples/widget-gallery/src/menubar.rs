use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    peniko::Color,
    prelude::*,
    style::{Background, Foreground, Opacity, Transition},
    taffy::FlexWrap,
    text::FontWeight,
    theme::StyleThemeExt,
    views::Decorators,
};

use crate::portal::{PortalPosition, anchored_portal};

#[derive(Clone, Copy)]
enum MenuVariant {
    Default,
    Destructive,
}

fn icon(name: &'static str, size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

fn trigger(label: &'static str, active: bool) -> AnyView {
    label
        .style(move |s| {
            s.min_height(26.0)
                .items_center()
                .padding_horiz(6.0)
                .padding_vert(2.0)
                .border_radius(4.0)
                .font_size(14.0)
                .font_weight(FontWeight::MEDIUM)
                .selectable(false)
                .transition(Background, Transition::linear(100.millis()))
                .with_theme(move |s, t| {
                    s.color(t.foreground())
                        .hover(|s| s.background(t.muted()))
                        .apply_if(active, |s| s.background(t.muted()))
                })
        })
        .into_any()
}

fn shortcut(text: &'static str) -> AnyView {
    text.style(|s| {
        s.font_size(12.0)
            .with_theme(|s, t| s.color(t.muted_foreground()))
    })
    .into_any()
}

fn label(text: &'static str, inset: bool) -> AnyView {
    text.style(move |s| {
        s.padding_left(if inset { 28.0 } else { 6.0 })
            .padding_right(6.0)
            .padding_vert(4.0)
            .font_size(14.0)
            .font_weight(FontWeight::MEDIUM)
            .with_theme(|s, t| s.color(t.popover_foreground()))
    })
    .into_any()
}

fn separator() -> AnyView {
    Empty::new()
        .style(|s| {
            s.height(1.0)
                .margin_vert(4.0)
                .margin_horiz(-4.0)
                .with_theme(|s, t| s.background(t.border()))
        })
        .into_any()
}

#[allow(clippy::too_many_arguments)]
fn menu_item(
    leading: Option<&'static str>,
    item_label: &'static str,
    trailing: Option<&'static str>,
    indicator: Option<&'static str>,
    variant: MenuVariant,
    disabled: bool,
    inset: bool,
    open: bool,
) -> AnyView {
    Stack::horizontal((
        leading
            .map(|name| icon(name, 16.0))
            .unwrap_or_else(|| Empty::new().into_any()),
        item_label.style(|s| s.font_size(14.0)),
        Empty::new().style(|s| s.flex_grow(1.0)),
        trailing
            .map(shortcut)
            .unwrap_or_else(|| Empty::new().into_any()),
        indicator
            .map(|name| icon(name, 16.0))
            .unwrap_or_else(|| Empty::new().into_any()),
    ))
    .style(move |s| {
        s.min_height(28.0)
            .items_center()
            .gap(6.0)
            .padding_left(if inset { 28.0 } else { 6.0 })
            .padding_right(6.0)
            .padding_vert(4.0)
            .border_radius(6.0)
            .selectable(false)
            .transition(Background, Transition::linear(100.millis()))
            .transition(Foreground, Transition::linear(100.millis()))
            .with_theme(move |s, t| match variant {
                MenuVariant::Default => s
                    .color(t.popover_foreground())
                    .apply_if(open, |s| {
                        s.background(t.accent()).color(t.accent_foreground())
                    })
                    .hover(|s| s.background(t.accent()).color(t.accent_foreground())),
                MenuVariant::Destructive => s.color(t.danger()).hover(|s| {
                    s.background(
                        t.def(|t| t.danger().with_alpha(if t.is_dark { 0.2 } else { 0.1 })),
                    )
                    .color(t.danger())
                }),
            })
            .disabled(|s| s.set(Opacity, 0.5).unset_cursor())
            .apply_if(disabled, |s| s.set_disabled(true))
    })
    .into_any()
}

fn check_item(item_label: &'static str, checked: bool, disabled: bool) -> AnyView {
    menu_item(
        None,
        item_label,
        None,
        checked.then_some("check"),
        MenuVariant::Default,
        disabled,
        true,
        false,
    )
}

fn radio_item(item_label: &'static str, checked: bool, disabled: bool) -> AnyView {
    menu_item(
        None,
        item_label,
        None,
        checked.then_some("check"),
        MenuVariant::Default,
        disabled,
        true,
        false,
    )
}

fn sub_trigger(item_label: &'static str, open: bool, inset: bool) -> AnyView {
    menu_item(
        None,
        item_label,
        None,
        Some("chevron-right"),
        MenuVariant::Default,
        false,
        inset,
        open,
    )
}

fn menubar_root(active: &'static str) -> AnyView {
    Stack::horizontal((
        trigger("File", active == "File"),
        trigger("Edit", active == "Edit"),
        trigger("View", active == "View"),
        trigger("Profiles", active == "Profiles"),
    ))
    .style(|s| {
        s.height(32.0)
            .items_center()
            .gap(2.0)
            .padding(3.0)
            .border(1.0)
            .border_radius(8.0)
            .corner_smoothing(0.6)
            .with_theme(|s, t| {
                s.background(t.background())
                    .border_color(t.border())
                    .color(t.foreground())
            })
    })
    .into_any()
}

fn menu_surface(content: impl IntoView + 'static, width: f64, sub_content: bool) -> AnyView {
    content
        .style(move |s| {
            s.width(width)
                .min_width(if sub_content { 128.0 } else { 144.0 })
                .flex_col()
                .padding(4.0)
                .border(1.0)
                .border_radius(8.0)
                .corner_smoothing(0.6)
                .box_shadow_blur(if sub_content { 16.0 } else { 12.0 })
                .box_shadow_color(Color::from_rgb8(0, 0, 0).with_alpha(0.16))
                .with_theme(|s, t| {
                    s.background(t.popover())
                        .color(t.popover_foreground())
                        .border_color(t.def(|t| t.foreground.with_alpha(0.10)))
                })
        })
        .into_any()
}

fn file_menu() -> AnyView {
    menu_surface(
        Stack::vertical((
            menu_item(
                Some("file-plus"),
                "New Tab",
                Some("Cmd T"),
                None,
                MenuVariant::Default,
                false,
                false,
                false,
            ),
            menu_item(
                Some("copy-plus"),
                "New Window",
                Some("Cmd N"),
                None,
                MenuVariant::Default,
                false,
                false,
                false,
            ),
            menu_item(
                None,
                "New Private Window",
                Some("Shift Cmd N"),
                None,
                MenuVariant::Default,
                false,
                true,
                false,
            ),
            separator(),
            check_item("Show Bookmarks Bar", true, false),
            check_item("Show Full URLs", false, false),
            separator(),
            sub_trigger("Share", true, false),
            menu_item(
                Some("printer"),
                "Print...",
                Some("Cmd P"),
                None,
                MenuVariant::Default,
                true,
                false,
                false,
            ),
            separator(),
            menu_item(
                Some("trash-2"),
                "Close Workspace",
                None,
                None,
                MenuVariant::Destructive,
                false,
                false,
                false,
            ),
        )),
        260.0,
        false,
    )
}

fn view_menu() -> AnyView {
    menu_surface(
        Stack::vertical((
            label("Density", true),
            radio_item("Compact", false, false),
            radio_item("Default", true, false),
            radio_item("Comfortable", false, false),
            separator(),
            label("Panels", true),
            check_item("Show Sidebar", true, false),
            check_item("Show Preview", false, false),
            check_item("Show Timeline", false, true),
        )),
        220.0,
        false,
    )
}

fn menubar_portal_preview(
    active: &'static str,
    content: impl Fn() -> AnyView + 'static,
) -> AnyView {
    let open = RwSignal::new(false);
    anchored_portal(
        menubar_root(active).on_event_stop(listener::Click, move |_, _| {
            open.update(|value| *value = !*value)
        }),
        open,
        PortalPosition::bottom_start(6.0),
        content,
    )
}

fn menubar_submenu_content(sub_open: RwSignal<bool>) -> AnyView {
    menu_surface(
        Stack::vertical((
            menu_item(
                Some("file-plus"),
                "New Tab",
                Some("Cmd T"),
                None,
                MenuVariant::Default,
                false,
                false,
                false,
            ),
            menu_item(
                Some("copy-plus"),
                "New Window",
                Some("Cmd N"),
                None,
                MenuVariant::Default,
                false,
                false,
                false,
            ),
            separator(),
            anchored_portal(
                sub_trigger("Share", sub_open.get(), false)
                    .on_event_stop(listener::PointerEnter, move |_, _| sub_open.set(true))
                    .on_event_stop(listener::Click, move |_, _| {
                        sub_open.update(|value| *value = !*value)
                    }),
                sub_open,
                PortalPosition::right_start(4.0),
                || {
                    menu_surface(
                        Stack::vertical((
                            menu_item(
                                None,
                                "Email link",
                                None,
                                None,
                                MenuVariant::Default,
                                false,
                                false,
                                false,
                            ),
                            menu_item(
                                None,
                                "Copy link",
                                None,
                                None,
                                MenuVariant::Default,
                                false,
                                false,
                                false,
                            ),
                            separator(),
                            menu_item(
                                None,
                                "Invite team",
                                None,
                                None,
                                MenuVariant::Default,
                                true,
                                false,
                                false,
                            ),
                        )),
                        150.0,
                        true,
                    )
                },
            ),
            separator(),
            menu_item(
                Some("trash-2"),
                "Close Workspace",
                None,
                None,
                MenuVariant::Destructive,
                false,
                false,
                false,
            ),
        )),
        260.0,
        false,
    )
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

pub fn menubar_view() -> impl IntoView {
    Stack::vertical((
        "Menubar".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::horizontal((
            section("Root + Content", menubar_portal_preview("File", file_menu)),
            section(
                "Radio / Checkbox",
                menubar_portal_preview("View", view_menu),
            ),
            section("Submenu", {
                let sub_open = RwSignal::new(false);
                menubar_portal_preview("File", move || menubar_submenu_content(sub_open))
            }),
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
