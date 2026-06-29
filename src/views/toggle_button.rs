#![deny(missing_docs)]
//! A toggle button widget. An example can be found in [widget-gallery/button](https://github.com/lapce/floem/tree/main/examples/widget-gallery)
//! in the floem examples.

use std::{cell::RefCell, rc::Rc};

use floem_reactive::{Effect, SignalGet, SignalUpdate};
use peniko::Brush;
use peniko::kurbo::{Point, Rect, Size};

use crate::context::Phases;
use crate::custom_event;
use crate::event::listener::EventListenerTrait;
use crate::{
    BoxTree, ElementId, Renderer,
    action::{TimerToken, exec_after_animation_frame},
    context::{EventCx, PaintCx, UpdateCx},
    event::{Event, EventPropagation, InteractionEvent, Phase, listener::UpdatePhaseLayout},
    platform::{Duration, Instant},
    prop, prop_extractor,
    style::{
        DirectTransition, FontSize, Foreground, LineHeight, Style, StyleSelector, Transition,
        recalc::StyleReason,
    },
    style_class,
    unit::Length,
    view::View,
    view::ViewId,
    views::Decorators,
};

const HANDLE_TRANSITION: Duration = Duration::from_millis(150);

prop!(pub ToggleButtonInset: Length {} = Length::Pt(0.));
prop!(pub ToggleButtonUncheckedInset: Option<Length> {} = None);
prop!(pub ToggleButtonCheckedInset: Option<Length> {} = None);
prop!(pub ToggleButtonCircleRad: Length {} = Length::Pct(95.));

prop_extractor! {
    ToggleStyle {
        foreground: Foreground,
        inset: ToggleButtonInset,
        unchecked_inset: ToggleButtonUncheckedInset,
        checked_inset: ToggleButtonCheckedInset,
        circle_rad: ToggleButtonCircleRad,
        font_size: FontSize,
        line_height: LineHeight,
    }
}

style_class!(
    /// A class for styling [ToggleButton] view.
    pub ToggleButtonClass
);

#[derive(Clone, Copy, Debug)]
/// Event fired when the toggle state changes
pub struct ToggleChanged(bool);
impl ToggleChanged {
    fn extract_inner(&self) -> &bool {
        &self.0
    }
}

custom_event!(ToggleChanged, bool, ToggleChanged::extract_inner);

struct HandleAnimationFrame;

struct Handle {
    element_id: ElementId,
    box_tree: Rc<RefCell<BoxTree>>,
    position: DirectTransition<f64>,
}

impl Handle {
    fn new(parent_id: ViewId) -> Self {
        Self {
            element_id: parent_id.create_child_element_id(1),
            box_tree: parent_id.box_tree(),
            position: DirectTransition::new(0.0, Some(Transition::linear(HANDLE_TRANSITION))),
        }
    }

    fn target_position(
        state: bool,
        width: f64,
        radius: f64,
        unchecked_inset: f64,
        checked_inset: f64,
    ) -> f64 {
        let min = radius + unchecked_inset;
        let max = (width - radius - checked_inset).max(min);
        if state { max } else { min }
    }

    fn update_bounds(&self, size: Size, radius: f64) {
        let position = self.position.get();
        let rect = Rect::new(position - radius, 0., position + radius, size.height);
        let mut bt = self.box_tree.borrow_mut();
        bt.set_local_bounds(self.element_id.0, rect);
    }

    fn snap(
        &mut self,
        state: bool,
        size: Size,
        radius: f64,
        unchecked_inset: f64,
        checked_inset: f64,
    ) {
        let target =
            Self::target_position(state, size.width, radius, unchecked_inset, checked_inset);
        self.position.set_immediate(target);
        self.update_bounds(size, radius);
    }

    fn transition_to_state(
        &mut self,
        state: bool,
        size: Size,
        radius: f64,
        unchecked_inset: f64,
        checked_inset: f64,
    ) -> bool {
        let target =
            Self::target_position(state, size.width, radius, unchecked_inset, checked_inset);
        if (self.position.get() - target).abs() < f64::EPSILON && !self.position.is_active() {
            self.update_bounds(size, radius);
            return false;
        }

        let started = self.position.transition_to(target);
        self.update_bounds(size, radius);
        started || self.position.is_active()
    }

