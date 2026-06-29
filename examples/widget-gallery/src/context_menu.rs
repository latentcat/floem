use std::time::Duration;

use floem::{
    AnyView, IntoView,
    easing::Spring,
    icons::{self as icon_library, IconLibrary},
    kurbo::Affine,
    menu::*,
    peniko::Color,
    prelude::*,
    style::{Background, Foreground, Opacity, Transition},
    taffy::FlexWrap,
    text::FontWeight,
    theme::StyleThemeExt,
    unit::Angle,
};

#[derive(Clone, Copy)]
enum ItemVariant {
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

fn context_label(text: &'static str, inset: bool) -> AnyView {
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

fn context_separator() -> AnyView {
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
fn context_item(
    leading: Option<&'static str>,
    label: &'static str,
    shortcut_text: Option<&'static str>,
    indicator: Option<&'static str>,
    variant: ItemVariant,
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
        shortcut_text
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
                ItemVariant::Default => s
                    .color(t.popover_foreground())
                    .apply_if(open, |s| {
                        s.background(t.accent()).color(t.accent_foreground())
                    })
                    .hover(|s| s.background(t.accent()).color(t.accent_foreground())),
                ItemVariant::Destructive => s.color(t.danger()).hover(|s| {
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

fn checkbox_item(label: &'static str, checked: bool, disabled: bool, inset: bool) -> AnyView {
    context_item(
        None,
        label,
        None,
        checked.then_some("check"),
        ItemVariant::Default,
        disabled,
        inset,
        false,
    )
}

fn radio_item(label: &'static str, checked: bool, disabled: bool, inset: bool) -> AnyView {
    context_item(
        None,
        label,
        None,
        checked.then_some("check"),
        ItemVariant::Default,
        disabled,
        inset,
        false,
    )
}

fn sub_trigger(label: &'static str, open: bool, inset: bool) -> AnyView {
    context_item(
        None,
        label,
        None,
        Some("chevron-right"),
        ItemVariant::Default,
        false,
        inset,
        open,
    )
}

fn context_surface(content: impl IntoView + 'static, width: f64, sub_content: bool) -> AnyView {
    content
        .style(move |s| {
            s.width(width)
                .min_width(if sub_content { 128.0 } else { 144.0 })
                .max_height(320.0)
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

fn canvas_menu() -> AnyView {
    context_surface(
        Stack::vertical((
            context_label("Canvas", false),
            context_item(
                Some("undo-2"),
                "Back",
                Some("Cmd ["),
                None,
                ItemVariant::Default,
                false,
                false,
                false,
            ),
            context_item(
                Some("redo-2"),
                "Forward",
                Some("Cmd ]"),
                None,
                ItemVariant::Default,
                true,
                false,
                false,
            ),
            context_item(
                Some("rotate-cw"),
                "Reload",
                Some("Cmd R"),
                None,
                ItemVariant::Default,
                false,
                false,
                false,
            ),
            context_separator(),
            checkbox_item("Show Grid", true, false, false),
            sub_trigger("Transform", true, false),
            context_separator(),
            context_item(
                Some("trash-2"),
                "Delete",
                None,
                None,
                ItemVariant::Destructive,
                false,
                false,
                false,
            ),
        )),
        224.0,
        false,
    )
}

fn options_menu() -> AnyView {
    context_surface(
        Stack::vertical((
            context_label("View", true),
            checkbox_item("Show minimap", true, false, true),
            checkbox_item("Show rulers", false, false, true),
            checkbox_item("Show comments", false, true, true),
            context_separator(),
            context_label("Zoom", true),
            radio_item("50%", false, false, true),
            radio_item("100%", true, false, true),
            radio_item("200%", false, false, true),
        )),
        220.0,
        false,
    )
}

fn submenu_preview() -> AnyView {
    Stack::horizontal((
        context_surface(
            Stack::vertical((
                context_item(
                    Some("copy-plus"),
                    "Duplicate",
                    Some("Cmd D"),
                    None,
                    ItemVariant::Default,
                    false,
                    false,
                    false,
                ),
                sub_trigger("Export as", true, false),
                sub_trigger("Share", false, false),
                context_separator(),
                context_item(
                    Some("info"),
                    "Properties",
                    None,
                    None,
                    ItemVariant::Default,
                    false,
                    false,
                    false,
                ),
            )),
            176.0,
            false,
        ),
        context_surface(
            Stack::vertical((
                context_item(
                    None,
                    "PDF",
                    None,
                    None,
                    ItemVariant::Default,
                    false,
                    false,
                    false,
                ),
                context_item(
                    None,
                    "PNG",
                    None,
                    None,
                    ItemVariant::Default,
                    false,
                    false,
                    false,
                ),
                context_item(
                    None,
                    "SVG",
                    None,
                    None,
                    ItemVariant::Default,
                    false,
                    false,
                    false,
                ),
                context_separator(),
                context_item(
                    None,
                    "HTML",
                    None,
                    None,
                    ItemVariant::Default,
                    true,
                    false,
                    false,
                ),
            )),
            140.0,
            true,
        ),
    ))
    .style(|s| s.items_start().gap(8.0))
    .into_any()
}

fn trigger_surface() -> AnyView {
    Stack::vertical((
        Stack::horizontal((
            icon("mouse-pointer-click", 18.0),
            Stack::vertical((
                "Right-click target".style(|s| {
                    s.font_size(14.0)
                        .font_weight(FontWeight::MEDIUM)
                        .with_theme(|s, t| s.color(t.foreground()))
                }),
                "ContextMenuTrigger is select-none.".style(|s| {
                    s.font_size(12.0)
                        .with_theme(|s, t| s.color(t.muted_foreground()))
                }),
            ))
            .style(|s| s.flex_col().gap(2.0)),
        ))
        .style(|s| s.items_center().gap(10.0)),
        canvas_menu().style(|s| s.margin_top(8.0)),
    ))
    .style(|s| s.flex_col().selectable(false))
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

pub fn menu_view() -> impl IntoView {
    let export_submenu = |m: SubMenu| {
        m.item("PDF", |i| i.action(|| println!("Exporting as PDF...")))
            .item("PNG", |i| i.action(|| println!("Exporting as PNG...")))
            .item("SVG", |i| i.action(|| println!("Exporting as SVG...")))
            .separator()
            .item("HTML", |i| {
                i.enabled(false)
                    .action(|| println!("HTML export coming soon..."))
            })
    };
    let popout_menu = move || {
        Menu::new()
            .item("New Document", |i| {
                i.action(|| println!("Creating new document..."))
            })
            .item("Open Recent", |i| {
                i.action(|| println!("Opening recent files..."))
            })
            .separator()
            .submenu("Export As", export_submenu)
            .separator()
            .item("Auto Save", |i| {
                i.checked(true).action(|| println!("Toggled auto save"))
            })
            .item("Show Grid", |i| {
                i.checked(false)
                    .action(|| println!("Toggled grid visibility"))
            })
            .separator()
            .item("Preferences", |i| {
                i.action(|| println!("Opening preferences..."))
            })
    };

    let rotation = RwSignal::new(Angle::Deg(0.));
    let transform = RwSignal::new(Affine::IDENTITY);
    let transform_submenu = move |m: SubMenu| {
        m.item("Rotate 90°", move |i| {
            i.action(move || {
                rotation.update(|r| *r = Angle::Deg(r.to_degrees() - 90.));
                println!("Rotating 90 degrees...")
            })
        })
        .item("Flip Horizontal", move |i| {
            i.action(move || {
                transform.update(|t| {
                    let flip = Affine::scale_non_uniform(-1.0, 1.0);
                    *t *= flip;
                });
                println!("Flipping horizontally...");
            })
        })
        .item("Flip Vertical", move |i| {
            i.action(move || {
                transform.update(|t| {
                    let flip = Affine::scale_non_uniform(1.0, -1.0);
                    *t *= flip;
                });
                println!("Flipping vertically...");
            })
        })
        .item("Scale", move |i| {
            i.action(move || {
                transform.update(|t| *t = t.then_scale(2.));
                println!("Scaling...");
            })
        })
        .separator()
        .item("Reset Transform", move |i| {
            i.action(move || {
                transform.set(Affine::IDENTITY);
                rotation.set(0.0.into());
                println!("Resetting transform...");
            })
        })
    };

    let context_menu = move || {
        Menu::new()
            .item("Cut", |i| {
                i.enabled(false).action(|| println!("Cut to clipboard"))
            })
            .item("Copy", |i| i.action(|| println!("Copied to clipboard")))
            .item("Paste", |i| {
                i.enabled(false)
                    .action(|| println!("Pasted from clipboard"))
            })
            .separator()
            .submenu("Transform", transform_submenu)
            .separator()
            .item("Duplicate", |i| {
                i.action(|| println!("Creating duplicate..."))
            })
            .item("Delete", |i| i.action(|| println!("Deleting item...")))
            .separator()
            .item("Properties", |i| {
                i.action(|| println!("Opening properties panel..."))
            })
    };

    let popout_button = "Click me (Popout menu)"
        .class(ButtonClass)
        .style(|s| s.padding(10.0))
        .popout_menu(popout_menu);

    let context_button = "Right click me (Context menu)"
        .class(ButtonClass)
        .on_event_stop(el::DoubleClick, move |_, _| {
            rotation.update(|r| *r = Angle::Deg(r.to_degrees() - 90.));
        })
        .style(move |s| {
            s.padding(10.0)
                .border(1.0)
                .transform(transform.get())
                .rotate(rotation.get())
                .transition_rotate(Transition::new(
                    Duration::from_millis(500),
                    Spring::snappy(),
                ))
                .transition_transform(Transition::new(
                    Duration::from_millis(500),
                    Spring::snappy(),
                ))
        })
        .context_menu(context_menu);

    Stack::vertical((
        "Context Menu".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::horizontal((
            section("Trigger + Content", trigger_surface()),
            section("Checkbox / Radio", options_menu()),
            section("Submenu", submenu_preview()),
            section(
                "Native actions",
                Stack::vertical((popout_button, context_button)).style(|s| s.flex_col().gap(10.0)),
            ),
        ))
        .style(|s| s.items_start().gap(24.0).flex_wrap(FlexWrap::Wrap)),
    ))
    .style(|s| {
        s.selectable(false)
            .flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
