use floem::{
    AnyView, IntoView,
    event::EventPropagation,
    peniko::Color,
    prelude::*,
    text::FontWeight,
    theme::StyleThemeExt,
    unit::Pct,
    views::{
        Decorators,
        slider::{self, SliderChanged},
    },
};

fn label(text: &'static str) -> impl IntoView {
    text.style(|s| {
        s.font_size(14.0)
            .font_weight(FontWeight::MEDIUM)
            .with_theme(|s, t| s.color(t.foreground()))
    })
}

fn value_label(value: RwSignal<Pct>) -> impl IntoView {
    Label::derived(move || format!("{:.0}%", value.get().0)).style(|s| {
        s.width(52.0)
            .font_size(13.0)
            .with_theme(|s, t| s.color(t.muted_foreground()))
    })
}

fn f64_value_label(value: RwSignal<f64>) -> impl IntoView {
    Label::derived(move || format!("{:.0}", value.get())).style(|s| {
        s.width(52.0)
            .font_size(13.0)
            .with_theme(|s, t| s.color(t.muted_foreground()))
    })
}

fn slider_row(
    title: &'static str,
    control: impl IntoView + 'static,
    value: impl IntoView + 'static,
) -> AnyView {
    Stack::vertical((
        Stack::horizontal((label(title), value))
            .style(|s| s.items_center().justify_between().gap(12.0).width(360.0)),
        control,
    ))
    .style(|s| s.flex_col().gap(10.0))
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
    .style(|s| s.flex_col().gap(12.0))
    .into_any()
}

fn range_preview() -> AnyView {
    Stack::horizontal((
        Empty::new().style(|s| {
            s.width(92.0)
                .height(4.0)
                .border_radius(999.0)
                .with_theme(|s, t| s.background(t.muted()))
        }),
        Empty::new().style(|s| {
            s.width(140.0)
                .height(4.0)
                .border_radius(999.0)
                .with_theme(|s, t| s.background(t.primary()))
        }),
        Empty::new().style(|s| {
            s.width(92.0)
                .height(4.0)
                .border_radius(999.0)
                .with_theme(|s, t| s.background(t.muted()))
        }),
    ))
    .style(|s| {
        s.width(360.0)
            .height(20.0)
            .items_center()
            .justify_center()
            .with_theme(|s, t| s.color(t.foreground()))
    })
    .into_any()
}

fn range_thumb() -> impl IntoView {
    Empty::new().style(|s| {
        s.size(12.0, 12.0)
            .border(1.0)
            .border_radius(999.0)
            .with_theme(|s, t| {
                s.background(t.def(|_| Color::WHITE))
                    .border_color(t.ring())
                    .outline(0.0)
            })
    })
}

fn range_visual() -> AnyView {
    Stack::vertical((
        Stack::horizontal((range_preview(), Empty::new().style(|s| s.width(0.0))))
            .style(|s| s.items_center()),
        Stack::horizontal((range_thumb(), range_thumb())).style(|s| {
            s.margin_top(-16.0)
                .margin_left(84.0)
                .gap(128.0)
                .items_center()
        }),
    ))
    .style(|s| s.width(360.0).height(24.0).flex_col())
    .into_any()
}

pub fn slider_view() -> impl IntoView {
    let volume = RwSignal::new(45.pct());
    let contrast = RwSignal::new(70.pct());
    let disabled = RwSignal::new(35.pct());
    let progress = RwSignal::new(62.pct());
    let ranged = RwSignal::new(24.0);
    let vertical = RwSignal::new(58.pct());

    let ranged_slider = slider::Slider::new_ranged(move || ranged.get(), -50.0..=50.0)
        .step(5.0)
        .on_event(SliderChanged::listener(), move |_cx, state| {
            ranged.set(state.value);
            EventPropagation::Continue
        })
        .style(|s| s.width(360.0));

    Stack::vertical((
        "Slider".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        section(
            "Horizontal",
            Stack::vertical((
                slider_row(
                    "Default",
                    slider::Slider::new_rw(volume).style(|s| s.width(360.0)),
                    value_label(volume),
                ),
                slider_row(
                    "Step 10",
                    slider::Slider::new_rw(contrast)
                        .step(10.0)
                        .style(|s| s.width(360.0)),
                    value_label(contrast),
                ),
                slider_row("Ranged -50..50", ranged_slider, f64_value_label(ranged)),
                slider_row(
                    "Disabled",
                    slider::Slider::new(move || disabled.get())
                        .style(|s| s.width(360.0).set_disabled(true)),
                    value_label(disabled),
                ),
            ))
            .style(|s| s.flex_col().gap(22.0)),
        ),
        section(
            "Vertical",
            Stack::horizontal((
                slider::Slider::new_rw(vertical)
                    .slider_style(|s| s.vertical(true))
                    .style(|s| s.width(16.0).height(180.0)),
                Stack::vertical((
                    label("Volume"),
                    value_label(vertical),
                    "Vertical orientation uses the same 4px track and 12px thumb."
                        .style(|s| {
                            s.width(240.0)
                                .font_size(13.0)
                                .line_height(1.35)
                                .with_theme(|s, t| s.color(t.muted_foreground()))
                        }),
                ))
                .style(|s| s.flex_col().gap(6.0)),
            ))
            .style(|s| s.items_center().gap(18.0)),
        ),
        section(
            "Readonly Progress",
            slider_row(
                "Progress",
                slider::Slider::new(move || progress.get())
                    .slider_style(|s| s.handle_radius(0.0).edge_align(true))
                    .style(|s| s.width(360.0).set_disabled(true)),
                value_label(progress),
            ),
        ),
        section(
            "Range Preview",
            Stack::vertical((
                range_visual(),
                "Static visual coverage for shadcn multi-thumb range; core multi-thumb API is still pending."
                    .style(|s| {
                        s.width(360.0)
                            .font_size(13.0)
                            .line_height(1.35)
                            .with_theme(|s, t| s.color(t.muted_foreground()))
                    }),
            ))
            .style(|s| s.flex_col().gap(8.0)),
        ),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .max_width(760.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
