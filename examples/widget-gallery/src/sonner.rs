use floem::{
    AnyView, IntoView, easing,
    icons::{self as icon_library, IconLibrary},
    prelude::*,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

#[derive(Clone, Copy)]
enum ToastKind {
    Default,
    Success,
    Info,
    Warning,
    Error,
    Loading,
}

impl ToastKind {
    fn icon(self) -> &'static str {
        match self {
            Self::Default => "bell",
            Self::Success => "circle-check",
            Self::Info => "info",
            Self::Warning => "triangle-alert",
            Self::Error => "octagon-x",
            Self::Loading => "loader-circle",
        }
    }
}

fn icon(kind: ToastKind) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, kind.icon())
        .map(|icon| {
            let styled = icon.style(move |s| {
                s.size(16.0, 16.0).flex_shrink(0.0).with_theme(move |s, t| {
                    let color = match kind {
                        ToastKind::Success => t.success(),
                        ToastKind::Info => t.primary(),
                        ToastKind::Warning => t.warning(),
                        ToastKind::Error => t.danger(),
                        _ => t.popover_foreground(),
                    };
                    s.color(color)
                })
            });
            if matches!(kind, ToastKind::Loading) {
                styled
                    .animation(|a| {
                        a.duration(1.seconds())
                            .keyframe(0, |f| f.style(|s| s.rotate(0.0.deg())))
                            .keyframe(100, |f| {
                                f.style(|s| s.rotate(360.0.deg())).ease(easing::Linear)
                            })
                            .repeat(true)
                    })
                    .into_any()
            } else {
                styled.into_any()
            }
        })
        .unwrap_or_else(|| Empty::new().style(|s| s.size(16.0, 16.0)).into_any())
}

fn toast(kind: ToastKind, title: &'static str, description: &'static str) -> AnyView {
    Stack::horizontal((
        icon(kind),
        Stack::vertical((
            title.style(|s| {
                s.font_size(14.0)
                    .font_weight(FontWeight::MEDIUM)
                    .line_height(1.35)
                    .with_theme(|s, t| s.color(t.popover_foreground()))
            }),
            description.style(|s| {
                s.font_size(13.0)
                    .line_height(1.35)
                    .with_theme(|s, t| s.color(t.muted_foreground()))
            }),
        ))
        .style(|s| s.flex_col().gap(2.0).flex_grow(1.0)),
        Button::new("Undo").style(|s| s.height(28.0).padding_horiz(8.0).font_size(13.0)),
    ))
    .style(|s| {
        s.width(372.0)
            .min_height(64.0)
            .items_start()
            .gap(10.0)
            .padding(14.0)
            .border(1.0)
            .border_radius(12.0)
            .corner_smoothing(0.6)
            .box_shadow_blur(18.0)
            .box_shadow_color(floem::peniko::Color::from_rgb8(0, 0, 0).with_alpha(0.16))
            .with_theme(|s, t| {
                s.background(t.popover())
                    .color(t.popover_foreground())
                    .border_color(t.border())
            })
    })
    .into_any()
}

pub fn sonner_view() -> impl IntoView {
    Stack::vertical((
        "Sonner".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::vertical((
            toast(
                ToastKind::Default,
                "Event scheduled",
                "Monday, June 30 at 9:00 AM",
            ),
            toast(
                ToastKind::Success,
                "Project published",
                "Your changes are now live.",
            ),
            toast(
                ToastKind::Info,
                "Sync complete",
                "All workspace files are up to date.",
            ),
            toast(
                ToastKind::Warning,
                "Storage almost full",
                "Upgrade or remove unused assets.",
            ),
            toast(
                ToastKind::Error,
                "Upload failed",
                "Check the file and try again.",
            ),
            toast(
                ToastKind::Loading,
                "Generating preview",
                "This may take a few seconds.",
            ),
        ))
        .style(|s| s.flex_col().gap(10.0)),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
