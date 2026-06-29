use std::{cell::RefCell, rc::Rc};

use floem::{
    AnyView, HasViewId, IntoView, ViewId,
    action::{TimerToken, add_overlay, exec_after, exec_after_animation_frame, remove_overlay},
    context::VisualChangedListener,
    kurbo::Rect,
    peniko::Color,
    prelude::*,
    reactive::{Effect, Scope},
    style::{BackdropBlur, Background, Opacity, ScaleX, ScaleY, Style, Transition, TranslateY},
    unit::UnitExt,
    views::{Decorators, Empty, Overlay},
};
use std::time::Duration;

#[derive(Clone, Copy)]
pub struct PortalMotion {
    pub present: RwSignal<bool>,
    pub active: RwSignal<bool>,
}

impl PortalMotion {
    pub fn new(open: RwSignal<bool>, duration: Duration) -> Self {
        let present = RwSignal::new(open.get_untracked());
        let active = RwSignal::new(false);

        Effect::new(move |_| {
            if open.get() {
                present.set(true);
                active.set(false);
                let token = exec_after_animation_frame(move |_| {
                    let token = exec_after_animation_frame(move |_| {
                        active.set(true);
                    });
                    if token == TimerToken::INVALID {
                        active.set(true);
                    }
                });
                if token == TimerToken::INVALID {
                    active.set(true);
                }
            } else {
                active.set(false);
                exec_after(duration, move |_| {
                    if !open.get_untracked() {
                        present.set(false);
                    }
                });
            }
        });

        Self { present, active }
    }

    pub fn is_active(self) -> bool {
        self.active.get()
    }
}

#[derive(Clone, Copy)]
pub enum PortalSide {
    Bottom,
    Right,
}

#[derive(Clone, Copy)]
pub struct PortalPosition {
    pub side: PortalSide,
    pub side_offset: f64,
    pub align_offset: f64,
}

impl PortalPosition {
    pub fn bottom_start(side_offset: f64) -> Self {
        Self {
            side: PortalSide::Bottom,
            side_offset,
            align_offset: 0.0,
        }
    }

    pub fn right_start(side_offset: f64) -> Self {
        Self {
            side: PortalSide::Right,
            side_offset,
            align_offset: 0.0,
        }
    }
}

fn transition(duration: Duration) -> Transition {
    Transition::ease_in_out(duration)
}

pub fn portal_surface_motion(s: Style, motion: PortalMotion, duration: Duration) -> Style {
    let active = motion.is_active();
    s.opacity(if active { 1.0 } else { 0.0 })
        .scale(if active { 100.pct() } else { 96.pct() })
        .translate_y(if active { 0.0 } else { -4.0 })
        .transition(Opacity, transition(duration))
        .transition(ScaleX, transition(duration))
        .transition(ScaleY, transition(duration))
        .transition(TranslateY, transition(duration))
}