    fn step(&mut self, now: &Instant, size: Size, radius: f64) -> bool {
        let changed = self.position.step(now);
        if changed {
            self.update_bounds(size, radius);
        }
        changed
    }

    fn is_animating(&self) -> bool {
        self.position.is_active()
    }

    fn paint(&self, cx: &mut PaintCx, color: Option<Brush>, size: Size, radius: f64) {
        let circle_point = Point::new(self.position.get(), size.to_rect().center().y);
        let circle = crate::kurbo::Circle::new(circle_point, radius);
        if let Some(color) = color {
            cx.fill(&circle, &color, 0.);
        }
    }
}

/// A toggle button.
pub struct ToggleButton {
    id: ViewId,
    state: bool,
    handle: Handle,
    handle_animation_frame: bool,
    style: ToggleStyle,
}

/// A reactive toggle button.
///
/// When the button is toggled by clicking or dragging the widget, an update will be
/// sent to the [`ToggleButton::on_toggle`] handler.
///
/// By default this toggle button has a style class of [`ToggleButtonClass`] applied
/// with a default style provided.
/// ### Examples
/// ```rust
/// # use floem::reactive::{SignalGet, SignalUpdate, RwSignal};
/// # use floem::views::toggle_button;
/// // An example using read-write signal
/// let state = RwSignal::new(true);
/// let toggle = toggle_button(move || state.get())
///     .on_toggle(move |new_state| state.set(new_state));
/// ```
/// ### Reactivity
/// This function is reactive and will reactively respond to changes.
#[deprecated]
pub fn toggle_button(state: impl Fn() -> bool + 'static) -> ToggleButton {
    ToggleButton::new(state)
}

impl ToggleButton {
    fn length_resolve_cx(&self) -> crate::style::FontSizeCx {
        let font_size = self.style.font_size();
        let line_height = match self.style.line_height() {
            crate::text::LineHeightValue::Pt(value) => f64::from(value),
            crate::text::LineHeightValue::Normal(value) => font_size * f64::from(value),
        };
        crate::style::FontSizeCx::new(font_size, line_height)
    }

    fn circle_radius(&self, size: Size) -> f64 {
        self.style
            .circle_rad()
            .resolve(size.width.min(size.height) / 2.0, &self.length_resolve_cx())
    }

    fn inset(&self, width: f64) -> f64 {
        self.style
            .inset()
            .resolve(width, &self.length_resolve_cx())
            .min(width / 2.0)
    }

    fn resolve_inset(&self, inset: Length, width: f64) -> f64 {
        inset
            .resolve(width, &self.length_resolve_cx())
            .min(width / 2.0)
    }

    fn unchecked_inset(&self, width: f64) -> f64 {
        self.style
            .unchecked_inset()
            .map(|inset| self.resolve_inset(inset, width))
            .unwrap_or_else(|| self.inset(width))
    }

    fn checked_inset(&self, width: f64) -> f64 {
        self.style
            .checked_inset()
            .map(|inset| self.resolve_inset(inset, width))
            .unwrap_or_else(|| self.inset(width))
    }

    fn post_layout(&mut self) {
        self.set_handle_position(false);
    }

    fn handle_geometry(&self) -> (Size, f64, f64, f64) {
        let size = self.id.get_layout_rect_local().size();
        let radius = self.circle_radius(size);
        let unchecked_inset = self.unchecked_inset(size.width);
        let checked_inset = self.checked_inset(size.width);
        (size, radius, unchecked_inset, checked_inset)
    }

    fn set_handle_position(&mut self, animated: bool) {
        let (size, radius, unchecked_inset, checked_inset) = self.handle_geometry();
        if animated && size.width > 0.0 && size.height > 0.0 {
            self.handle.transition_to_state(
                self.state,
                size,
                radius,
                unchecked_inset,
                checked_inset,
            );
            self.request_handle_animation_frame();
        } else {
            self.handle
                .snap(self.state, size, radius, unchecked_inset, checked_inset);
            self.handle_animation_frame = false;
        }
        self.id.request_box_tree_commit();
    }

