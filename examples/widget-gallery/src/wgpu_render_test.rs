use floem::{WgpuRenderContext, prelude::*};

use crate::render_test_common::gpu_scene_page;

const FRAME_TIME: f32 = 1.0 / 60.0;

const SHADER: &str = r#"
struct Params {
    time: f32,
    aspect: f32,
    frame: f32,
    _pad: f32,
};

@group(0) @binding(0)
var<uniform> params: Params;

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> VertexOut {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>( 3.0, -1.0),
        vec2<f32>(-1.0,  3.0),
    );

    let pos = positions[index];
    var out: VertexOut;
    out.position = vec4<f32>(pos, 0.0, 1.0);
    out.uv = pos * 0.5 + vec2<f32>(0.5, 0.5);
    return out;
}

fn rotate(p: vec2<f32>, angle: f32) -> vec2<f32> {
    let c = cos(angle);
    let s = sin(angle);
    return vec2<f32>(p.x * c - p.y * s, p.x * s + p.y * c);
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    let p = (in.uv * 2.0 - vec2<f32>(1.0, 1.0)) * vec2<f32>(params.aspect, 1.0);
    let spin = rotate(p, params.time * 0.42);
    let wave = 0.5 + 0.5 * sin(10.0 * length(spin) - params.time * 3.2);

    let orbit_a = vec2<f32>(cos(params.time * 1.25), sin(params.time * 0.92)) * vec2<f32>(0.58, 0.34);
    let orbit_b = vec2<f32>(sin(params.time * 0.74), cos(params.time * 1.38)) * vec2<f32>(0.42, 0.44);
    let glow_a = exp(-9.0 * dot(p - orbit_a, p - orbit_a));
    let glow_b = exp(-7.0 * dot(p + orbit_b, p + orbit_b));
    let ring = smoothstep(0.024, 0.0, abs(length(spin) - (0.42 + 0.08 * sin(params.time))));

    let base = vec3<f32>(
        0.08 + 0.42 * in.uv.x + 0.18 * wave,
        0.10 + 0.34 * in.uv.y + 0.16 * sin(params.time + spin.x * 4.0),
        0.18 + 0.48 * (1.0 - in.uv.x) + 0.20 * cos(params.time * 0.7 + spin.y * 5.0)
    );
    let color = base
        + glow_a * vec3<f32>(1.0, 0.28, 0.06)
        + glow_b * vec3<f32>(0.05, 0.82, 1.0)
        + ring * vec3<f32>(0.88, 1.0, 0.18);

    return vec4<f32>(pow(color, vec3<f32>(0.9)), 1.0);
}
"#;

pub fn wgpu_render_test_view() -> impl IntoView {
    let mut scene = WgpuDirectScene::default();
    gpu_scene_page(
        "WGPU Render Test",
        "Direct animated pass into Floem's current wgpu target",
        "Restart WGPU animation",
        "No texture readback, no PNG encode; paint schedules the next visible frame only.",
        move |ctx, frame| scene.render(ctx, frame),
    )
}

#[derive(Default)]
struct WgpuDirectScene {
    format: Option<wgpu::TextureFormat>,
    pipeline: Option<wgpu::RenderPipeline>,
    uniform_buffer: Option<wgpu::Buffer>,
    bind_group: Option<wgpu::BindGroup>,
}