pub fn modal_portal(open: RwSignal<bool>, content: impl Fn() -> AnyView + 'static) -> AnyView {
    let surface_duration = 160.millis();
    let backdrop_duration = 180.millis();
    let motion = PortalMotion::new(open, backdrop_duration);
    let content = Rc::new(content);
    let host = Empty::new();
    let host_id = host.id();
    let overlay = Rc::new(RefCell::new(None::<(ViewId, Scope)>));
    let build_content = Scope::current().enter_child(move |_| (content)());

    Effect::new({
        let overlay = overlay.clone();
        move |_| {
            if open.get() {
                if overlay.borrow().is_some() {
                    return;
                }

                let (content_view, content_scope) = build_content(());
                let backdrop = Empty::new()
                    .on_event_stop(listener::Click, move |_, _| open.set(false))
                    .style(move |s| {
                        let active = motion.is_active();
                        s.absolute()
                            .inset(0.0)
                            .size_full()
                            .backdrop_blur(if active { 16.0 } else { 0.0 })
                            .background(Color::from_rgb8(0, 0, 0).with_alpha(if active {
                                0.38
                            } else {
                                0.0
                            }))
                            .transition(BackdropBlur, transition(backdrop_duration))
                            .transition(Background, transition(backdrop_duration))
                    });

                let surface = Stack::vertical((content_view,))
                    .on_event_stop(listener::PointerDown, |_, _| {})
                    .style(move |s| {
                        let s = s.pointer_events_auto();
                        portal_surface_motion(s, motion, surface_duration)
                    });

                let surface_layer = Stack::vertical((surface,)).style(|s| {
                    s.absolute()
                        .inset(0.0)
                        .size_full()
                        .items_center()
                        .justify_center()
                        .padding(24.0)
                        .pointer_events_none()
                });

                let overlay_view = Stack::new((backdrop, surface_layer)).style(|s| {
                    s.absolute()
                        .inset(0.0)
                        .size_full()
                        .z_index(40)
                        .pointer_events_auto()
                });

                let overlay_id = add_overlay(overlay_view);
                overlay_id.set_style_parent(host_id);
                *overlay.borrow_mut() = Some((overlay_id, content_scope));
            } else if overlay.borrow().is_some() {
                let overlay = overlay.clone();
                exec_after(backdrop_duration, move |_| {
                    if !open.get_untracked()
                        && let Some((overlay_id, content_scope)) = overlay.borrow_mut().take()
                    {
                        remove_overlay(overlay_id);
                        content_scope.dispose();
                    }
                });
            }
        }
    });

    host.on_cleanup(move || {
        if let Some((overlay_id, content_scope)) = overlay.borrow_mut().take() {
            remove_overlay(overlay_id);
            content_scope.dispose();
        }
    })
    .into_any()
}

pub fn anchored_portal(
    trigger: impl IntoView + 'static,
    open: RwSignal<bool>,
    position: PortalPosition,
    content: impl Fn() -> AnyView + 'static,
) -> AnyView {
    let duration = 120.millis();
    let motion = PortalMotion::new(open, duration);
    let anchor = RwSignal::new(Rect::ZERO);
    let content = Rc::new(content);

    let trigger = trigger.into_intermediate();
    let trigger_id = trigger.view_id();

    Effect::new(move |_| {
        if open.get() {
            let rect = trigger_id.get_visual_rect();
            if rect.width() > 0.0 || rect.height() > 0.0 {
                anchor.set(rect);
            }

            let token = exec_after_animation_frame(move |_| {
                if trigger_id.is_valid() {
                    let rect = trigger_id.get_visual_rect();
                    if rect.width() > 0.0 || rect.height() > 0.0 {
                        anchor.set(rect);
                    }
                }
            });
            if token == TimerToken::INVALID {
                let rect = trigger_id.get_visual_rect();
                if rect.width() > 0.0 || rect.height() > 0.0 {
                    anchor.set(rect);
                }
            }
        }
    });

    let trigger = trigger
        .on_event_stop(VisualChangedListener, move |_, visual| {
            anchor.set(visual.new_visual_aabb);
        })
        .into_any();

    let overlay = Overlay::new_dyn(move || {
        if !motion.present.get() {
            return Empty::new().into_any();
        }

        let rect = anchor.get();
        let (left, top) = match position.side {
            PortalSide::Bottom => (
                rect.x0 + position.align_offset,
                rect.y1 + position.side_offset,
            ),
            PortalSide::Right => (
                rect.x1 + position.side_offset,
                rect.y0 + position.align_offset,
            ),
        };

        let dismiss_layer = Empty::new()
            .on_event_stop(listener::Click, move |_, _| open.set(false))
            .style(|s| {
                s.absolute()
                    .inset(0.0)
                    .background(Color::from_rgb8(0, 0, 0).with_alpha(0.0))
            });

        let surface = Stack::vertical(((content)(),))
            .on_event_stop(listener::PointerDown, |_, _| {})
            .style(move |s| {
                let s = s.absolute().inset_left(left).inset_top(top);
                portal_surface_motion(s, motion, duration)
            });

        Stack::new((dismiss_layer, surface))
            .style(|s| s.absolute().inset(0.0).z_index(30))
            .into_any()
    })
    .style(move |s| {
        s.absolute()
            .inset(0.0)
            .z_index(30)
            .apply_if(!motion.present.get(), |s| s.pointer_events_none())
    });

    Stack::new((trigger, overlay)).into_any()
}
