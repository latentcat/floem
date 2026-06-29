use floem::{
    AnyView, IntoView,
    icons::{self as icon_library, IconLibrary},
    prelude::*,
    style::{Background, Foreground, Opacity, Transition},
    taffy::style::Display,
    text::FontWeight,
    theme::StyleThemeExt,
    views::{Button, Decorators},
};

fn icon(name: &'static str) -> AnyView {
    icon_library::icon(IconLibrary::Lucide, name)
        .map(|icon| {
            icon.style(|s| s.size(16.0, 16.0).flex_shrink(0.0))
                .into_any()
        })
        .unwrap_or_else(|| Empty::new().style(|s| s.size(16.0, 16.0)).into_any())
}

fn accordion_item(
    index: usize,
    title: &'static str,
    body: &'static str,
    open: RwSignal<Option<usize>>,
    disabled: bool,
) -> AnyView {
    let is_open = move || open.get() == Some(index);

    Stack::vertical((
        Button::new(Stack::horizontal((
            title.style(|s| {
                s.font_size(14.0)
                    .font_weight(FontWeight::MEDIUM)
                    .line_height(1.3)
            }),
            Empty::new().style(|s| s.flex_grow(1.0)),
            dyn_view(move || {
                if is_open() {
                    icon("chevron-up")
                } else {
                    icon("chevron-down")
                }
            })
            .style(|s| s.with_theme(|s, t| s.color(t.muted_foreground()))),
        )))
        .action(move || {
            if !disabled {
                open.update(|current| {
                    *current = if *current == Some(index) {
                        None
                    } else {
                        Some(index)
                    };
                })
            }
        })
        .style(move |s| {
            s.width_full()
                .min_height(42.0)
                .items_start()
                .justify_between()
                .padding_horiz(0.0)
                .padding_vert(10.0)
                .border(1.0)
                .border_color(floem::peniko::Color::TRANSPARENT)
                .background(floem::peniko::Color::TRANSPARENT)
                .transition(Background, Transition::linear(100.millis()))
                .transition(Foreground, Transition::linear(100.millis()))
                .with_theme(|s, t| {
                    s.color(t.foreground())
                        .hover(|s| s.color(t.foreground()))
                        .focus_visible(|s| {
                            s.border_color(t.ring())
                                .outline(3.0)
                                .outline_color(t.ring_focus())
                        })
                        .disabled(|s| s.set(Opacity, 0.5).unset_cursor())
                })
                .apply_if(disabled, |s| s.set_disabled(true))
        }),
        body.style(move |s| {
            s.padding_bottom(10.0)
                .font_size(14.0)
                .line_height(1.45)
                .with_theme(|s, t| s.color(t.muted_foreground()))
                .apply_if(!is_open(), |s| s.display(Display::None))
        }),
    ))
    .style(|s| {
        s.flex_col()
            .border_bottom(1.0)
            .with_theme(|s, t| s.border_color(t.border()))
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

pub fn accordion_view() -> impl IntoView {
    let single_open = RwSignal::new(Some(0usize));
    let compact_open = RwSignal::new(None::<usize>);

    Stack::vertical((
        "Accordion".style(|s| {
            s.font_size(20.0)
                .font_weight(FontWeight::SEMI_BOLD)
                .with_theme(|s, t| s.color(t.foreground()))
        }),
        Stack::horizontal((
            section(
                "Single",
                Stack::vertical((
                    accordion_item(
                        0,
                        "Is it accessible?",
                        "Yes. It follows the same disclosure structure and keyboard focus styling as other controls.",
                        single_open,
                        false,
                    ),
                    accordion_item(
                        1,
                        "Is it styled?",
                        "The trigger, border, icon, and content spacing are aligned to the shadcn b0 defaults.",
                        single_open,
                        false,
                    ),
                    accordion_item(
                        2,
                        "Can it animate?",
                        "Animation is left for the follow-up pass after coverage is complete.",
                        single_open,
                        false,
                    ),
                    accordion_item(
                        3,
                        "Is this disabled?",
                        "Disabled triggers keep opacity and do not toggle.",
                        single_open,
                        true,
                    ),
                ))
                .style(|s| s.width(460.0).flex_col()),
            ),
            section(
                "Closed default",
                Stack::vertical((
                    accordion_item(
                        0,
                        "Project settings",
                        "Control deployment regions, contributors, and notification defaults.",
                        compact_open,
                        false,
                    ),
                    accordion_item(
                        1,
                        "Billing",
                        "Review current plan and invoice history.",
                        compact_open,
                        false,
                    ),
                ))
                .style(|s| s.width(360.0).flex_col()),
            ),
        ))
        .style(|s| s.items_start().gap(24.0).flex_wrap(floem::taffy::FlexWrap::Wrap)),
    ))
    .style(|s| {
        s.flex_col()
            .gap(24.0)
            .padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
