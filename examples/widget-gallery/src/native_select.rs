use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    prelude::*,
    theme::StyleThemeExt,
    views::dropdown::Dropdown,
};

use crate::form::{form, form_item};

#[derive(Clone, Copy, Debug, PartialEq)]
enum SelectValue {
    Apple,
    Banana,
    Blueberry,
    Grapes,
}

impl std::fmt::Display for SelectValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Apple => f.write_str("Apple"),
            Self::Banana => f.write_str("Banana"),
            Self::Blueberry => f.write_str("Blueberry"),
            Self::Grapes => f.write_str("Grapes"),
        }
    }
}

fn chevron() -> AnyView {
    icon_library::icon(IconLibrary::Lucide, "chevron-down")
        .map(|icon| icon.style(|s| s.size(16.0, 16.0)).into_any())
        .unwrap_or_else(|| Empty::new().style(|s| s.size(16.0, 16.0)).into_any())
}

fn native_select(size: &'static str, height: f64) -> AnyView {
    let active = RwSignal::new(SelectValue::Apple);
    Dropdown::new_rw(
        active,
        [
            SelectValue::Apple,
            SelectValue::Banana,
            SelectValue::Blueberry,
            SelectValue::Grapes,
        ],
    )
    .main_view(move |value: SelectValue| {
        Stack::horizontal((value.to_string(), chevron()))
            .style(move |s| {
                s.width(220.0)
                    .height(height)
                    .items_center()
                    .justify_between()
                    .padding_left(10.0)
                    .padding_right(8.0)
                    .border(1.0)
                    .border_radius(if size == "sm" { 7.0 } else { 8.0 })
                    .corner_smoothing(0.6)
                    .font_size(14.0)
                    .with_theme(|s, t| {
                        s.background(t.def(|t| {
                            if t.is_dark {
                                t.input().with_alpha(0.30)
                            } else {
                                t.background
                            }
                        }))
                        .border_color(t.input())
                        .color(t.foreground())
                    })
                    .focus_visible(|s| {
                        s.with_theme(|s, t| {
                            s.border_color(t.ring())
                                .outline(3.0)
                                .outline_color(t.ring_focus())
                        })
                    })
            })
            .into_any()
    })
    .style(|s| s.width(220.0))
    .into_any()
}

pub fn native_select_view() -> impl IntoView {
    form((
        form_item("Default:", native_select("default", 32.0)),
        form_item("Small:", native_select("sm", 28.0)),
    ))
    .style(|s| s.with_theme(|s, t| s.background(t.background()).color(t.foreground())))
}
