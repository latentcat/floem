use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    prelude::*,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators, TextInput},
};

use crate::form::{form, form_item};

fn group_icon(name: &'static str) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| icon.style(|s| s.size(16.0, 16.0)).into_any())
        .unwrap_or_else(|| Empty::new().style(|s| s.size(16.0, 16.0)).into_any())
}

fn group_input(placeholder: &'static str) -> TextInput {
    let value = RwSignal::new(String::new());
    TextInput::new(value).placeholder(placeholder).style(|s| {
        s.flex_grow(1.0)
            .height(30.0)
            .border(0.0)
            .background(floem::peniko::Color::TRANSPARENT)
            .padding_horiz(4.0)
            .focus_visible(|s| s.outline(0.0))
    })
}

fn input_group(children: impl floem::view::IntoViewIter + 'static) -> AnyView {
    Stack::horizontal(children)
        .style(|s| {
            s.width(360.0)
                .height(32.0)
                .items_center()
                .border(1.0)
                .border_radius(8.0)
                .corner_smoothing(0.6)
                .with_theme(|s, t| {
                    s.background(t.input_background())
                        .border_color(t.input())
                        .color(t.foreground())
                })
                .focus_visible(|s| {
                    s.with_theme(|s, t| {
                        s.border_color(t.ring())
                            .outline(3.0)
                            .outline_color(t.ring_focus())
                    })
                })
        })
        .into_any()
}

fn addon(view: impl floem::view::IntoViewIter + 'static, start: bool) -> AnyView {
    Stack::horizontal(view)
        .style(move |s| {
            s.height_full()
                .items_center()
                .justify_center()
                .gap(8.0)
                .padding_left(if start { 10.0 } else { 6.0 })
                .padding_right(if start { 6.0 } else { 10.0 })
                .font_size(14.0)
                .font_weight(FontWeight::MEDIUM)
                .selectable(false)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        })
        .into_any()
}

pub fn input_group_view() -> impl IntoView {
    form((
        form_item(
            "Inline start:",
            input_group((
                addon((group_icon("search"),), true),
                group_input("Search documentation..."),
            )),
        ),
        form_item(
            "Inline end:",
            input_group((group_input("example.com"), addon((".com",), false))),
        ),
        form_item(
            "Button:",
            input_group((
                group_input("Invite by email"),
                Button::new("Send").style(|s| s.height(26.0).padding_horiz(8.0).font_size(13.0)),
            )),
        ),
        form_item(
            "Block:",
            Stack::vertical((
                Stack::horizontal((group_icon("sparkles"), "AI Prompt")).style(|s| {
                    s.width(360.0)
                        .items_center()
                        .gap(8.0)
                        .padding_horiz(10.0)
                        .padding_vert(8.0)
                        .border(1.0)
                        .border_bottom(0.0)
                        .border_top_left_radius(8.0)
                        .border_top_right_radius(8.0)
                        .corner_smoothing(0.6)
                        .font_size(14.0)
                        .with_theme(|s, t| {
                            s.background(t.muted())
                                .border_color(t.input())
                                .color(t.muted_foreground())
                        })
                }),
                input_group((group_input("Ask anything..."),))
                    .style(|s| s.border_top_left_radius(0.0).border_top_right_radius(0.0)),
            ))
            .style(|s| s.flex_col()),
        ),
    ))
    .style(|s| s.with_theme(|s, t| s.background(t.background()).color(t.foreground())))
}
