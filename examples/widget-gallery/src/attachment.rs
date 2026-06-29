use floem::{
    AnyView, IntoView,
    animate::Animation,
    easing,
    icons::{self as icon_library, IconLibrary},
    prelude::*,
    style::Opacity,
    taffy::FlexWrap,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

#[derive(Clone, Copy)]
enum AttachmentState {
    Idle,
    Uploading,
    Processing,
    Error,
    Done,
}

#[derive(Clone, Copy)]
enum AttachmentSize {
    Default,
    Sm,
    Xs,
}

#[derive(Clone, Copy)]
enum AttachmentOrientation {
    Horizontal,
    Vertical,
}

impl AttachmentSize {
    fn text_size(self) -> f64 {
        match self {
            Self::Default => 14.0,
            Self::Sm | Self::Xs => 12.0,
        }
    }

    fn radius(self) -> f64 {
        match self {
            Self::Xs => 8.0,
            _ => 12.0,
        }
    }

    fn media(self) -> f64 {
        match self {
            Self::Default => 40.0,
            Self::Sm => 32.0,
            Self::Xs => 28.0,
        }
    }

    fn padding(self) -> f64 {
        match self {
            Self::Default => 8.0,
            Self::Sm => 6.0,
            Self::Xs => 4.0,
        }
    }
}

fn icon(name: &'static str, size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

fn state_icon(state: AttachmentState) -> AnyView {
    let name = match state {
        AttachmentState::Idle => "plus",
        AttachmentState::Uploading | AttachmentState::Processing => "loader-circle",
        AttachmentState::Error => "triangle-alert",
        AttachmentState::Done => "file-text",
    };

    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            let styled = icon.style(move |s| {
                s.size(16.0, 16.0).flex_shrink(0.0).with_theme(move |s, t| {
                    let color = match state {
                        AttachmentState::Error => t.danger(),
                        _ => t.foreground(),
                    };
                    s.color(color)
                })
            });

            if matches!(
                state,
                AttachmentState::Uploading | AttachmentState::Processing
            ) {
                let spin = RwSignal::new(
                    Animation::new()
                        .duration(1.seconds())
                        .keyframe(0, |f| f.style(|s| s.rotate(0.0.deg())).ease(easing::Linear))
                        .keyframe(25, |f| {
                            f.style(|s| s.rotate(90.0.deg())).ease(easing::Linear)
                        })
                        .keyframe(50, |f| {
                            f.style(|s| s.rotate(180.0.deg())).ease(easing::Linear)
                        })
                        .keyframe(75, |f| {
                            f.style(|s| s.rotate(270.0.deg())).ease(easing::Linear)
                        })
                        .keyframe(100, |f| {
                            f.style(|s| s.rotate(360.0.deg())).ease(easing::Linear)
                        })
                        .repeat(true),
                );
                styled.animation(move |_| spin.get()).into_any()
            } else {
                styled.into_any()
            }
        })
        .unwrap_or_else(|| Empty::new().style(|s| s.size(16.0, 16.0)).into_any())
}

