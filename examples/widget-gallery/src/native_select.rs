use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    menu::Menu,
    prelude::*,
    theme::StyleThemeExt,
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

impl SelectValue {
    fn all() -> [Self; 4] {
        [Self::Apple, Self::Banana, Self::Blueberry, Self::Grapes]
    }
}

fn chevron() -> AnyView {
    icon_library::icon(IconLibrary::Lucide, "chevron-down")
        .map(|icon| {
            icon.style(|s| s.size(16.0, 16.0).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(|s| s.size(16.0, 16.0)).into_any())
}

fn native_select_menu(active: RwSignal<SelectValue>) -> Menu {
    SelectValue::all()
        .into_iter()
        .fold(Menu::new(), move |menu, value| {
            menu.item(value.to_string(), move |item| {
                item.checked(active.get_untracked() == value)
                    .action(move || active.set(value))
            })
        })
}

fn native_select(size: &'static str, height: f64) -> AnyView {
    let active = RwSignal::new(SelectValue::Apple);
    Stack::horizontal((
        dyn_view(move || {
            active
                .get()
                .to_string()
                .style(|s| s.font_size(14.0))
                .into_any()
        }),
        Empty::new().style(|s| s.flex_grow(1.0)),
        chevron(),
    ))
    .popout_menu(move || native_select_menu(active))
    .style(move |s| {
        s.width(220.0)
            .height(height)
            .items_center()
            .gap(8.0)
            .padding_left(10.0)
            .padding_right(8.0)
            .border(1.0)
            .border_radius(if size == "sm" { 7.0 } else { 8.0 })
            .corner_smoothing(0.6)
            .font_size(14.0)
            .selectable(false)
            .cursor(floem::style::CursorStyle::Pointer)
            .with_theme(|s, t| {
                s.background(t.input_background())
                    .border_color(t.input())
                    .color(t.foreground())
                    .hover(|s| s.background(t.input_background()))
                    .focus_visible(|s| {
                        s.border_color(t.ring())
                            .outline(3.0)
                            .outline_color(t.ring_focus())
                    })
            })
    })
    .into_any()
}

pub fn native_select_view() -> impl IntoView {
    form((
        form_item("Default:", native_select("default", 32.0)),
        form_item("Small:", native_select("sm", 28.0)),
    ))
    .style(|s| s.with_theme(|s, t| s.background(t.background()).color(t.foreground())))
}
