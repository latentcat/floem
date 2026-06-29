use floem::prelude::*;

#[cfg(feature = "bevy-render-test")]
use crate::render_test_common::gpu_scene_page;

#[cfg(not(feature = "bevy-render-test"))]
pub fn bevy_render_test_view() -> impl IntoView {
    Stack::vertical((
        "Bevy Render Test".style(|s| {
            s.font_size(24)
                .font_weight(floem::text::FontWeight::BOLD)
                .color(Color::from_rgb8(15, 23, 42))
        }),
        "Rebuild widget-gallery with --features \"vello bevy-render-test\""
            .style(|s| s.font_size(14).color(Color::from_rgb8(71, 85, 105))),
    ))
    .style(|s| {
        s.size_full()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(8)
            .background(Color::from_rgb8(241, 245, 249))
    })
}

#[cfg(feature = "bevy-render-test")]
pub fn bevy_render_test_view() -> impl IntoView {
    let mut scene = bevy_direct::BevyDirectScene::new();
    gpu_scene_page(
        "Bevy Render Test",
        "Headless Bevy ECS drives a direct Floem wgpu pass",
        "Restart Bevy animation",
        "No winit, no Bevy screenshot, no GPU readback; Bevy updates only while this view paints.",
        move |ctx, frame| scene.render(ctx, frame),
    )
}

#[cfg(feature = "bevy-render-test")]
mod bevy_direct {
    use floem::WgpuRenderContext;

    use bevy::prelude::*;

    const WIDTH: f32 = 512.0;
    const HEIGHT: f32 = 320.0;
    const MAX_VERTICES: u64 = 256;
    const FRAME_TIME: f32 = 1.0 / 60.0;

    const SHADER: &str = r#"
struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec2<f32>,
    @location(1) color: vec4<f32>,
) -> VertexOut {
    var out: VertexOut;
    out.position = vec4<f32>(position, 0.0, 1.0);
    out.color = color;
    return out;
}

