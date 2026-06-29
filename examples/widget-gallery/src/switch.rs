use floem::{
    IntoView,
    prelude::*,
    views::{Decorators, ToggleButton},
};

use crate::form::{form, form_item};

fn small_switch(state: RwSignal<bool>) -> ToggleButton {
    ToggleButton::new_rw(state)
        .style(|s| s.width(24.0).height(14.0).corner_smoothing(0.6))
        .toggle_style(|s| s.circle_rad(6.0).handle_inset(1.0))
}

pub fn switch_view() -> impl IntoView {
    let checked = RwSignal::new(true);
    let unchecked = RwSignal::new(false);
    let small_checked = RwSignal::new(true);
    let small_unchecked = RwSignal::new(false);
    let disabled_checked = RwSignal::new(true);
    let disabled_unchecked = RwSignal::new(false);

    form((
        form_item("Default checked:", ToggleButton::new_rw(checked)),
        form_item("Default unchecked:", ToggleButton::new_rw(unchecked)),
        form_item("Small checked:", small_switch(small_checked)),
        form_item("Small unchecked:", small_switch(small_unchecked)),
        form_item(
            "Disabled checked:",
            ToggleButton::new(move || disabled_checked.get())
                .on_event_stop(ToggleChanged::listener(), move |_, value| {
                    disabled_checked.set(*value);
                })
                .style(|s| s.set_disabled(true)),
        ),
        form_item(
            "Disabled unchecked:",
            ToggleButton::new(move || disabled_unchecked.get())
                .on_event_stop(ToggleChanged::listener(), move |_, value| {
                    disabled_unchecked.set(*value);
                })
                .style(|s| s.set_disabled(true)),
        ),
    ))
}