impl WgpuDirectScene {
    fn render(&mut self, ctx: &mut WgpuRenderContext<'_>, frame: u64) {
        if self.pipeline.is_none() || self.format != Some(ctx.target_format) {
            self.create_resources(ctx);
        }

        let Some(pipeline) = self.pipeline.as_ref() else {
            return;
        };
        let Some(uniform_buffer) = self.uniform_buffer.as_ref() else {
            return;
        };
        let Some(bind_group) = self.bind_group.as_ref() else {
            return;
        };
        let Some(viewport) = viewport(ctx.rect, ctx.surface_size.width, ctx.surface_size.height)
        else {
            return;
        };

        let time = frame as f32 * FRAME_TIME;
        ctx.queue.write_buffer(
            uniform_buffer,
            0,
            &uniform_bytes(
                time,
                viewport.width / viewport.height.max(1.0),
                frame as f32,
            ),
        );

        let color_attachment = Some(wgpu::RenderPassColorAttachment {
            view: ctx.target_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            },
            depth_slice: None,
        });
        let mut pass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("widget-gallery-direct-wgpu-scene"),
            color_attachments: &[color_attachment],
            ..Default::default()
        });
        pass.set_pipeline(pipeline);
        pass.set_bind_group(0, bind_group, &[]);
        pass.set_viewport(
            viewport.x,
            viewport.y,
            viewport.width,
            viewport.height,
            0.0,
            1.0,
        );
        pass.set_scissor_rect(
            viewport.scissor_x,
            viewport.scissor_y,
            viewport.scissor_width,
            viewport.scissor_height,
        );
        pass.draw(0..3, 0..1);
    }

    fn create_resources(&mut self, ctx: &WgpuRenderContext<'_>) {
        let uniform_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("widget-gallery-direct-wgpu-uniforms"),
            size: 16,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let bind_group_layout =
            ctx.device
                .create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                    label: Some("widget-gallery-direct-wgpu-bind-group-layout"),
                    entries: &[wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Buffer {
                            ty: wgpu::BufferBindingType::Uniform,
                            has_dynamic_offset: false,
                            min_binding_size: None,
                        },
                        count: None,
                    }],
                });

        let bind_group = ctx.device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("widget-gallery-direct-wgpu-bind-group"),
            layout: &bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: uniform_buffer.as_entire_binding(),
            }],
        });

        let shader = ctx
            .device
            .create_shader_module(wgpu::ShaderModuleDescriptor {
                label: Some("widget-gallery-direct-wgpu-shader"),
                source: wgpu::ShaderSource::Wgsl(SHADER.into()),
            });

        let pipeline_layout = ctx
            .device
            .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                label: Some("widget-gallery-direct-wgpu-pipeline-layout"),
                bind_group_layouts: &[&bind_group_layout],
                push_constant_ranges: &[],
            });

        let pipeline = ctx
            .device
            .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("widget-gallery-direct-wgpu-pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &[],
                },
                primitive: wgpu::PrimitiveState::default(),
                depth_stencil: None,
                multisample: wgpu::MultisampleState::default(),
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: ctx.target_format,
                        blend: Some(wgpu::BlendState::REPLACE),
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                multiview: None,
                cache: None,
            });

        self.format = Some(ctx.target_format);
        self.pipeline = Some(pipeline);
        self.uniform_buffer = Some(uniform_buffer);
        self.bind_group = Some(bind_group);
    }
}

fn uniform_bytes(time: f32, aspect: f32, frame: f32) -> [u8; 16] {
    let mut bytes = [0; 16];
    bytes[0..4].copy_from_slice(&time.to_ne_bytes());
    bytes[4..8].copy_from_slice(&aspect.to_ne_bytes());
    bytes[8..12].copy_from_slice(&frame.to_ne_bytes());
    bytes
}

struct Viewport {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    scissor_x: u32,
    scissor_y: u32,
    scissor_width: u32,
    scissor_height: u32,
}

fn viewport(
    rect: floem::peniko::kurbo::Rect,
    surface_width: f64,
    surface_height: f64,
) -> Option<Viewport> {
    let x0 = rect.x0.max(0.0).min(surface_width);
    let y0 = rect.y0.max(0.0).min(surface_height);
    let x1 = rect.x1.max(0.0).min(surface_width);
    let y1 = rect.y1.max(0.0).min(surface_height);
    let width = (x1 - x0).max(0.0) as f32;
    let height = (y1 - y0).max(0.0) as f32;
    if width < 1.0 || height < 1.0 {
        return None;
    }

    let scissor_x = x0.floor() as u32;
    let scissor_y = y0.floor() as u32;
    let scissor_width = (x1.ceil() as u32).saturating_sub(scissor_x).max(1);
    let scissor_height = (y1.ceil() as u32).saturating_sub(scissor_y).max(1);

    Some(Viewport {
        x: x0 as f32,
        y: y0 as f32,
        width,
        height,
        scissor_x,
        scissor_y,
        scissor_width,
        scissor_height,
    })
}