fn description(state: AttachmentState, bytes: &'static str) -> &'static str {
    match state {
        AttachmentState::Idle => "Drop to upload",
        AttachmentState::Uploading => "Uploading 64%",
        AttachmentState::Processing => "Processing",
        AttachmentState::Error => "Upload failed",
        AttachmentState::Done => bytes,
    }
}

fn attachment(
    title: &'static str,
    bytes: &'static str,
    state: AttachmentState,
    size: AttachmentSize,
    orientation: AttachmentOrientation,
) -> AnyView {
    let media_size = size.media();
    let text_size = size.text_size();
    let vertical = matches!(orientation, AttachmentOrientation::Vertical);

    let content = Stack::vertical((
        title.style(move |s| {
            s.font_size(text_size)
                .font_weight(FontWeight::MEDIUM)
                .line_height(1.25)
                .text_ellipsis()
                .with_theme(|s, t| s.color(t.card_foreground()))
                .apply_if(
                    matches!(
                        state,
                        AttachmentState::Uploading | AttachmentState::Processing
                    ),
                    |s| s.set(Opacity, 0.72),
                )
        }),
        description(state, bytes).style(move |s| {
            s.font_size(12.0)
                .line_height(1.25)
                .text_ellipsis()
                .with_theme(move |s, t| {
                    if matches!(state, AttachmentState::Error) {
                        s.color(t.danger())
                    } else {
                        s.color(t.muted_foreground())
                    }
                })
        }),
    ))
    .style(move |s| {
        s.flex_col()
            .gap(2.0)
            .min_width(0.0)
            .apply_if(vertical, |s| s.width_full())
            .apply_if(!vertical, |s| s.flex_grow(1.0))
    });

    let media = Stack::vertical((state_icon(state),)).style(move |s| {
        s.size(media_size, media_size)
            .flex_shrink(0.0)
            .items_center()
            .justify_center()
            .border_radius(if matches!(size, AttachmentSize::Xs) {
                6.0
            } else {
                8.0
            })
            .corner_smoothing(0.6)
            .with_theme(move |s, t| {
                if matches!(state, AttachmentState::Error) {
                    s.background(
                        t.def(|t| t.danger().with_alpha(if t.is_dark { 0.2 } else { 0.1 })),
                    )
                } else {
                    s.background(t.muted())
                }
            })
    });

    Stack::horizontal((
        media,
        content,
        Stack::horizontal((
            Button::new(icon("download", 14.0)).style(|s| s.size(28.0, 28.0).padding(0.0)),
            Button::new(icon("x", 14.0)).style(|s| s.size(28.0, 28.0).padding(0.0)),
        ))
        .style(|s| s.items_center().gap(2.0)),
    ))
    .style(move |s| {
        s.width(if vertical { 128.0 } else { 320.0 })
            .min_width(if vertical { 128.0 } else { 160.0 })
            .items_center()
            .gap(if matches!(size, AttachmentSize::Xs) {
                6.0
            } else {
                8.0
            })
            .padding(size.padding())
            .border(1.0)
            .border_radius(size.radius())
            .corner_smoothing(0.6)
            .apply_if(vertical, |s| s.flex_col().items_stretch())
            .with_theme(move |s, t| {
                let border = if matches!(state, AttachmentState::Error) {
                    t.def(|t| t.danger().with_alpha(0.3))
                } else {
                    t.border()
                };
                s.background(t.card())
                    .color(t.card_foreground())
                    .border_color(border)
                    .hover(|s| s.background(t.def(|t| t.muted().with_alpha(0.5))))
            })
            .apply_if(matches!(state, AttachmentState::Idle), |s| s.border(1.0))
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

pub fn attachment_view() -> impl IntoView {
    Stack::vertical((
        "Attachment".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        section(
            "States",
            Stack::horizontal((
                attachment(
                    "proposal.pdf",
                    "2.4 MB",
                    AttachmentState::Done,
                    AttachmentSize::Default,
                    AttachmentOrientation::Horizontal,
                ),
                attachment(
                    "recording.mov",
                    "18.2 MB",
                    AttachmentState::Uploading,
                    AttachmentSize::Default,
                    AttachmentOrientation::Horizontal,
                ),
                attachment(
                    "archive.zip",
                    "42 MB",
                    AttachmentState::Error,
                    AttachmentSize::Default,
                    AttachmentOrientation::Horizontal,
                ),
            ))
            .style(|s| s.gap(12.0).flex_wrap(FlexWrap::Wrap)),
        ),
        section(
            "Sizes",
            Stack::horizontal((
                attachment(
                    "screenshot.png",
                    "812 KB",
                    AttachmentState::Done,
                    AttachmentSize::Default,
                    AttachmentOrientation::Horizontal,
                ),
                attachment(
                    "notes.md",
                    "8 KB",
                    AttachmentState::Processing,
                    AttachmentSize::Sm,
                    AttachmentOrientation::Horizontal,
                ),
                attachment(
                    "Upload",
                    "",
                    AttachmentState::Idle,
                    AttachmentSize::Xs,
                    AttachmentOrientation::Horizontal,
                ),
            ))
            .style(|s| s.gap(12.0).items_center().flex_wrap(FlexWrap::Wrap)),
        ),
        section(
            "Group",
            Stack::horizontal((
                attachment(
                    "image.png",
                    "320 KB",
                    AttachmentState::Done,
                    AttachmentSize::Default,
                    AttachmentOrientation::Vertical,
                ),
                attachment(
                    "brief.pdf",
                    "1.8 MB",
                    AttachmentState::Done,
                    AttachmentSize::Default,
                    AttachmentOrientation::Vertical,
                ),
                attachment(
                    "dataset.csv",
                    "Processing",
                    AttachmentState::Processing,
                    AttachmentSize::Default,
                    AttachmentOrientation::Vertical,
                ),
            ))
            .style(|s| s.gap(12.0).padding_vert(4.0).flex_wrap(FlexWrap::Wrap)),
        ),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
