use std::{cell::Cell, rc::Rc};

use floem::{
    WgpuRenderContext,
    peniko::kurbo::{Point, Rect},
    prelude::*,
    style::Style,
    text::FontWeight,
};

pub fn gpu_scene_page(
    title: &'static str,
    subtitle: &'static str,
    button_label: &'static str,
    details: &'static str,
    render: impl for<'a> FnMut(&mut WgpuRenderContext<'a>, u64) + 'static,
) -> impl IntoView {
    let frame = Rc::new(Cell::new(0_u64));
    let repaint_pending = Rc::new(Cell::new(false));
    let render = Rc::new(std::cell::RefCell::new(render));

    Stack::vertical((
        Stack::vertical((
            title.style(|s| {
                s.font_size(24)
                    .font_weight(FontWeight::BOLD)
                    .color(Color::from_rgb8(15, 23, 42))
            }),
            subtitle.style(|s| {
                s.font_size(14)
                    .font_weight(FontWeight::MEDIUM)
                    .color(Color::from_rgb8(71, 85, 105))
            }),
        ))
        .style(|s| s.flex_col().gap(4).items_center()),
        Stack::vertical((
            canvas({
                let frame = frame.clone();
                let repaint_pending = repaint_pending.clone();
                let render = render.clone();
                move |cx, size| {
                    let rect = Rect::from_origin_size(Point::ZERO, size);
                    let frame_index = frame.get();
                    frame.set(frame_index + 1);

                    cx.draw_wgpu_scene(
                        rect,
                        Box::new({
                            let render = render.clone();
                            move |gpu| {
                                (render.borrow_mut())(gpu, frame_index);
                            }
                        }),
                    );

                    if !repaint_pending.replace(true) {
                        let repaint_pending = repaint_pending.clone();
                        let id = cx.target_id.owning_id();
                        cx.request_animation_frame(move |_| {
                            repaint_pending.set(false);
                            id.request_paint();
                        });
                    }
                }
            })
            .style(|s| {
                s.width(512)
                    .height(320)
                    .border_radius(18)
                    .corner_smoothing(0.6)
                    .background(Color::from_rgb8(10, 14, 23))
            }),
            Stack::vertical((
                title.style(|s| {
                    s.font_size(15)
                        .font_weight(FontWeight::SEMI_BOLD)
                        .color(Color::from_rgb8(15, 23, 42))
                }),
                details.style(|s| {
                    s.font_size(12)
                        .color(Color::from_rgb8(71, 85, 105))
                        .selectable(false)
                }),
            ))
            .style(|s| s.flex_col().gap(3).items_center()),
        ))
        .style(panel_style),
        Button::new(button_label)
            .action({
                let frame = frame.clone();
                move || frame.set(0)
            })
            .style(|s| {
                s.padding_horiz(18)
                    .height(36)
                    .border_radius(10)
                    .corner_smoothing(0.6)
            }),
    ))
    .style(|s| {
        s.size_full()
            .min_width(680)
            .min_height(560)
            .flex_col()
            .items_center()
            .justify_center()
            .gap(22)
            .padding(28)
            .background(Color::from_rgb8(241, 245, 249))
    })
}

fn panel_style(s: Style) -> Style {
    s.width(560)
        .max_width(88.pct())
        .padding(22)
        .flex_col()
        .gap(14)
        .items_center()
        .border_radius(22)
        .corner_smoothing(0.6)
        .background(Color::from_rgba8(255, 255, 255, 238))
        .border(1)
        .border_color(Color::from_rgba8(148, 163, 184, 80))
        .box_shadow_blur(24)
        .box_shadow_color(Color::from_rgba8(15, 23, 42, 34))
        .box_shadow_v_offset(12)
}
