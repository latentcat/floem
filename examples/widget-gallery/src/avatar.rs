use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    prelude::*,
    text::FontWeight,
    theme::StyleThemeExt,
};

use crate::form::{form, form_item};

#[derive(Clone, Copy)]
enum AvatarSize {
    Sm,
    Default,
    Lg,
}

impl AvatarSize {
    fn px(self) -> f64 {
        match self {
            Self::Sm => 24.0,
            Self::Default => 32.0,
            Self::Lg => 40.0,
        }
    }

    fn font_size(self) -> f64 {
        match self {
            Self::Sm => 12.0,
            Self::Default => 14.0,
            Self::Lg => 15.0,
        }
    }

    fn badge_size(self) -> f64 {
        match self {
            Self::Sm => 8.0,
            Self::Default => 10.0,
            Self::Lg => 12.0,
        }
    }
}

fn avatar_base(initials: &'static str, size: AvatarSize) -> AnyView {
    initials
        .style(move |s| {
            let px = size.px();
            s.size(px, px)
                .items_center()
                .justify_center()
                .border_radius(100.0)
                .corner_smoothing(0.6)
                .border(1.0)
                .font_size(size.font_size())
                .font_weight(FontWeight::MEDIUM)
                .selectable(false)
                .with_theme(|s, t| {
                    s.background(t.muted())
                        .color(t.muted_foreground())
                        .border_color(t.border())
                })
        })
        .into_any()
}

fn avatar_badge(size: AvatarSize) -> Empty {
    Empty::new().style(move |s| {
        let px = size.badge_size();
        s.absolute()
            .inset_right(0.0)
            .inset_bottom(0.0)
            .size(px, px)
            .border_radius(100.0)
            .border(2.0)
            .with_theme(|s, t| {
                s.background(t.primary())
                    .border_color(t.background())
                    .color(t.primary_foreground())
            })
    })
}

fn avatar(initials: &'static str, size: AvatarSize, badge: bool) -> AnyView {
    let px = size.px();
    Stack::new((
        avatar_base(initials, size),
        badge
            .then(|| avatar_badge(size).into_any())
            .unwrap_or_else(|| Empty::new().into_any()),
    ))
    .style(move |s| s.size(px, px).flex_shrink(0.0))
    .into_any()
}

fn count_avatar(label: &'static str, size: AvatarSize) -> AnyView {
    let px = size.px();
    label
        .style(move |s| {
            s.size(px, px)
                .items_center()
                .justify_center()
                .border_radius(100.0)
                .corner_smoothing(0.6)
                .border(2.0)
                .font_size(size.font_size())
                .with_theme(|s, t| {
                    s.background(t.muted())
                        .color(t.muted_foreground())
                        .border_color(t.background())
                })
        })
        .into_any()
}

fn icon_avatar(name: &'static str, size: AvatarSize) -> AnyView {
    let px = size.px();
    let icon_size = match size {
        AvatarSize::Sm => 12.0,
        AvatarSize::Default => 16.0,
        AvatarSize::Lg => 20.0,
    };

    let icon = icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| icon.style(move |s| s.size(icon_size, icon_size)).into_any())
        .unwrap_or_else(|| {
            Empty::new()
                .style(move |s| s.size(icon_size, icon_size))
                .into_any()
        });

    Stack::new((icon,))
        .style(move |s| {
            s.size(px, px)
                .items_center()
                .justify_center()
                .border_radius(100.0)
                .corner_smoothing(0.6)
                .with_theme(|s, t| s.background(t.muted()).color(t.muted_foreground()))
        })
        .into_any()
}

pub fn avatar_view() -> impl IntoView {
    form((
        form_item(
            "Sizes:",
            Stack::horizontal((
                avatar("AB", AvatarSize::Sm, false),
                avatar("CD", AvatarSize::Default, false),
                avatar("EF", AvatarSize::Lg, false),
            ))
            .style(|s| s.items_center().gap(10.0)),
        ),
        form_item(
            "Badge:",
            Stack::horizontal((
                avatar("ON", AvatarSize::Sm, true),
                avatar("ON", AvatarSize::Default, true),
                avatar("ON", AvatarSize::Lg, true),
            ))
            .style(|s| s.items_center().gap(10.0)),
        ),
        form_item(
            "Fallbacks:",
            Stack::horizontal((
                avatar("JD", AvatarSize::Default, false),
                avatar("SK", AvatarSize::Default, false),
                icon_avatar("user", AvatarSize::Default),
            ))
            .style(|s| s.items_center().gap(10.0)),
        ),
        form_item(
            "Group:",
            Stack::horizontal((
                avatar("AB", AvatarSize::Default, false),
                avatar("CD", AvatarSize::Default, false).style(|s| s.margin_left(-8.0)),
                avatar("EF", AvatarSize::Default, false).style(|s| s.margin_left(-8.0)),
                count_avatar("+3", AvatarSize::Default).style(|s| s.margin_left(-8.0)),
            ))
            .style(|s| s.items_center()),
        ),
    ))
    .style(|s| s.with_theme(|s, t| s.background(t.background()).color(t.foreground())))
}
