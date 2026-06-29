use floem::style::Style;

pub fn wrap_text() -> Style {
    Style::new().min_width(0.0).text_wrap()
}

pub fn text_column() -> Style {
    Style::new().flex_col().min_width(0.0)
}

pub fn fixed_square(size: f64) -> Style {
    Style::new()
        .size(size, size)
        .min_width(size)
        .max_width(size)
        .aspect_ratio(1.0)
        .flex_shrink(0.0)
}