    fn step_handle_animation(&mut self) {
        self.handle_animation_frame = false;

        let (size, radius, _, _) = self.handle_geometry();
        if self.handle.step(&Instant::now(), size, radius) {
            self.id.request_box_tree_commit();
            self.id.request_paint();
        }

        self.request_handle_animation_frame();
    }

    fn request_handle_animation_frame(&mut self) {
        if self.handle_animation_frame || !self.handle.is_animating() {
            return;
        }

        let id = self.id;
        let token = exec_after_animation_frame(move |_| id.update_state(HandleAnimationFrame));
        self.handle_animation_frame = token != TimerToken::INVALID;
    }

    /// Create new [ToggleButton].
    ///
    /// When the button is toggled by clicking or dragging the widget, an update will be
    /// sent to the [`ToggleButton::on_toggle`] handler.
    ///
    /// By default this toggle button has a style class of [`ToggleButtonClass`] applied
    /// with a default style provided.
    /// ### Examples
    /// ```rust
    /// # use floem::reactive::{SignalGet, SignalUpdate, RwSignal};
    /// # use floem::views::toggle_button;
    /// // An example using read-write signal
    /// let state = RwSignal::new(true);
    /// let toggle = toggle_button(move || state.get())
    ///     .on_toggle(move |new_state| state.set(new_state));
    /// ```
    /// ### Reactivity
    /// This function is reactive and will reactively respond to changes.
    pub fn new(state: impl Fn() -> bool + 'static) -> Self {
        let id = ViewId::new();
        id.register_listener(UpdatePhaseLayout::listener_key());

        Effect::new(move |_| {
            let state = state();
            id.update_state(state);
        });

        Self {
            id,
            state: false,
            handle: Handle::new(id),
            handle_animation_frame: false,
            style: Default::default(),
        }
        .class(ToggleButtonClass)
    }

    /// Create new [ToggleButton] with read-write signal.
    /// ### Examples
    /// ```rust
    /// # use floem::prelude::*;
    /// # use floem::prelude::palette::css;
    /// // Create read-write signal that will hold toggle button state
    /// let state = RwSignal::new(false);
    /// let simple = ToggleButton::new_rw(state);
    /// ```
    /// ### Reactivity
    /// This function will update provided signal on toggle or will be updated if signal changes
    /// due to external signal update.
    pub fn new_rw(state: impl SignalGet<bool> + SignalUpdate<bool> + Copy + 'static) -> Self {
        Self::new(move || state.get())
            .on_event_stop(ToggleChanged::listener(), move |_cx, ns| state.set(*ns))
    }

    /// Add an event handler to be run when the button is toggled.
    ///
    /// This does not run if the state is changed because of an outside signal.
    #[deprecated(note = "use .on_event_stop(ToggleChanged::listener(), |_, _|) directly instead")]
    pub fn on_toggle(self, ontoggle: impl Fn(bool) + 'static) -> Self {
        self.on_event_stop(ToggleChanged::listener(), move |_cx, e| ontoggle(*e))
    }

    /// Set styles related to [ToggleButton]:
    /// - handle color
    /// - accent color
    /// - handle inset
    /// - circle radius
    pub fn toggle_style(
        self,
        style: impl Fn(ToggleButtonCustomStyle) -> ToggleButtonCustomStyle + 'static,
    ) -> Self {
        self.style(move |s| s.apply_custom(style(Default::default())))
    }
}

impl View for ToggleButton {
    fn id(&self) -> ViewId {
        self.id
    }

