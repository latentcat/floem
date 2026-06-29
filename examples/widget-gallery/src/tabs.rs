use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    peniko::Color,
    prelude::*,
    taffy::FlexWrap,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

#[derive(Clone, Copy, PartialEq, Eq)]
enum TabsVariant {
    Default,
    Line,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TabsOrientation {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum TabsLayout {
    Fit,
    FillProportional,
    FillEqual,
}

fn icon(name: &'static str, size: f64) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(move |s| s.size(size, size).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(move |s| s.size(size, size)).into_any())
}

fn trigger_content(label: &'static str, icon_name: Option<&'static str>) -> AnyView {
    if let Some(icon_name) = icon_name {
        Stack::horizontal((icon(icon_name, 14.0), label))
            .style(|s| s.items_center().gap(6.0))
            .into_any()
    } else {
        label.into_any()
    }
}

fn tabs_trigger(
    label: &'static str,
    icon_name: Option<&'static str>,
    index: usize,
    active: RwSignal<usize>,
    variant: TabsVariant,
    orientation: TabsOrientation,
    layout: TabsLayout,
    disabled: bool,
) -> Button {
    Button::new(trigger_content(label, icon_name))
        .action(move || {
            if !disabled {
                active.set(index);
            }
        })
        .style(move |s| {
            let selected = active.get() == index;
            let base = s
                .height(26.0)
                .padding_horiz(6.0)
                .padding_vert(0.0)
                .border(1.0)
                .border_radius(6.0)
                .corner_smoothing(0.6)
                .font_size(14.0)
                .font_weight(FontWeight::MEDIUM)
                .apply_if(
                    orientation == TabsOrientation::Horizontal && layout != TabsLayout::Fit,
                    |s| s.flex_grow(1.0).justify_center(),
                )
                .apply_if(
                    orientation == TabsOrientation::Horizontal && layout == TabsLayout::FillEqual,
                    |s| s.flex_basis(0.0),
                )
                .background(Color::TRANSPARENT)
                .border_color(Color::TRANSPARENT)
                .with_theme(|s, t| {
                    s.color(t.def(|t| t.foreground.with_alpha(if t.is_dark { 0.7 } else { 0.6 })))
                        .hover(|s| s.color(t.foreground()))
                        .disabled(|s| s.set(floem::style::Opacity, 0.5).unset_cursor())
                })
                .apply_if(disabled, |s| s.set_disabled(true));

            match variant {
                TabsVariant::Default => base
                    .apply_if(orientation == TabsOrientation::Vertical, |s| {
                        s.width_full().justify_start()
                    })
                    .apply_if(selected, |s| {
                        s.box_shadow_v_offset(1.0)
                            .box_shadow_blur(2.0)
                            .box_shadow_color(Color::from_rgb8(0, 0, 0).with_alpha(0.08))
                            .with_theme(|s, t| {
                                s.background(t.background())
                                    .color(t.foreground())
                                    .border_color(t.def(|t| {
                                        if t.is_dark {
                                            t.input
                                        } else {
                                            Color::TRANSPARENT
                                        }
                                    }))
                            })
                    }),
                TabsVariant::Line => base
                    .height(32.0)
                    .border(0.0)
                    .border_radius(0.0)
                    .background(Color::TRANSPARENT)
                    .apply_if(orientation == TabsOrientation::Horizontal, |s| {
                        s.border_bottom(2.0)
                    })
                    .apply_if(orientation == TabsOrientation::Vertical, |s| {
                        s.width_full().justify_start().border_right(2.0)
                    })
                    .apply_if(selected, |s| {
                        s.with_theme(|s, t| s.color(t.foreground()).border_color(t.foreground()))
                    }),
            }
        })
}

fn tabs_list(
    active: RwSignal<usize>,
    variant: TabsVariant,
    orientation: TabsOrientation,
    layout: TabsLayout,
    items: [(&'static str, Option<&'static str>, bool); 3],
) -> AnyView {
    let triggers = (
        tabs_trigger(
            items[0].0,
            items[0].1,
            0,
            active,
            variant,
            orientation,
            layout,
            items[0].2,
        ),
        tabs_trigger(
            items[1].0,
            items[1].1,
            1,
            active,
            variant,
            orientation,
            layout,
            items[1].2,
        ),
        tabs_trigger(
            items[2].0,
            items[2].1,
            2,
            active,
            variant,
            orientation,
            layout,
            items[2].2,
        ),
    );

    match orientation {
        TabsOrientation::Horizontal => Stack::horizontal(triggers)
            .style(move |s| {
                let s = match variant {
                    TabsVariant::Default => s
                        .height(32.0)
                        .items_center()
                        .gap(0.0)
                        .padding(3.0)
                        .border_radius(8.0)
                        .corner_smoothing(0.6)
                        .with_theme(|s, t| s.background(t.muted()).color(t.muted_foreground())),
                    TabsVariant::Line => s
                        .height(36.0)
                        .items_center()
                        .gap(4.0)
                        .padding(0.0)
                        .background(Color::TRANSPARENT),
                };

                s.apply_if(layout == TabsLayout::Fit, |s| s.flex_shrink(0.0))
                    .apply_if(layout != TabsLayout::Fit, |s| s.width_full())
            })
            .into_any(),
        TabsOrientation::Vertical => Stack::vertical(triggers)
            .style(move |s| match variant {
                TabsVariant::Default => s
                    .width(184.0)
                    .items_stretch()
                    .gap(0.0)
                    .padding(3.0)
                    .border_radius(8.0)
                    .corner_smoothing(0.6)
                    .with_theme(|s, t| s.background(t.muted()).color(t.muted_foreground())),
                TabsVariant::Line => s
                    .width(184.0)
                    .items_stretch()
                    .gap(4.0)
                    .padding(0.0)
                    .background(Color::TRANSPARENT),
            })
            .into_any(),
    }
}

fn tabs_content(active: RwSignal<usize>) -> impl IntoView {
    dyn_view(move || {
        let (title, body) = match active.get() {
            0 => (
                "Account",
                "Manage account details and authentication settings.",
            ),
            1 => ("Password", "Update credentials and recovery options."),
            _ => (
                "Billing",
                "Review plan limits, invoices, and payment methods.",
            ),
        };

        Stack::vertical((
            title.style(|s| {
                s.font_size(14.0)
                    .font_weight(FontWeight::SEMI_BOLD)
                    .with_theme(|s, t| s.color(t.foreground()))
            }),
            body.style(|s| {
                s.font_size(13.0)
                    .line_height(1.35)
                    .with_theme(|s, t| s.color(t.muted_foreground()))
            }),
        ))
        .style(|s| {
            s.flex_col()
                .gap(6.0)
                .padding(14.0)
                .border(1.0)
                .border_radius(8.0)
                .corner_smoothing(0.6)
                .with_theme(|s, t| s.background(t.card()).border_color(t.border()))
        })
    })
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

pub fn tab_view() -> impl IntoView {
    let default_tab = RwSignal::new(0usize);
    let line_tab = RwSignal::new(0usize);
    let icon_tab = RwSignal::new(0usize);
    let fill_tab = RwSignal::new(0usize);
    let equal_tab = RwSignal::new(0usize);
    let vertical_tab = RwSignal::new(0usize);

    Stack::vertical((
        "Tabs".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        section(
            "Default",
            Stack::vertical((
                tabs_list(
                    default_tab,
                    TabsVariant::Default,
                    TabsOrientation::Horizontal,
                    TabsLayout::Fit,
                    [
                        ("Account", None, false),
                        ("Password", None, false),
                        ("Billing", None, false),
                    ],
                ),
                tabs_content(default_tab),
            ))
            .style(|s| s.flex_col().items_start().gap(10.0).max_width(460.0)),
        ),
        section(
            "Line",
            Stack::vertical((
                tabs_list(
                    line_tab,
                    TabsVariant::Line,
                    TabsOrientation::Horizontal,
                    TabsLayout::Fit,
                    [
                        ("Preview", None, false),
                        ("Code", None, false),
                        ("Activity", None, false),
                    ],
                ),
                tabs_content(line_tab),
            ))
            .style(|s| s.flex_col().items_start().gap(10.0).max_width(460.0)),
        ),
        section(
            "Icons & Disabled",
            Stack::vertical((
                tabs_list(
                    icon_tab,
                    TabsVariant::Default,
                    TabsOrientation::Horizontal,
                    TabsLayout::Fit,
                    [
                        ("Overview", Some("layout-dashboard"), false),
                        ("Files", Some("folder"), false),
                        ("Archive", Some("archive"), true),
                    ],
                ),
                tabs_content(icon_tab),
            ))
            .style(|s| s.flex_col().items_start().gap(10.0).max_width(460.0)),
        ),
        section(
            "Full Width Proportional",
            Stack::vertical((
                tabs_list(
                    fill_tab,
                    TabsVariant::Default,
                    TabsOrientation::Horizontal,
                    TabsLayout::FillProportional,
                    [
                        ("Short", None, false),
                        ("Longer Label", None, false),
                        ("Activity", None, false),
                    ],
                ),
                tabs_content(fill_tab),
            ))
            .style(|s| s.width(460.0).flex_col().gap(10.0)),
        ),
        section(
            "Full Width Equal",
            Stack::vertical((
                tabs_list(
                    equal_tab,
                    TabsVariant::Default,
                    TabsOrientation::Horizontal,
                    TabsLayout::FillEqual,
                    [
                        ("Account", None, false),
                        ("Password", None, false),
                        ("Billing", None, false),
                    ],
                ),
                tabs_content(equal_tab),
            ))
            .style(|s| s.width(460.0).flex_col().gap(10.0)),
        ),
        section(
            "Vertical",
            Stack::horizontal((
                tabs_list(
                    vertical_tab,
                    TabsVariant::Line,
                    TabsOrientation::Vertical,
                    TabsLayout::Fit,
                    [
                        ("Profile", Some("user"), false),
                        ("Security", Some("shield"), false),
                        ("Billing", Some("credit-card"), false),
                    ],
                ),
                tabs_content(vertical_tab).style(|s| s.width(360.0)),
            ))
            .style(|s| {
                s.items_start()
                    .gap(16.0)
                    .flex_wrap(floem::taffy::FlexWrap::Wrap)
            }),
        ),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .flex_wrap(FlexWrap::Wrap)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
