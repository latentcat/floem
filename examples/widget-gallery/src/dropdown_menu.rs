use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    peniko::Color,
    prelude::*,
    style::{Background, Foreground, Opacity, Transition},
    taffy::FlexWrap,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
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

fn menu_label(text: &'static str, inset: bool) -> AnyView {
    text.style(move |s| {
        s.padding_left(if inset { 28.0 } else { 6.0 })
            .padding_right(6.0)
            .padding_vert(4.0)
            .font_size(12.0)
            .font_weight(FontWeight::MEDIUM)
            .with_theme(|s, t| s.color(t.muted_foreground()))
    })
    .into_any()
}

fn menu_separator() -> AnyView {
    Empty::new()
        .style(|s| {
            s.height(1.0)
                .margin_vert(4.0)
                .margin_horiz(-4.0)
                .with_theme(|s, t| s.background(t.border()))
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

#[allow(clippy::too_many_arguments)]
fn menu_item(
    leading: Option<&'static str>,
    label: &'static str,
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
        label.style(|s| s.font_size(14.0)),
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
    .action(move || println!("{label}"))
    .into_any()
}

fn checkbox_item(label: &'static str, checked: bool, disabled: bool, inset: bool) -> AnyView {
    menu_item(
        None,
        label,
        None,
        checked.then_some("check"),
        MenuVariant::Default,
        disabled,
        inset,
        false,
    )
}

fn radio_item(label: &'static str, checked: bool, disabled: bool, inset: bool) -> AnyView {
    menu_item(
        None,
        label,
        None,
        checked.then_some("check"),
        MenuVariant::Default,
        disabled,
        inset,
        false,
    )
}

fn sub_trigger(label: &'static str, open: bool, inset: bool) -> AnyView {
    menu_item(
        None,
        label,
        None,
        Some("chevron-right"),
        MenuVariant::Default,
        false,
        inset,
        open,
    )
}

fn menu_surface(content: impl IntoView + 'static, width: f64) -> AnyView {
    content
        .style(move |s| {
            s.width(width)
                .min_width(128.0)
                .max_height(320.0)
                .flex_col()
                .padding(4.0)
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

fn account_menu() -> AnyView {
    menu_surface(
        Stack::vertical((
            menu_label("My Account", false),
            menu_item(
                Some("user"),
                "Profile",
                Some("Shift Cmd P"),
                None,
                MenuVariant::Default,
                false,
                false,
                false,
            ),
            menu_item(
                Some("credit-card"),
                "Billing",
                Some("Cmd B"),
                None,
                MenuVariant::Default,
                false,
                false,
                false,
            ),
            menu_item(
                Some("settings"),
                "Settings",
                Some("Cmd S"),
                None,
                MenuVariant::Default,
                false,
                false,
                false,
            ),
            menu_item(
                Some("keyboard"),
                "Keyboard shortcuts",
                Some("Cmd K"),
                None,
                MenuVariant::Default,
                true,
                false,
                false,
            ),
            menu_separator(),
            checkbox_item("Show sidebar", true, false, false),
            checkbox_item("Command palette", false, false, false),
            menu_separator(),
            menu_item(
                Some("log-out"),
                "Log out",
                None,
                None,
                MenuVariant::Destructive,
                false,
                false,
                false,
            ),
        )),
        240.0,
    )
}

fn choice_menu() -> AnyView {
    menu_surface(
        Stack::vertical((
            menu_label("Panel", true),
            checkbox_item("Show minimap", true, false, true),
            checkbox_item("Show rulers", false, false, true),
            checkbox_item("Show comments", false, true, true),
            menu_separator(),
            menu_label("Density", true),
            radio_item("Compact", false, false, true),
            radio_item("Default", true, false, true),
            radio_item("Comfortable", false, false, true),
        )),
        220.0,
    )
}

fn trigger_preview() -> AnyView {
    let open = RwSignal::new(false);
    anchored_portal(
        Button::new(
            Stack::horizontal(("Open menu", icon("chevron-down", 16.0)))
                .style(|s| s.items_center().gap(6.0)),
        )
        .action(move || open.update(|value| *value = !*value)),
        open,
        PortalPosition::bottom_start(8.0),
        account_menu,
    )
}

fn submenu_portal_content(sub_open: RwSignal<bool>) -> AnyView {
    menu_surface(
        Stack::vertical((
            menu_item(
                Some("plus"),
                "New task",
                Some("Cmd N"),
                None,
                MenuVariant::Default,
                false,
                false,
                false,
            ),
            anchored_portal(
                sub_trigger("Move to", sub_open.get(), false)
                    .on_event_stop(listener::PointerEnter, move |_, _| sub_open.set(true))
                    .on_event_stop(listener::Click, move |_, _| {
                        sub_open.update(|value| *value = !*value)
                    }),
                sub_open,
                PortalPosition::right_start(4.0),
                move || {
                    menu_surface(
                        Stack::vertical((
                            radio_item("Backlog", false, false, false),
                            radio_item("Todo", true, false, false),
                            radio_item("In Progress", false, false, false),
                            radio_item("Done", false, true, false),
                        )),
                        160.0,
                    )
                },
            ),
            sub_trigger("Assign to", false, false),
            menu_separator(),
            menu_item(
                None,
                "Archive",
                None,
                None,
                MenuVariant::Default,
                false,
                false,
                false,
            ),
        )),
        176.0,
    )
}

fn submenu_portal_preview() -> AnyView {
    let open = RwSignal::new(false);
    let sub_open = RwSignal::new(false);
    anchored_portal(
        Button::new(
            Stack::horizontal(("Open submenu", icon("chevron-down", 16.0)))
                .style(|s| s.items_center().gap(6.0)),
        )
        .action(move || open.update(|value| *value = !*value)),
        open,
        PortalPosition::bottom_start(8.0),
        move || submenu_portal_content(sub_open),
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

pub fn dropdown_menu_view() -> impl IntoView {
    Stack::vertical((
        "Dropdown Menu".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::horizontal((
            section("Trigger + Content", trigger_preview()),
            section("Checkbox / Radio", choice_menu()),
            section("Submenu", submenu_portal_preview()),
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