    fn debug_name(&self) -> std::borrow::Cow<'static, str> {
        "Toggle Button".into()
    }

    fn view_style(&self) -> Option<Style> {
        Some(Style::new().keyboard_navigable().set_selected(self.state))
    }

    fn update(&mut self, _cx: &mut UpdateCx, state: Box<dyn std::any::Any>) {
        if state.is::<HandleAnimationFrame>() {
            self.step_handle_animation();
            return;
        }

        if let Ok(state) = state.downcast::<bool>() {
            let state = *state;
            if self.state == state {
                return;
            }

            self.state = state;
            self.set_handle_position(true);
            let mut reason = StyleReason::view_style();
            reason.merge(StyleReason::with_selector(StyleSelector::Selected));
            self.id.request_style(reason);
            self.id.request_paint();
        }
    }

    fn event(&mut self, cx: &mut EventCx) -> EventPropagation {
        if UpdatePhaseLayout::extract(&cx.event).is_some() {
            self.post_layout();
            return EventPropagation::Stop;
        }

        if cx.phase != Phase::Target {
            return EventPropagation::Continue;
        }

        if self.id.is_disabled() {
            return EventPropagation::Stop;
        }

        if let Event::Interaction(InteractionEvent::Click) = &cx.event {
            self.state = !self.state;
            self.set_handle_position(true);
            let mut reason = StyleReason::view_style();
            reason.merge(StyleReason::with_selector(StyleSelector::Selected));
            self.id.request_style(reason);
            self.id.request_paint();
            self.id.route_event_with_caused_by(
                Event::new_custom(ToggleChanged(self.state)),
                crate::event::RouteKind::Directed {
                    target: self.id.get_element_id(),
                    phases: Phases::TARGET,
                },
                Some(cx.event.clone()),
            );
            return EventPropagation::Stop;
        }

        EventPropagation::Continue
    }

    fn style_pass(&mut self, cx: &mut crate::context::StyleCx<'_>) {
        if self.style.read(cx) {
            self.set_handle_position(false);
            cx.window_state.request_paint(self.id);
        }
    }

    fn paint(&mut self, cx: &mut PaintCx) {
        if cx.target_id == self.handle.element_id {
            let size = self.id.get_layout_rect_local().size();
            let radius = self.circle_radius(size);
            self.handle.paint(cx, self.style.foreground(), size, radius);
        }
    }
}

/// Represents a custom style for a [ToggleButton].
#[derive(Debug, Default, Clone)]
pub struct ToggleButtonCustomStyle(Style);
impl From<ToggleButtonCustomStyle> for Style {
    fn from(value: ToggleButtonCustomStyle) -> Self {
        value.0
    }
}

impl ToggleButtonCustomStyle {
    /// Create new styles for [ToggleButton].
    pub fn new() -> Self {
        Self(Style::new())
    }

    /// Sets the color of the toggle handle.
    pub fn handle_color(mut self, color: impl Into<Brush>) -> Self {
        self = Self(self.0.set(Foreground, Some(color.into())));
        self
    }

    /// Sets the accent color of the toggle button (same as background color).
    pub fn accent_color(mut self, color: impl Into<Brush>) -> Self {
        self = Self(self.0.background(color));
        self
    }

    /// Sets the inset of the toggle handle from the edge.
    pub fn handle_inset(mut self, inset: impl Into<Length>) -> Self {
        self = Self(self.0.set(ToggleButtonInset, inset));
        self
    }

    /// Sets the inset of the toggle handle from the start edge when unchecked.
    pub fn unchecked_handle_inset(mut self, inset: impl Into<Length>) -> Self {
        self = Self(self.0.set(ToggleButtonUncheckedInset, Some(inset.into())));
        self
    }

    /// Sets the inset of the toggle handle from the end edge when checked.
    pub fn checked_handle_inset(mut self, inset: impl Into<Length>) -> Self {
        self = Self(self.0.set(ToggleButtonCheckedInset, Some(inset.into())));
        self
    }

    /// Sets the radius of the toggle circle.
    pub fn circle_rad(mut self, rad: impl Into<Length>) -> Self {
        self = Self(self.0.set(ToggleButtonCircleRad, rad));
        self
    }

    /// Sets the styles of the toggle button if `cond` is `true`.
    pub fn apply_if(self, cond: bool, f: impl FnOnce(Self) -> Self) -> Self {
        if cond { f(self) } else { self }
    }
}