@fragment
fn fs_main(in: VertexOut) -> @location(0) vec4<f32> {
    return in.color;
}
"#;

    #[derive(Resource, Default)]
    struct AnimationClock {
        time: f32,
    }

    #[derive(Component)]
    struct AnimatedSprite {
        base: Vec3,
        radius: Vec2,
        speed: f32,
        spin: f32,
        phase: f32,
    }

    #[derive(Component)]
    struct GpuSprite {
        size: Vec2,
        color: [f32; 4],
    }

    #[repr(C)]
    #[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
    struct Vertex {
        position: [f32; 2],
        color: [f32; 4],
    }

    pub(super) struct BevyDirectScene {
        app: App,
        renderer: RectangleRenderer,
    }

    impl BevyDirectScene {
        pub(super) fn new() -> Self {
            let mut app = App::new();
            app.init_resource::<AnimationClock>()
                .add_plugins(MinimalPlugins.set(bevy::app::ScheduleRunnerPlugin::run_once()))
                .add_systems(Startup, setup_scene)
                .add_systems(Update, animate_scene);
            app.finish();
            app.cleanup();
            app.update();

            Self {
                app,
                renderer: RectangleRenderer::default(),
            }
        }

        pub(super) fn render(&mut self, ctx: &mut WgpuRenderContext<'_>, frame: u64) {
            {
                let mut clock = self.app.world_mut().resource_mut::<AnimationClock>();
                clock.time = frame as f32 * FRAME_TIME;
            }
            self.app.update();

            let mut vertices = background_vertices();
            let mut query = self.app.world_mut().query::<(&Transform, &GpuSprite)>();
            for (transform, sprite) in query.iter(self.app.world()) {
                append_sprite_vertices(&mut vertices, transform, sprite);
            }

            self.renderer.render(ctx, &vertices);
        }
    }

    fn setup_scene(mut commands: Commands) {
        spawn_sprite(
            &mut commands,
            [0.16, 0.31, 0.92, 0.92],
            Vec2::new(340.0, 150.0),
            AnimatedSprite {
                base: Vec3::new(-58.0, 12.0, 0.0),
                radius: Vec2::new(58.0, 34.0),
                speed: 1.25,
                spin: -0.9,
                phase: 0.2,
            },
        );
        spawn_sprite(
            &mut commands,
            [1.0, 0.34, 0.08, 0.86],
            Vec2::new(210.0, 210.0),
            AnimatedSprite {
                base: Vec3::new(84.0, 10.0, 1.0),
                radius: Vec2::new(42.0, 52.0),
                speed: 1.65,
                spin: 1.35,
                phase: 1.7,
            },
        );
        spawn_sprite(
            &mut commands,
            [0.12, 0.92, 0.60, 0.80],
            Vec2::new(282.0, 72.0),
            AnimatedSprite {
                base: Vec3::new(16.0, -86.0, 2.0),
                radius: Vec2::new(84.0, 22.0),
                speed: 0.95,
                spin: 0.55,
                phase: 3.1,
            },
        );
    }

    fn spawn_sprite(
        commands: &mut Commands,
        color: [f32; 4],
        size: Vec2,
        animation: AnimatedSprite,
    ) {
        commands.spawn((
            Transform::from_translation(animation.base),
            GpuSprite { size, color },
            animation,
        ));
    }

    fn animate_scene(
        clock: Res<AnimationClock>,
        mut sprites: Query<(&mut Transform, &AnimatedSprite)>,
    ) {
        for (mut transform, animation) in &mut sprites {
            let t = clock.time * animation.speed + animation.phase;
            transform.translation = animation.base
                + Vec3::new(
                    t.cos() * animation.radius.x,
                    t.sin() * animation.radius.y,
                    0.0,
                );
            transform.rotation = Quat::from_rotation_z(t * animation.spin);
            let scale = 0.92 + 0.12 * (t * 1.7).sin();
            transform.scale = Vec3::splat(scale);
        }
    }

    fn background_vertices() -> Vec<Vertex> {
        let color = [0.04, 0.05, 0.08, 1.0];
        vec![
            Vertex {
                position: [-1.0, -1.0],
                color,
            },
            Vertex {
                position: [1.0, -1.0],
                color,
            },
            Vertex {
                position: [1.0, 1.0],
                color,
            },
            Vertex {
                position: [-1.0, -1.0],
                color,
            },
            Vertex {
                position: [1.0, 1.0],
                color,
            },
            Vertex {
                position: [-1.0, 1.0],
                color,
            },
        ]
    }

    fn append_sprite_vertices(
        vertices: &mut Vec<Vertex>,
        transform: &Transform,
        sprite: &GpuSprite,
    ) {
        let half = sprite.size * 0.5 * transform.scale.truncate();
        let corners = [
            Vec2::new(-half.x, -half.y),
            Vec2::new(half.x, -half.y),
            Vec2::new(half.x, half.y),
            Vec2::new(-half.x, half.y),
        ];
        let rotation = transform.rotation.to_euler(EulerRot::XYZ).2;
        let sin = rotation.sin();
        let cos = rotation.cos();
        let center = transform.translation.truncate();
        let mut points = [[0.0; 2]; 4];

        for (index, corner) in corners.iter().enumerate() {
            let rotated = Vec2::new(
                corner.x * cos - corner.y * sin,
                corner.x * sin + corner.y * cos,
            );
            let point = center + rotated;
            points[index] = [point.x / (WIDTH * 0.5), point.y / (HEIGHT * 0.5)];
        }

        let color = sprite.color;
        vertices.extend_from_slice(&[
            Vertex {
                position: points[0],
                color,
            },
            Vertex {
                position: points[1],
                color,
            },
            Vertex {
                position: points[2],
                color,
            },
            Vertex {
                position: points[0],
                color,
            },
            Vertex {
                position: points[2],
                color,
            },
            Vertex {
                position: points[3],
                color,
            },
        ]);
    }

    #[derive(Default)]
    struct RectangleRenderer {
        format: Option<wgpu::TextureFormat>,
        pipeline: Option<wgpu::RenderPipeline>,
        vertex_buffer: Option<wgpu::Buffer>,
    }

    impl RectangleRenderer {
        fn render(&mut self, ctx: &mut WgpuRenderContext<'_>, vertices: &[Vertex]) {
            if self.pipeline.is_none() || self.format != Some(ctx.target_format) {
                self.create_resources(ctx);
            }
            let Some(pipeline) = self.pipeline.as_ref() else {
                return;
            };
            let Some(buffer) = self.vertex_buffer.as_ref() else {
                return;
            };
            let Some(viewport) =
                viewport(ctx.rect, ctx.surface_size.width, ctx.surface_size.height)
            else {
                return;
            };

            let bytes = bytemuck::cast_slice(vertices);
            ctx.queue.write_buffer(buffer, 0, bytes);

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
                label: Some("widget-gallery-bevy-direct-scene"),
                color_attachments: &[color_attachment],
                ..Default::default()
            });
            pass.set_pipeline(pipeline);
            pass.set_vertex_buffer(0, buffer.slice(..bytes.len() as u64));
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
            pass.draw(0..vertices.len() as u32, 0..1);
        }

        fn create_resources(&mut self, ctx: &WgpuRenderContext<'_>) {
            let shader = ctx
                .device
                .create_shader_module(wgpu::ShaderModuleDescriptor {
                    label: Some("widget-gallery-bevy-direct-shader"),
                    source: wgpu::ShaderSource::Wgsl(SHADER.into()),
                });

            let pipeline_layout =
                ctx.device
                    .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                        label: Some("widget-gallery-bevy-direct-pipeline-layout"),
                        bind_group_layouts: &[],
                        push_constant_ranges: &[],
                    });

            let pipeline = ctx
                .device
                .create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                    label: Some("widget-gallery-bevy-direct-pipeline"),
                    layout: Some(&pipeline_layout),
                    vertex: wgpu::VertexState {
                        module: &shader,
                        entry_point: Some("vs_main"),
                        compilation_options: Default::default(),
                        buffers: &[wgpu::VertexBufferLayout {
                            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                            step_mode: wgpu::VertexStepMode::Vertex,
                            attributes: &wgpu::vertex_attr_array![
                                0 => Float32x2,
                                1 => Float32x4,
                            ],
                        }],
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
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: wgpu::ColorWrites::ALL,
                        })],
                    }),
                    multiview: None,
                    cache: None,
                });

            let vertex_buffer = ctx.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("widget-gallery-bevy-direct-vertices"),
                size: MAX_VERTICES * std::mem::size_of::<Vertex>() as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });

            self.format = Some(ctx.target_format);
            self.pipeline = Some(pipeline);
            self.vertex_buffer = Some(vertex_buffer);
        }
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
}
