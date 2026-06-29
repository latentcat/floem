use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    prelude::*,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

fn empty_icon(name: &'static str) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| icon.style(|s| s.size(16.0, 16.0)).into_any())
        .unwrap_or_else(|| Empty::new().style(|s| s.size(16.0, 16.0)).into_any())
}

fn empty_media_icon(name: &'static str) -> AnyView {
    Stack::new((empty_icon(name),))
        .style(|s| {
            s.size(32.0, 32.0)
                .items_center()
                .justify_center()
                .border_radius(8.0)
                .corner_smoothing(0.6)
                .with_theme(|s, t| s.background(t.muted()).color(t.foreground()))
        })
        .into_any()
}

fn empty_title(title: &'static str) -> AnyView {
    title
        .style(|s| {
            s.font_size(14.0)
                .font_weight(FontWeight::MEDIUM)
                .with_theme(|s, t| s.color(t.foreground()))
        })
        .into_any()
}

fn empty_description(description: &'static str) -> AnyView {
    description
        .style(|s| {
            s.width(320.0)
                .font_size(14.0)
                .line_height(1.625)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        })
        .into_any()
}

pub fn empty_view() -> impl IntoView {
    Stack::vertical((
        Stack::vertical((
            empty_media_icon("inbox"),
            empty_title("No projects yet"),
            empty_description(
                "Create your first project to start organizing work across your team.",
            ),
        ))
        .style(|s| s.max_width(384.0).items_center().gap(8.0)),
        Stack::horizontal((
            Button::new("Create project"),
            Button::new("Import").style(|s| {
                s.with_theme(|s, t| {
                    s.background(t.background())
                        .border_color(t.border())
                        .color(t.foreground())
                        .hover(|s| s.background(t.muted()))
                })
                .border(1.0)
            }),
        ))
        .style(|s| s.items_center().gap(10.0)),
    ))
    .style(|s| {
        s.width_full()
            .min_height(360.0)
            .items_center()
            .justify_center()
            .gap(16.0)
            .padding(24.0)
            .border(1.0)
            .border_radius(12.0)
            .corner_smoothing(0.6)
            .with_theme(|s, t| {
                s.background(t.background())
                    .color(t.foreground())
                    .border_color(t.border())
            })
    })
}
