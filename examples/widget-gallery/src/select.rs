use std::fmt::Display;

use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    peniko::Color,
    prelude::*,
    style::{Background, Foreground, Opacity, Style, Transition},
    taffy::FlexWrap,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{
        Decorators,
        dropdown::{self, Dropdown},
    },
};
use strum::IntoEnumIterator;

#[derive(Clone, Copy, Debug, PartialEq, Eq, strum::EnumIter)]
enum Region {
    NorthAmerica,
    Europe,
    AsiaPacific,
    LatinAmerica,
}

impl Region {
    fn label(self) -> &'static str {
        match self {
            Self::NorthAmerica => "North America",
            Self::Europe => "Europe",
            Self::AsiaPacific => "Asia Pacific",
            Self::LatinAmerica => "Latin America",
        }
    }

    fn detail(self) -> &'static str {
        match self {
            Self::NorthAmerica => "us-east-1",
            Self::Europe => "eu-central-1",
            Self::AsiaPacific => "ap-southeast-1",
            Self::LatinAmerica => "sa-east-1",
        }
    }
}

impl Display for Region {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.label())
    }
}

#[derive(Clone, Copy)]
enum SelectSize {
    Default,
    Sm,
}

fn icon(name: &'static str, size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

fn select_size_style(size: SelectSize) -> Style {
    match size {
        SelectSize::Default => Style::new().height(32.0).border_radius(8.0),
        SelectSize::Sm => Style::new().height(28.0).border_radius(8.0).font_size(13.0),
    }
}

fn invalid_style() -> Style {
    Style::new().with_theme(|s, t| {
        s.border_color(t.danger())
            .outline(3.0)
            .outline_color(t.def(|t| t.danger().with_alpha(if t.is_dark { 0.4 } else { 0.2 })))
    })
}

fn select_trigger(region: Option<Region>) -> AnyView {
    let is_placeholder = region.is_none();
    let label = region.map_or("Select region", Region::label);

    Stack::horizontal((
        label.style(move |s| {
            s.text_ellipsis().apply_if(is_placeholder, |s| {
                s.with_theme(|s, t| s.color(t.muted_foreground()))
            })
        }),
        icon("chevron-down", 16.0).style(|s| {
            s.with_theme(|s, t| s.color(t.muted_foreground()))
                .flex_shrink(0.0)
        }),
    ))
    .style(|s| s.items_center().justify_between().gap(6.0).width_full())
    .into_any()
}

fn select_item(region: Option<Region>, selected: RwSignal<Option<Region>>) -> AnyView {
    let Some(region) = region else {
        return Empty::new().into_any();
    };

    Stack::horizontal((
        Stack::vertical((
            region.label().style(|s| s.font_size(14.0)),
            region.detail().style(|s| {
                s.font_size(12.0)
                    .with_theme(|s, t| s.color(t.muted_foreground()))
            }),
        ))
        .style(|s| s.flex_col().gap(1.0)),
        dyn_view(move || {
            if selected.get() == Some(region) {
                icon("check", 16.0)
            } else {
                Empty::new().style(|s| s.size(16.0, 16.0)).into_any()
            }
        }),
    ))
    .style(|s| s.items_center().justify_between().gap(12.0).width_full())
    .into_any()
}

fn select_control(selected: RwSignal<Option<Region>>) -> Dropdown<Option<Region>> {
    Dropdown::custom(
        move || selected.get(),
        select_trigger,
        Region::iter().map(Some).collect::<Vec<_>>(),
        move |region| select_item(*region, selected),
    )
    .on_accept(move |value| selected.set(value))
}

fn select_label(label: &'static str) -> AnyView {
    label
        .style(|s| {
            s.padding_horiz(6.0)
                .padding_vert(4.0)
                .font_size(12.0)
                .font_weight(FontWeight::MEDIUM)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        })
        .into_any()
}

fn select_separator() -> AnyView {
    Empty::new()
        .style(|s| {
            s.height(1.0)
                .margin_vert(4.0)
                .margin_horiz(-4.0)
                .with_theme(|s, t| s.background(t.border()))
        })
        .into_any()
}

fn static_select_item(region: Region, checked: bool, disabled: bool) -> AnyView {
    Stack::horizontal((
        region.label().style(|s| s.font_size(14.0)),
        Empty::new().style(|s| s.flex_grow(1.0)),
        if checked {
            icon("check", 16.0)
        } else {
            Empty::new().style(|s| s.size(16.0, 16.0)).into_any()
        },
    ))
    .style(move |s| {
        s.min_height(28.0)
            .items_center()
            .gap(6.0)
            .padding_left(6.0)
            .padding_right(8.0)
            .padding_vert(4.0)
            .border_radius(6.0)
            .transition(Background, Transition::linear(100.millis()))
            .transition(Foreground, Transition::linear(100.millis()))
            .with_theme(|s, t| {
                s.color(t.popover_foreground())
                    .hover(|s| s.background(t.accent()).color(t.accent_foreground()))
            })
            .apply_if(disabled, |s| s.set(Opacity, 0.5).set_disabled(true))
    })
    .into_any()
}

fn select_content_surface() -> AnyView {
    Stack::vertical((
        select_label("Regions"),
        static_select_item(Region::NorthAmerica, false, false),
        static_select_item(Region::Europe, true, false),
        select_separator(),
        static_select_item(Region::AsiaPacific, false, false),
        static_select_item(Region::LatinAmerica, false, true),
    ))
    .style(|s| {
        s.width(240.0)
            .flex_col()
            .padding(4.0)
            .border(1.0)
            .border_radius(8.0)
            .corner_smoothing(0.6)
            .box_shadow_blur(12.0)
            .box_shadow_color(Color::from_rgb8(0, 0, 0).with_alpha(0.16))
            .with_theme(|s, t| {
                s.background(t.popover())
                    .color(t.popover_foreground())
                    .border_color(t.def(|t| t.foreground.with_alpha(0.10)))
            })
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

fn row(content: impl IntoView + 'static) -> AnyView {
    content
        .style(|s| s.items_start().gap(24.0).flex_wrap(FlexWrap::Wrap))
        .into_any()
}

pub fn select_view() -> impl IntoView {
    let region = RwSignal::new(Some(Region::Europe));
    let small_region = RwSignal::new(Some(Region::NorthAmerica));
    let placeholder_region = RwSignal::new(None);
    let disabled_region = RwSignal::new(Some(Region::NorthAmerica));
    let invalid_region = RwSignal::new(Some(Region::LatinAmerica));
    let open_region = RwSignal::new(Some(Region::AsiaPacific));
    let open = RwSignal::new(true);

    let open_select = select_control(open_region)
        .show_list(move || open.get())
        .on_event_stop(
            dropdown::DropdownOpenChanged::listener(),
            move |_, is_open| {
                open.set(*is_open);
            },
        )
        .style(|s| s.width(240.0));

    Stack::vertical((
        "Select".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        row(Stack::horizontal((
            section(
                "Default",
                select_control(region)
                    .style(|s| s.width(220.0).apply(select_size_style(SelectSize::Default))),
            ),
            section(
                "Small",
                select_control(small_region)
                    .style(|s| s.width(180.0).apply(select_size_style(SelectSize::Sm))),
            ),
            section(
                "Placeholder",
                select_control(placeholder_region).style(|s| s.width(220.0)),
            ),
        ))),
        row(Stack::horizontal((
            section(
                "Disabled",
                select_control(disabled_region).style(|s| s.width(220.0).set_disabled(true)),
            ),
            section(
                "Invalid",
                select_control(invalid_region).style(|s| s.width(220.0).apply(invalid_style())),
            ),
            section("Open", open_select),
        ))),
        section("Content anatomy", select_content_surface()),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
