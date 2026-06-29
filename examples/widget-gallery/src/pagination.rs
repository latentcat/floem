use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    peniko::Color,
    prelude::*,
    style::{Opacity, Style},
    taffy::FlexWrap,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

fn page_icon(name: &'static str) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(|s| s.size(16.0, 16.0).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(|s| s.size(16.0, 16.0)).into_any())
}

fn ghost_style() -> Style {
    Style::new()
        .background(Color::TRANSPARENT)
        .border_color(Color::TRANSPARENT)
        .with_theme(|s, t| {
            s.color(t.foreground())
                .hover(|s| s.background(t.muted()).color(t.foreground()))
                .disabled(|s| s.set(Opacity, 0.5).unset_cursor())
        })
}

fn outline_style() -> Style {
    Style::new().border(1.0).with_theme(|s, t| {
        s.background(t.background())
            .border_color(t.border())
            .color(t.foreground())
            .hover(|s| s.background(t.muted()))
    })
}

fn page_button(label: &'static str, active: bool, disabled: bool) -> Button {
    Button::new(label).style(move |s| {
        s.size(32.0, 32.0)
            .padding(0.0)
            .apply(if active {
                outline_style()
            } else {
                ghost_style()
            })
            .apply_if(disabled, |s| s.set_disabled(true))
    })
}

fn icon_text_button(
    icon_name: &'static str,
    label: &'static str,
    icon_end: bool,
    disabled: bool,
) -> Button {
    let content = if icon_end {
        Stack::horizontal((label.into_any(), page_icon(icon_name)))
    } else {
        Stack::horizontal((page_icon(icon_name), label.into_any()))
    };

    Button::new(content.style(|s| s.items_center().gap(6.0))).style(move |s| {
        s.height(32.0)
            .padding_horiz(8.0)
            .font_size(14.0)
            .apply(ghost_style())
            .apply_if(disabled, |s| s.set_disabled(true))
    })
}

fn icon_button(icon_name: &'static str, disabled: bool) -> Button {
    Button::new(page_icon(icon_name)).style(move |s| {
        s.size(32.0, 32.0)
            .padding(0.0)
            .apply(ghost_style())
            .apply_if(disabled, |s| s.set_disabled(true))
    })
}

fn ellipsis() -> AnyView {
    Stack::new((page_icon("ellipsis"),))
        .style(|s| {
            s.size(32.0, 32.0)
                .items_center()
                .justify_center()
                .with_theme(|s, t| s.color(t.muted_foreground()))
        })
        .into_any()
}

fn pagination_row(content: impl IntoView + 'static) -> AnyView {
    content
        .style(|s| s.items_center().gap(2.0).flex_wrap(FlexWrap::Wrap))
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

pub fn pagination_view() -> impl IntoView {
    Stack::vertical((
        "Pagination".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::vertical((
            section(
                "Default",
                pagination_row(Stack::horizontal((
                    icon_text_button("chevron-left", "Previous", false, false),
                    page_button("1", false, false),
                    page_button("2", true, false),
                    page_button("3", false, false),
                    ellipsis(),
                    page_button("10", false, false),
                    icon_text_button("chevron-right", "Next", true, false),
                ))),
            ),
            section(
                "Compact",
                pagination_row(Stack::horizontal((
                    icon_button("chevron-left", true),
                    page_button("1", true, false),
                    page_button("2", false, false),
                    ellipsis(),
                    page_button("8", false, false),
                    icon_button("chevron-right", false),
                ))),
            ),
            section(
                "Disabled Edges",
                pagination_row(Stack::horizontal((
                    icon_text_button("chevron-left", "Previous", false, true),
                    page_button("1", true, false),
                    page_button("2", false, false),
                    page_button("3", false, false),
                    icon_text_button("chevron-right", "Next", true, false),
                ))),
            ),
        ))
        .style(|s| s.flex_col().items_center().gap(24.0).width_full()),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
