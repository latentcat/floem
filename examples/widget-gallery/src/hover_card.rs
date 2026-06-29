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

use crate::shadcn_style::{fixed_square, text_column, wrap_text};

fn icon(name: &'static str, size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

fn avatar(initials: &'static str, size: f64) -> AnyView {
    initials
        .style(move |s| {
            s.apply(fixed_square(size))
                .items_center()
                .justify_center()
                .border_radius(100.0)
                .font_size(size * 0.32)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.background(t.primary()).color(t.primary_foreground()))
        })
        .into_any()
}

fn hover_card_surface(content: impl IntoView + 'static) -> AnyView {
    content
        .style(|s| {
            s.width(256.0)
                .flex_col()
                .gap(12.0)
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

fn title(text: &'static str) -> AnyView {
    text.style(|s| {
        s.font_size(14.0)
            .font_weight(FontWeight::SEMI_BOLD)
            .apply(wrap_text())
            .with_theme(|s, t| s.color(t.popover_foreground()))
    })
    .into_any()
}

fn description(text: &'static str) -> AnyView {
    text.style(|s| {
        s.font_size(13.0)
            .line_height(1.35)
            .apply(wrap_text())
            .with_theme(|s, t| s.color(t.muted_foreground()))
    })
    .into_any()
}

fn meta_row(icon_name: &'static str, text: &'static str) -> AnyView {
    Stack::horizontal((
        icon(icon_name, 14.0).style(|s| s.with_theme(|s, t| s.color(t.muted_foreground()))),
        text.style(|s| {
            s.font_size(12.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
    ))
    .style(|s| s.items_center().gap(6.0))
    .into_any()
}

fn profile_card() -> AnyView {
    hover_card_surface(Stack::vertical((
        Stack::horizontal((
            avatar("SC", 40.0),
            Stack::vertical((
                title("@shadcn"),
                description("Design components for copy and paste."),
            ))
            .style(|s| s.apply(text_column()).gap(4.0)),
        ))
        .style(|s| s.items_start().gap(10.0)),
        meta_row("calendar-days", "Joined December 2021"),
        Stack::horizontal((
            "12.8k followers".style(|s| {
                s.font_size(12.0)
                    .font_weight(FontWeight::MEDIUM)
                    .with_theme(|s, t| s.color(t.popover_foreground()))
            }),
            "1.2k following".style(|s| {
                s.font_size(12.0)
                    .with_theme(|s, t| s.color(t.muted_foreground()))
            }),
        ))
        .style(|s| s.items_center().gap(10.0)),
    )))
}

fn repository_card() -> AnyView {
    hover_card_surface(Stack::vertical((
        Stack::horizontal((
            icon("package", 18.0),
            Stack::vertical((title("ui"), description("Beautifully designed components.")))
                .style(|s| s.apply(text_column()).gap(3.0)),
        ))
        .style(|s| s.items_start().gap(8.0)),
        Stack::horizontal((
            meta_row("star", "84k"),
            meta_row("git-fork", "5.2k"),
            meta_row("circle", "TypeScript"),
        ))
        .style(|s| s.items_center().gap(10.0).flex_wrap(FlexWrap::Wrap)),
    )))
}

fn file_card() -> AnyView {
    hover_card_surface(Stack::vertical((
        Stack::horizontal((
            icon("file-code-2", 18.0),
            Stack::vertical((
                title("button.tsx"),
                description("Updated in the current preset."),
            ))
            .style(|s| s.apply(text_column()).gap(3.0)),
        ))
        .style(|s| s.items_start().gap(8.0)),
        meta_row("clock", "Modified 2 hours ago"),
        meta_row("git-branch", "preset/b0"),
    )))
}

fn team_card() -> AnyView {
    let avatar_group_width = 36.0 + (36.0 - 10.0) * 2.0;
    let avatars = Stack::horizontal((
        avatar("DS", 36.0),
        avatar("FE", 36.0).style(|s| s.margin_left(-10.0)),
        avatar("QA", 36.0).style(|s| s.margin_left(-10.0)),
    ))
    .style(move |s| {
        s.width(avatar_group_width)
            .min_width(avatar_group_width)
            .max_width(avatar_group_width)
            .items_center()
            .flex_shrink(0.0)
    });

    hover_card_surface(Stack::vertical((
        Stack::horizontal((
            avatars,
            Stack::vertical((
                title("Design System"),
                description("3 maintainers active this week."),
            ))
            .style(|s| {
                s.apply(text_column())
                    .gap(3.0)
                    .flex_grow(1.0)
                    .min_width(0.0)
            }),
        ))
        .style(|s| s.items_center().gap(10.0).min_width(0.0)),
        meta_row("message-circle", "18 component updates reviewed"),
    )))
}

fn section(title_text: &'static str, content: impl IntoView + 'static) -> AnyView {
    Stack::vertical((
        title_text.style(|s| {
            s.font_size(14.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        content,
    ))
    .style(|s| s.flex_col().gap(10.0))
    .into_any()
}

pub fn hover_card_view() -> impl IntoView {
    Stack::vertical((
        "Hover Card".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        section(
            "Triggers",
            Stack::horizontal((
                Button::new("@shadcn").tooltip(profile_card),
                Button::new(
                    Stack::horizontal((icon("package", 16.0), "Repository"))
                        .style(|s| s.items_center().gap(6.0)),
                )
                .tooltip(repository_card),
                Button::new(icon("file-code-2", 16.0)).tooltip(file_card),
            ))
            .style(|s| s.items_center().gap(8.0).flex_wrap(FlexWrap::Wrap)),
        ),
        section(
            "Surfaces",
            Stack::horizontal((profile_card(), repository_card(), file_card(), team_card()))
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
