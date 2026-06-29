use std::rc::Rc;

use floem::{
    AnyView, IntoView,
    action::{TimerToken, exec_after, exec_after_animation_frame},
    context::VisualChangedListener,
    kurbo::Rect,
    peniko::Color,
    prelude::*,
    reactive::Effect,
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
                    active.set(true);
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
    Transition::linear(duration)
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
    let duration = 180.millis();
    let motion = PortalMotion::new(open, duration);
    let content = Rc::new(content);

    Overlay::new_dyn(move || {
        if !motion.present.get() {
            return Empty::new().into_any();
        }

        let active = motion.is_active();
        let backdrop = Empty::new()
            .on_event_stop(listener::Click, move |_, _| open.set(false))
            .style(move |s| {
                s.absolute()
                    .inset(0.0)
                    .opacity(if active { 1.0 } else { 0.0 })
                    .backdrop_blur(if active { 8.0 } else { 0.0 })
                    .background(Color::from_rgb8(0, 0, 0).with_alpha(if active {
                        0.42
                    } else {
                        0.0
                    }))
                    .transition(Opacity, transition(duration))
                    .transition(BackdropBlur, transition(duration))
                    .transition(Background, transition(duration))
            });

        let surface = Stack::vertical(((content)(),)).style(move |s| {
            let s = s
                .absolute()
                .inset(0.0)
                .items_center()
                .justify_center()
                .padding(24.0);
            portal_surface_motion(s, motion, duration)
        });

        Stack::new((backdrop, surface))
            .style(|s| s.absolute().inset(0.0).z_index(40))
            .into_any()
    })
    .style(|s| s.absolute().inset(0.0).z_index(40))
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
    .style(|s| s.absolute().inset(0.0).z_index(30));

    Stack::new((trigger, overlay)).into_any()
}
