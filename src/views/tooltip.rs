#![deny(missing_docs)]
use floem_reactive::{RwSignal, SignalGet, SignalUpdate};
use peniko::kurbo::{Rect, Size};
use std::cell::RefCell;
use std::rc::Rc;
use ui_events::pointer::PointerEvent;

use crate::{
    action::{TimerToken, add_overlay, exec_after, exec_after_animation_frame, remove_overlay},
    context::{EventCx, UpdateCx, VisualChangedListener},
    event::{Event, EventPropagation, Phase},
    platform::Duration,
    prop, prop_extractor, style_class,
    theme::StyleThemeExt,
    unit::UnitExt,
    view::{IntoView, View, ViewId},
    views::{Decorators, Empty, Stack},
};

style_class!(
    /// A class for the tooltip views.
    pub TooltipClass
);
style_class!(
    /// A class for the tooltip container view.
    pub TooltipContainerClass
);

prop!(pub Delay: Duration {} = Duration::from_millis(600));

prop_extractor! {
    TooltipStyle {
        delay: Delay,
    }
}

/// A view that displays a tooltip for its child.
pub struct Tooltip {
    id: ViewId,
    /// Timer token used to delay showing the tooltip.
    hover_token: Option<TimerToken>,
    /// Tooltip overlay view id.
    overlay: Rc<RefCell<Option<ViewId>>>,
    /// Provided by user function that dislays tooltip content.
    tip: Rc<dyn Fn() -> Box<dyn View>>,
    /// A tooltip specific styles (currently its just a delay).
    style: TooltipStyle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum TooltipSide {
    Top,
    Bottom,
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct TooltipPlacement {
    left: f64,
    top: f64,
    arrow_left: f64,
    side: TooltipSide,
}

impl TooltipPlacement {
    fn initial(anchor: Rect) -> Self {
        Self {
            left: anchor.x0,
            top: anchor.y1 + TOOLTIP_SIDE_OFFSET,
            arrow_left: TOOLTIP_ARROW_SIZE,
            side: TooltipSide::Bottom,
        }
    }

    fn nearly_eq(self, other: Self) -> bool {
        self.side == other.side
            && (self.left - other.left).abs() < 0.5
            && (self.top - other.top).abs() < 0.5
            && (self.arrow_left - other.arrow_left).abs() < 0.5
    }
}

const TOOLTIP_VIEWPORT_PADDING: f64 = 8.0;
const TOOLTIP_SIDE_OFFSET: f64 = 8.0;
const TOOLTIP_ARROW_SIZE: f64 = 10.0;
const TOOLTIP_ARROW_EDGE_PADDING: f64 = 8.0;

fn clamp_to_axis(value: f64, min: f64, max: f64) -> f64 {
    if max <= min {
        min
    } else {
        value.clamp(min, max)
    }
}

fn compute_tooltip_placement(anchor: Rect, surface_size: Size, viewport: Size) -> TooltipPlacement {
    let surface_width = surface_size.width.max(1.0);
    let surface_height = surface_size.height.max(1.0);
    let viewport_width = viewport.width.max(1.0);
    let viewport_height = viewport.height.max(1.0);
    let anchor_center_x = (anchor.x0 + anchor.x1) * 0.5;

    let max_left =
        (viewport_width - surface_width - TOOLTIP_VIEWPORT_PADDING).max(TOOLTIP_VIEWPORT_PADDING);
    let left = clamp_to_axis(
        anchor_center_x - surface_width * 0.5,
        TOOLTIP_VIEWPORT_PADDING,
        max_left,
    );

    let below_top = anchor.y1 + TOOLTIP_SIDE_OFFSET;
    let above_top = anchor.y0 - surface_height - TOOLTIP_SIDE_OFFSET;
    let fits_below = below_top + surface_height <= viewport_height - TOOLTIP_VIEWPORT_PADDING;
    let above_space = anchor.y0 - TOOLTIP_VIEWPORT_PADDING;
    let below_space = viewport_height - anchor.y1 - TOOLTIP_VIEWPORT_PADDING;
    let side = if fits_below || below_space >= above_space {
        TooltipSide::Bottom
    } else {
        TooltipSide::Top
    };

    let ideal_top = match side {
        TooltipSide::Top => above_top,
        TooltipSide::Bottom => below_top,
    };
    let max_top =
        (viewport_height - surface_height - TOOLTIP_VIEWPORT_PADDING).max(TOOLTIP_VIEWPORT_PADDING);
    let top = clamp_to_axis(ideal_top, TOOLTIP_VIEWPORT_PADDING, max_top);

    let min_arrow_left = TOOLTIP_ARROW_EDGE_PADDING;
    let max_arrow_left =
        (surface_width - TOOLTIP_ARROW_SIZE - TOOLTIP_ARROW_EDGE_PADDING).max(min_arrow_left);
    let arrow_left = clamp_to_axis(
        anchor_center_x - left - TOOLTIP_ARROW_SIZE * 0.5,
        min_arrow_left,
        max_arrow_left,
    );

    TooltipPlacement {
        left,
        top,
        arrow_left,
        side,
    }
}

fn refresh_tooltip_placement(
    anchor: RwSignal<Rect>,
    surface_size: RwSignal<Size>,
    viewport: RwSignal<Size>,
    placement: RwSignal<TooltipPlacement>,
) {
    let next = compute_tooltip_placement(
        anchor.get_untracked(),
        surface_size.get_untracked(),
        viewport.get_untracked(),
    );
    if !placement.get_untracked().nearly_eq(next) {
        placement.set(next);
    }
}

/// A view that displays a tooltip for its child.
pub fn tooltip<V: IntoView + 'static, T: IntoView + 'static>(
    child: V,
    tip: impl Fn() -> T + 'static,
) -> Tooltip {
    let id = ViewId::new();
    let child = child.into_view();
    id.set_children([child]);
    let overlay = Rc::new(RefCell::new(None));
    Tooltip {
        id,
        tip: Rc::new(move || tip().into_any()),
        hover_token: None,
        overlay: overlay.clone(),
        style: Default::default(),
    }
    .class(TooltipContainerClass)
    .on_cleanup(move || {
        if let Some(overlay_id) = overlay.borrow_mut().take() {
            remove_overlay(overlay_id);
        }
    })
}

impl View for Tooltip {
    fn id(&self) -> ViewId {
        self.id
    }

    fn update(&mut self, _cx: &mut UpdateCx, state: Box<dyn std::any::Any>) {
        if let Ok(token) = state.downcast::<TimerToken>()
            && self.hover_token == Some(*token)
        {
            self.show_tooltip();
        }
    }

    fn style_pass(&mut self, cx: &mut crate::context::StyleCx<'_>) {
        self.style.read(cx);
        if self.overlay.borrow().is_some() && self.id.is_hidden() {
            let id = self.overlay.take().unwrap();
            self.hover_token = None;
            remove_overlay(id);
        }
    }

    fn event_capture(&mut self, cx: &mut EventCx) -> EventPropagation {
        self.handle_event(cx)
    }

    fn event(&mut self, cx: &mut EventCx) -> EventPropagation {
        if cx.phase != Phase::Target {
            return EventPropagation::Continue;
        }
        self.handle_event(cx)
    }
}

impl Tooltip {
    fn hide_tooltip(&mut self) {
        if let Some(token) = self.hover_token.take() {
            token.cancel();
        }
        if let Some(id) = self.overlay.borrow_mut().take() {
            remove_overlay(id);
        }
    }

    fn show_tooltip(&mut self) {
        if self.overlay.borrow().is_some() {
            return;
        }

        let anchor = self.id.get_visual_rect();
        let viewport = self.id.root().get_size().unwrap_or_default();
        if anchor.width() <= 0.0 || anchor.height() <= 0.0 || viewport.width <= 0.0 {
            return;
        }

        let anchor_signal = RwSignal::new(anchor);
        let viewport_signal = RwSignal::new(viewport);
        let surface_size = RwSignal::new(Size::ZERO);
        let placement = RwSignal::new(TooltipPlacement::initial(anchor));
        let anchor_id = self.id;

        let top_arrow = Empty::new().style(move |s| {
            let placement = placement.get();
            s.size(TOOLTIP_ARROW_SIZE, TOOLTIP_ARROW_SIZE)
                .margin_left(placement.arrow_left)
                .margin_bottom(-TOOLTIP_ARROW_SIZE * 0.5)
                .rotate(45.0.deg())
                .border_radius(2.0)
                .apply_if(placement.side != TooltipSide::Bottom, |s| s.hide())
                .with_theme(|s, t| s.background(t.foreground()))
        });

        let bottom_arrow = Empty::new().style(move |s| {
            let placement = placement.get();
            s.size(TOOLTIP_ARROW_SIZE, TOOLTIP_ARROW_SIZE)
                .margin_left(placement.arrow_left)
                .margin_top(-TOOLTIP_ARROW_SIZE * 0.5)
                .rotate(45.0.deg())
                .border_radius(2.0)
                .apply_if(placement.side != TooltipSide::Top, |s| s.hide())
                .with_theme(|s, t| s.background(t.foreground()))
        });

        let content = (self.tip)().class(TooltipClass);
        let surface = Stack::vertical((top_arrow, content, bottom_arrow))
            .on_event_stop(VisualChangedListener, move |_, visual| {
                let size = visual.new_visual_aabb.size();
                if surface_size.get_untracked() != size {
                    surface_size.set(size);
                    refresh_tooltip_placement(
                        anchor_signal,
                        surface_size,
                        viewport_signal,
                        placement,
                    );
                }
            })
            .on_event_stop(listener::WindowResized, move |_, size| {
                viewport_signal.set(*size);
                anchor_signal.set(anchor_id.get_visual_rect());
                refresh_tooltip_placement(anchor_signal, surface_size, viewport_signal, placement);
            })
            .style(move |s| {
                let placement = placement.get();
                s.inset_left(placement.left)
                    .inset_top(placement.top)
                    .items_start()
                    .gap(0.0)
            });

        let overlay_id = add_overlay(surface);
        overlay_id.set_style_parent(self.id);
        exec_after_animation_frame(move |_| {
            if overlay_id.try_root().is_some() {
                surface_size.set(overlay_id.get_visual_rect_no_clip().size());
                anchor_signal.set(anchor_id.get_visual_rect());
                refresh_tooltip_placement(anchor_signal, surface_size, viewport_signal, placement);
            }
        });
        *self.overlay.borrow_mut() = Some(overlay_id);
    }

    fn start_hover_timer(&mut self) {
        if self.overlay.borrow().is_some() || self.hover_token.is_some() {
            return;
        }

        let id = self.id();
        let token = exec_after(self.style.delay(), move |token| {
            id.update_state(token);
        });
        self.hover_token = Some(token);
    }

    fn handle_event(&mut self, cx: &mut EventCx) -> EventPropagation {
        match &cx.event {
            Event::Pointer(PointerEvent::Enter(_)) | Event::Pointer(PointerEvent::Move(_))
                if self.overlay.borrow().is_none() =>
            {
                self.start_hover_timer();
            }
            Event::Pointer(PointerEvent::Leave(_))
            | Event::Pointer(PointerEvent::Cancel(_))
            | Event::Pointer(PointerEvent::Down(_))
            | Event::Pointer(PointerEvent::Scroll(_))
            | Event::Key(_) => {
                self.hide_tooltip();
            }
            _ => {}
        }
        EventPropagation::Continue
    }
}

/// Adds a [tooltip] function to a type that implements [`IntoView`].
pub trait TooltipExt {
    /// Adds a tooltip to the view.
    ///
    /// ### Examples
    /// ```rust
    /// # use floem::views::TooltipExt;
    /// # use floem::views::{text, Decorators};
    /// # use floem::prelude::{RwSignal, SignalGet};
    /// // Simple usage:
    /// let simple = text("A text with tooltip")
    ///     .tooltip(|| "This is a tooltip.");
    /// // More complex usage:
    /// let mut click_counter = RwSignal::new(0);
    /// let complex = text("A text with a tooltip that changes on click")
    ///     .on_click_stop(move|_| click_counter += 1)
    ///     .tooltip(move || format!("Clicked {} times on the label", click_counter.get()));
    /// ```
    /// ### Reactivity
    /// This function is not reactive, but it is computing its result on every tooltip trigger.
    /// It is possible then to have different tooltip output, but the output it will **not** change
    /// once while displaying a hover.
    fn tooltip<V: IntoView + 'static>(self, tip: impl Fn() -> V + 'static) -> Tooltip;
}

impl<T: IntoView + 'static> TooltipExt for T {
    fn tooltip<V: IntoView + 'static>(self, tip: impl Fn() -> V + 'static) -> Tooltip {
        tooltip(self, tip)
    }
}
