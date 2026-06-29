use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    peniko::Color,
    prelude::*,
    style::{Background, Style, Transition},
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum Align {
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy)]
enum ToggleGroupVariant {
    Default,
    Outline,
}

fn icon(name: &'static str) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(|s| s.size(16.0, 16.0).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(|s| s.size(16.0, 16.0)).into_any())
}

fn item_style(
    variant: ToggleGroupVariant,
    pressed: bool,
    joined: bool,
    first: bool,
    last: bool,
) -> Style {
    let style = Style::new()
        .height(32.0)
        .min_width(32.0)
        .padding_horiz(10.0)
        .gap(4.0)
        .font_size(14.0)
        .font_weight(FontWeight::MEDIUM)
        .border(1.0)
        .border_color(Color::TRANSPARENT)
        .background(Color::TRANSPARENT)
        .transition(Background, Transition::linear(100.millis()))
        .with_theme(|s, t| {
            s.color(t.foreground())
                .hover(|s| s.background(t.muted()).color(t.foreground()))
        })
        .focus_visible(|s| {
            s.with_theme(|s, t| {
                s.border_color(t.ring())
                    .outline(3.0)
                    .outline_color(t.ring_focus())
            })
        })
        .apply_if(pressed, |s| s.with_theme(|s, t| s.background(t.muted())))
        .apply_if(!joined, |s| s.border_radius(8.0).corner_smoothing(0.6))
        .apply_if(joined, |s| {
            s.border_radius(0.0)
                .apply_if(first, |s| {
                    s.border_top_left_radius(8.0)
                        .border_bottom_left_radius(8.0)
                        .corner_smoothing(0.6)
                })
                .apply_if(last, |s| {
                    s.border_top_right_radius(8.0)
                        .border_bottom_right_radius(8.0)
                        .corner_smoothing(0.6)
                })
        });

    match variant {
        ToggleGroupVariant::Default => style,
        ToggleGroupVariant::Outline => style.with_theme(|s, t| {
            s.border_color(t.input())
                .background(t.input_background())
                .hover(|s| s.background(t.muted()))
        }),
    }
}

fn toggle_item(
    icon_name: &'static str,
    value: Align,
    selected: RwSignal<Align>,
    variant: ToggleGroupVariant,
    joined: bool,
    first: bool,
    last: bool,
) -> Button {
    Button::new(icon(icon_name))
        .action(move || selected.set(value))
        .style(move |s| {
            s.apply(item_style(
                variant,
                selected.get() == value,
                joined,
                first,
                last,
            ))
        })
}

fn group(selected: RwSignal<Align>, variant: ToggleGroupVariant, joined: bool) -> impl IntoView {
    Stack::horizontal((
        toggle_item(
            "align-left",
            Align::Left,
            selected,
            variant,
            joined,
            true,
            false,
        ),
        toggle_item(
            "align-center",
            Align::Center,
            selected,
            variant,
            joined,
            false,
            false,
        ),
        toggle_item(
            "align-right",
            Align::Right,
            selected,
            variant,
            joined,
            false,
            true,
        ),
    ))
    .style(move |s| s.items_center().gap(if joined { 0.0 } else { 8.0 }))
}

fn vertical_group(selected: RwSignal<Align>) -> impl IntoView {
    Stack::vertical((
        toggle_item(
            "align-left",
            Align::Left,
            selected,
            ToggleGroupVariant::Outline,
            false,
            false,
            false,
        ),
        toggle_item(
            "align-center",
            Align::Center,
            selected,
            ToggleGroupVariant::Outline,
            false,
            false,
            false,
        ),
        toggle_item(
            "align-right",
            Align::Right,
            selected,
            ToggleGroupVariant::Outline,
            false,
            false,
            false,
        ),
    ))
    .style(|s| s.items_start().gap(8.0))
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

pub fn toggle_group_view() -> impl IntoView {
    let default_value = RwSignal::new(Align::Center);
    let outline_value = RwSignal::new(Align::Left);
    let joined_value = RwSignal::new(Align::Right);
    let vertical_value = RwSignal::new(Align::Center);

    Stack::vertical((
        "Toggle Group".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::horizontal((
            section(
                "Default spacing",
                group(default_value, ToggleGroupVariant::Default, false),
            ),
            section(
                "Outline spacing",
                group(outline_value, ToggleGroupVariant::Outline, false),
            ),
            section(
                "Joined",
                group(joined_value, ToggleGroupVariant::Outline, true),
            ),
            section("Vertical", vertical_group(vertical_value)),
        ))
        .style(|s| {
            s.items_start()
                .gap(24.0)
                .flex_wrap(floem::taffy::FlexWrap::Wrap)
        }),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
