use floem::{
    AnyView, IntoView,
    prelude::*,
    style::{Background, Style, Transition},
    text::FontWeight,
    theme::StyleThemeExt,
};

fn cell_style(width: f64, header: bool) -> Style {
    Style::new()
        .width(width)
        .height(if header { 40.0 } else { 36.0 })
        .padding_horiz(8.0)
        .items_center()
        .font_size(14.0)
        .apply_if(header, |s| s.font_weight(FontWeight::MEDIUM))
}

fn cell(text: &'static str, width: f64, header: bool) -> AnyView {
    text.style(move |s| {
        s.apply(cell_style(width, header)).with_theme(move |s, t| {
            if header {
                s.color(t.foreground())
            } else {
                s.color(t.foreground())
            }
        })
    })
    .into_any()
}

fn muted_cell(text: &'static str, width: f64) -> AnyView {
    text.style(move |s| {
        s.apply(cell_style(width, false))
            .with_theme(|s, t| s.color(t.muted_foreground()))
    })
    .into_any()
}

fn row(cells: (AnyView, AnyView, AnyView, AnyView), selected: bool) -> AnyView {
    Stack::horizontal(cells)
        .style(move |s| {
            s.width(640.0)
                .border_bottom(1.0)
                .transition(Background, Transition::linear(100.millis()))
                .with_theme(|s, t| {
                    s.border_color(t.border())
                        .hover(|s| s.background(t.def(|t| t.muted().with_alpha(0.5))))
                })
                .apply_if(selected, |s| s.with_theme(|s, t| s.background(t.muted())))
        })
        .into_any()
}

pub fn table_view() -> impl IntoView {
    let header = row(
        (
            cell("Invoice", 150.0, true),
            cell("Status", 150.0, true),
            cell("Method", 170.0, true),
            cell("Amount", 170.0, true),
        ),
        false,
    );

    let footer = Stack::horizontal((cell("Total", 470.0, true), cell("$2,500.00", 170.0, true)))
        .style(|s| {
            s.width(640.0).border_top(1.0).with_theme(|s, t| {
                s.background(t.def(|t| t.muted().with_alpha(0.5)))
                    .border_color(t.border())
            })
        });

    Stack::vertical((
        header,
        row(
            (
                cell("INV001", 150.0, false),
                muted_cell("Paid", 150.0),
                muted_cell("Credit Card", 170.0),
                cell("$250.00", 170.0, false),
            ),
            false,
        ),
        row(
            (
                cell("INV002", 150.0, false),
                muted_cell("Pending", 150.0),
                muted_cell("PayPal", 170.0),
                cell("$150.00", 170.0, false),
            ),
            true,
        ),
        row(
            (
                cell("INV003", 150.0, false),
                muted_cell("Unpaid", 150.0),
                muted_cell("Bank Transfer", 170.0),
                cell("$350.00", 170.0, false),
            ),
            false,
        ),
        footer,
        "A list of recent invoices.".style(|s| {
            s.margin_top(16.0)
                .font_size(14.0)
                .with_theme(|s, t| s.color(t.muted_foreground()))
        }),
    ))
    .style(|s| {
        s.padding(30.0)
            .with_theme(|s, t| s.background(t.background()).color(t.foreground()))
    })
}
