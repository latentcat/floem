use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    prelude::*,
    style::{Foreground, Transition},
    taffy::FlexWrap,
    text::FontWeight,
    theme::StyleThemeExt,
};

fn crumb_icon(name: &'static str, size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

fn crumb_link(label: &'static str) -> AnyView {
    label
        .style(|s| {
            s.font_size(14.0)
                .cursor(floem::style::CursorStyle::Pointer)
                .with_theme(|s, t| {
                    s.color(t.muted_foreground())
                        .hover(|s| s.color(t.foreground()))
                })
                .transition(Foreground, Transition::linear(100.millis()))
        })
        .into_any()
}

fn crumb_link_icon(icon_name: &'static str, label: &'static str) -> AnyView {
    Stack::horizontal((crumb_icon(icon_name, 14.0), crumb_link(label)))
        .style(|s| s.items_center().gap(4.0))
        .into_any()
}

fn crumb_page(label: &'static str) -> AnyView {
    label
        .style(|s| s.font_size(14.0).with_theme(|s, t| s.color(t.foreground())))
        .into_any()
}

fn separator_icon(name: &'static str) -> AnyView {
    crumb_icon(name, 14.0).style(|s| s.with_theme(|s, t| s.color(t.muted_foreground())))
}

fn ellipsis() -> AnyView {
    Stack::new((crumb_icon("ellipsis", 16.0),))
        .style(|s| {
            s.size(20.0, 20.0)
                .items_center()
                .justify_center()
                .with_theme(|s, t| s.color(t.muted_foreground()))
        })
        .into_any()
}

fn breadcrumb_row(content: impl IntoView + 'static) -> AnyView {
    content
        .style(|s| {
            s.items_center()
                .gap(6.0)
                .flex_wrap(FlexWrap::Wrap)
                .with_theme(|s, t| s.color(t.muted_foreground()))
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

pub fn breadcrumb_view() -> impl IntoView {
    Stack::vertical((
        "Breadcrumb".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::vertical((
            section(
                "Basic",
                breadcrumb_row(Stack::horizontal((
                    crumb_link("Home"),
                    separator_icon("chevron-right"),
                    crumb_link("Components"),
                    separator_icon("chevron-right"),
                    crumb_page("Breadcrumb"),
                ))),
            ),
            section(
                "Collapsed",
                breadcrumb_row(Stack::horizontal((
                    crumb_link("Home"),
                    separator_icon("chevron-right"),
                    ellipsis(),
                    separator_icon("chevron-right"),
                    crumb_link("Docs"),
                    separator_icon("chevron-right"),
                    crumb_page("Installation"),
                ))),
            ),
            section(
                "Icon Root",
                breadcrumb_row(Stack::horizontal((
                    crumb_link_icon("house", "Home"),
                    separator_icon("slash"),
                    crumb_link("Projects"),
                    separator_icon("slash"),
                    crumb_link("Floem"),
                    separator_icon("slash"),
                    crumb_page("Gallery"),
                ))),
            ),
            section(
                "Long Path",
                breadcrumb_row(Stack::horizontal((
                    crumb_link("Workspace"),
                    separator_icon("chevron-right"),
                    crumb_link("Design system"),
                    separator_icon("chevron-right"),
                    crumb_link("Navigation"),
                    separator_icon("chevron-right"),
                    crumb_page("Breadcrumb current page"),
                ))),
            ),
        ))
        .style(|s| s.flex_col().gap(24.0)),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
