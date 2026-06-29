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

fn icon(name: &'static str, size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

fn combobox_trigger(
    value: &'static str,
    placeholder: bool,
    disabled: bool,
    invalid: bool,
    show_clear: bool,
) -> AnyView {
    Stack::horizontal((
        value.style(move |s| {
            s.text_ellipsis()
                .font_size(14.0)
                .apply_if(placeholder, |s| {
                    s.with_theme(|s, t| s.color(t.muted_foreground()))
                })
        }),
        Empty::new().style(|s| s.flex_grow(1.0)),
        if show_clear {
            icon("x", 14.0).style(|s| s.with_theme(|s, t| s.color(t.muted_foreground())))
        } else {
            Empty::new().into_any()
        },
        icon("chevron-down", 16.0).style(|s| s.with_theme(|s, t| s.color(t.muted_foreground()))),
    ))
    .style(move |s| {
        s.width(240.0)
            .height(32.0)
            .items_center()
            .gap(6.0)
            .padding_left(10.0)
            .padding_right(8.0)
            .border(1.0)
            .border_radius(8.0)
            .corner_smoothing(0.6)
            .selectable(false)
            .transition(Background, Transition::linear(100.millis()))
            .transition(Foreground, Transition::linear(100.millis()))
            .with_theme(move |s, t| {
                s.background(t.input_background())
                    .border_color(t.input())
                    .color(t.foreground())
                    .hover(|s| s.background(t.input_background()))
                    .focus_visible(|s| {
                        s.outline(3.0)
                            .outline_color(t.ring_focus())
                            .border_color(t.ring())
                    })
                    .apply_if(invalid, |s| {
                        s.border_color(t.danger()).outline(3.0).outline_color(
                            t.def(|t| t.danger().with_alpha(if t.is_dark { 0.4 } else { 0.2 })),
                        )
                    })
                    .disabled(|s| s.set(Opacity, 0.5).unset_cursor())
            })
            .apply_if(disabled, |s| s.set_disabled(true))
    })
    .into_any()
}

fn combobox_input(value: &'static str, disabled: bool) -> AnyView {
    Stack::horizontal((
        TextInput::new(RwSignal::new(value.to_string()))
            .placeholder("Search framework...")
            .style(|s| {
                s.flex_grow(1.0)
                    .height(28.0)
                    .border(0.0)
                    .padding_horiz(8.0)
                    .background(Color::TRANSPARENT)
            }),
        icon("search", 16.0).style(|s| s.with_theme(|s, t| s.color(t.muted_foreground()))),
    ))
    .style(move |s| {
        s.height(32.0)
            .items_center()
            .gap(8.0)
            .padding_left(2.0)
            .padding_right(8.0)
            .border(1.0)
            .border_radius(8.0)
            .corner_smoothing(0.6)
            .with_theme(|s, t| {
                s.background(t.input_background())
                    .border_color(t.input())
                    .disabled(|s| s.set(Opacity, 0.5).unset_cursor())
            })
            .apply_if(disabled, |s| s.set_disabled(true))
    })
    .into_any()
}

fn combo_label(text: &'static str) -> AnyView {
    text.style(|s| {
        s.padding_horiz(8.0)
            .padding_vert(6.0)
            .font_size(12.0)
            .with_theme(|s, t| s.color(t.muted_foreground()))
    })
    .into_any()
}

fn combo_separator() -> AnyView {
    Empty::new()
        .style(|s| {
            s.height(1.0)
                .margin_horiz(-4.0)
                .margin_vert(4.0)
                .with_theme(|s, t| s.background(t.border()))
        })
        .into_any()
}

fn combo_item(label: &'static str, selected: bool, highlighted: bool, disabled: bool) -> AnyView {
    Stack::horizontal((
        label.style(|s| s.font_size(14.0)),
        Empty::new().style(|s| s.flex_grow(1.0)),
        if selected {
            icon("check", 16.0)
        } else {
            Empty::new().style(|s| s.size(16.0, 16.0)).into_any()
        },
    ))
    .style(move |s| {
        s.min_height(30.0)
            .items_center()
            .gap(8.0)
            .padding_left(6.0)
            .padding_right(8.0)
            .padding_vert(4.0)
            .border_radius(6.0)
            .selectable(false)
            .transition(Background, Transition::linear(100.millis()))
            .transition(Foreground, Transition::linear(100.millis()))
            .with_theme(|s, t| {
                s.color(t.popover_foreground())
                    .hover(|s| s.background(t.accent()).color(t.accent_foreground()))
                    .disabled(|s| s.set(Opacity, 0.5).unset_cursor())
            })
            .apply_if(highlighted, |s| {
                s.with_theme(|s, t| s.background(t.accent()).color(t.accent_foreground()))
            })
            .apply_if(disabled, |s| s.set_disabled(true))
    })
    .into_any()
}

fn combo_empty() -> AnyView {
    "No framework found."
        .style(|s| {
            s.width_full()
                .items_center()
                .justify_center()
                .padding_vert(8.0)
                .font_size(14.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        })
        .into_any()
}

fn combo_content(empty: bool) -> AnyView {
    Stack::vertical((
        Stack::vertical((combobox_input(if empty { "ember" } else { "" }, false),))
            .style(|s| s.padding(4.0).padding_bottom(0.0)),
        if empty {
            combo_empty()
        } else {
            Stack::vertical((
                combo_label("Frameworks"),
                combo_item("Next.js", true, true, false),
                combo_item("SvelteKit", false, false, false),
                combo_item("Nuxt.js", false, false, false),
                combo_separator(),
                combo_label("Meta frameworks"),
                combo_item("Remix", false, false, false),
                combo_item("Astro", false, false, true),
            ))
            .style(|s| s.flex_col().padding(4.0))
            .into_any()
        },
    ))
    .style(|s| {
        s.width(276.0)
            .max_height(320.0)
            .flex_col()
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

fn chip(label: &'static str, removable: bool) -> AnyView {
    Stack::horizontal((
        label.style(|s| s.font_size(12.0).font_weight(FontWeight::MEDIUM)),
        if removable {
            icon("x", 12.0).style(|s| s.opacity(0.5))
        } else {
            Empty::new().into_any()
        },
    ))
    .style(|s| {
        s.height(21.0)
            .items_center()
            .justify_center()
            .gap(4.0)
            .padding_left(6.0)
            .padding_right(4.0)
            .border_radius(4.0)
            .with_theme(|s, t| s.background(t.muted()).color(t.foreground()))
    })
    .into_any()
}

fn chips_input(invalid: bool, disabled: bool) -> AnyView {
    Stack::horizontal((
        chip("Next.js", true),
        chip("Remix", true),
        "Search...".style(|s| {
            s.min_width(80.0)
                .font_size(14.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
    ))
    .style(move |s| {
        s.min_height(32.0)
            .width(360.0)
            .items_center()
            .gap(4.0)
            .padding_horiz(6.0)
            .padding_vert(4.0)
            .border(1.0)
            .border_radius(8.0)
            .corner_smoothing(0.6)
            .flex_wrap(FlexWrap::Wrap)
            .with_theme(move |s, t| {
                s.background(t.input_background())
                    .border_color(t.input())
                    .color(t.foreground())
                    .focus_visible(|s| {
                        s.outline(3.0)
                            .outline_color(t.ring_focus())
                            .border_color(t.ring())
                    })
                    .apply_if(invalid, |s| {
                        s.border_color(t.danger()).outline(3.0).outline_color(
                            t.def(|t| t.danger().with_alpha(if t.is_dark { 0.4 } else { 0.2 })),
                        )
                    })
                    .disabled(|s| s.set(Opacity, 0.5).unset_cursor())
            })
            .apply_if(disabled, |s| s.set_disabled(true))
    })
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

pub fn combobox_view() -> impl IntoView {
    Stack::vertical((
        "Combobox".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::horizontal((
            section(
                "Popover list",
                Stack::vertical((
                    combobox_trigger("Next.js", false, false, false, true),
                    combo_content(false),
                ))
                .style(|s| s.flex_col().gap(8.0)),
            ),
            section(
                "Trigger states",
                Stack::vertical((
                    combobox_trigger("Select framework", true, false, false, false),
                    combobox_trigger("Disabled", false, true, false, false),
                    combobox_trigger("Invalid", false, false, true, false),
                ))
                .style(|s| s.flex_col().gap(8.0)),
            ),
            section("Empty", combo_content(true)),
            section(
                "Chips",
                Stack::vertical((
                    chips_input(false, false),
                    chips_input(true, false),
                    chips_input(false, true),
                ))
                .style(|s| s.flex_col().gap(10.0)),
            ),
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
