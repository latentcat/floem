use std::f32::consts::TAU;

use bytemuck::{Pod, Zeroable};
use floem_renderer::BackdropBlur;
use peniko::kurbo::{Affine, Point, Rect, Size};
use wgpu::util::DeviceExt;

const SAMPLE_GROUPS: usize = 8;
const GAUSSIAN_TAP_COUNT: usize = 27;
const GAUSSIAN_CENTER_INDEX: usize = 13;
const WEIGHT_CULL_THRESHOLD: f32 = 0.002;
const MIN_VARIANCE: f32 = 0.000_001;

const T8: f32 = 20.0 * 20.0;
const T4: f32 = 9.5 * 9.5;
const T2_SMALL_SIGMA: f32 = 3.6 * 3.5;
const T2_LARGE_SIGMA: f32 = 5.5 * 5.5;
const LARGE_SIGMA_VARIANCE: f32 = 100.0;
const C8: f32 = 0.140_625;
const C4: f32 = 0.472_656_25;
const C2: f32 = 0.756_625;

const SIGMA_PER_RADIUS: f32 = 0.25;

const BACKDROP_BLUR_SHADER: &str = r#"
struct BlurUniform {
    step: vec4<f32>,
    weights0: vec4<f32>,
    weights1: vec4<f32>,
    offsets0: vec4<f32>,
    offsets1: vec4<f32>,
    misc: vec4<f32>,
    rect: vec4<f32>,
    params: vec4<f32>,
};

@group(0) @binding(0) var source_texture: texture_2d<f32>;
@group(0) @binding(1) var source_sampler: sampler;
@group(0) @binding(2) var<uniform> blur_uniform: BlurUniform;

struct VertexOut {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

@vertex
fn vs_main(@location(0) position: vec2<f32>, @location(1) uv: vec2<f32>) -> VertexOut {
    var out: VertexOut;
    out.position = vec4<f32>(position, 0.0, 1.0);
    out.uv = uv;
    return out;
}

fn weight(index: u32) -> f32 {
    if (index == 0u) { return blur_uniform.weights0.x; }
    if (index == 1u) { return blur_uniform.weights0.y; }
    if (index == 2u) { return blur_uniform.weights0.z; }
    if (index == 3u) { return blur_uniform.weights0.w; }
    if (index == 4u) { return blur_uniform.weights1.x; }
    if (index == 5u) { return blur_uniform.weights1.y; }
    if (index == 6u) { return blur_uniform.weights1.z; }
    return blur_uniform.weights1.w;
}

fn sample_offset(index: u32) -> f32 {
    if (index == 0u) { return blur_uniform.offsets0.x; }
    if (index == 1u) { return blur_uniform.offsets0.y; }
    if (index == 2u) { return blur_uniform.offsets0.z; }
    if (index == 3u) { return blur_uniform.offsets0.w; }
    if (index == 4u) { return blur_uniform.offsets1.x; }
    if (index == 5u) { return blur_uniform.offsets1.y; }
    if (index == 6u) { return blur_uniform.offsets1.z; }
    return blur_uniform.offsets1.w;
}

fn blur_sample(uv: vec2<f32>) -> vec4<f32> {
    var color = vec4<f32>(0.0);
    for (var index = 0u; index < 8u; index = index + 1u) {
        let w = weight(index);
        if (w > 0.0) {
            let delta = blur_uniform.step.xy * sample_offset(index);
            color += textureSampleLevel(source_texture, source_sampler, uv + delta, 0.0) * w;
            color += textureSampleLevel(source_texture, source_sampler, uv - delta, 0.0) * w;
        }
    }
    return color;
}

fn rounded_rect_signed_distance(pixel: vec2<f32>, bounds: vec4<f32>, radius_value: f32) -> f32 {
    let bounds_min = bounds.xy;
    let bounds_max = bounds.zw;
    let size = max(bounds_max - bounds_min, vec2<f32>(0.0));
    if (size.x <= 0.0 || size.y <= 0.0) {
        return 1.0;
    }

    let radius = min(max(radius_value, 0.0), min(size.x, size.y) * 0.5);
    let center = (bounds_min + bounds_max) * 0.5;
    let half_size = max(size * 0.5 - vec2<f32>(radius), vec2<f32>(0.0));
    let q = abs(pixel - center) - half_size;
    let outside = length(max(q, vec2<f32>(0.0)));
    let inside = min(max(q.x, q.y), 0.0);
    return outside + inside - radius;
}

fn rounded_rect_coverage(pixel: vec2<f32>) -> f32 {
    let distance = rounded_rect_signed_distance(pixel, blur_uniform.rect, blur_uniform.params.x);
    let aa = max(fwidth(distance), 0.75);
    return 1.0 - smoothstep(0.0, aa, distance);
}

fn composite_rounded(pixel: vec2<f32>, color: vec4<f32>) -> vec4<f32> {
    let coverage = rounded_rect_coverage(pixel);
    if (coverage <= 0.0) {
        discard;
    }
    return vec4<f32>(color.rgb, color.a * coverage);
}

@fragment
fn fs_texture(in: VertexOut) -> @location(0) vec4<f32> {
    return textureSampleLevel(source_texture, source_sampler, in.uv, 0.0);
}

@fragment
fn fs_blur(in: VertexOut) -> @location(0) vec4<f32> {
    return blur_sample(in.uv);
}

@fragment
fn fs_down2(in: VertexOut) -> @location(0) vec4<f32> {
    let source_size = vec2<f32>(textureDimensions(source_texture));
    let pixel = floor(in.position.xy);
    let base = pixel * 2.0 + vec2<f32>(1.0);
    return textureSampleLevel(source_texture, source_sampler, base / source_size, 0.0);
}

@fragment
fn fs_down4(in: VertexOut) -> @location(0) vec4<f32> {
    let source_size = vec2<f32>(textureDimensions(source_texture));
    let pixel = floor(in.position.xy);
    let base = pixel * 4.0 + vec2<f32>(1.0);
    var color = vec4<f32>(0.0);
    color += textureSampleLevel(source_texture, source_sampler, (base + vec2<f32>(0.0, 0.0)) / source_size, 0.0);
    color += textureSampleLevel(source_texture, source_sampler, (base + vec2<f32>(2.0, 0.0)) / source_size, 0.0);
    color += textureSampleLevel(source_texture, source_sampler, (base + vec2<f32>(0.0, 2.0)) / source_size, 0.0);
    color += textureSampleLevel(source_texture, source_sampler, (base + vec2<f32>(2.0, 2.0)) / source_size, 0.0);
    return color * 0.25;
}

@fragment
fn fs_down8(in: VertexOut) -> @location(0) vec4<f32> {
    let source_size = vec2<f32>(textureDimensions(source_texture));
    let pixel = floor(in.position.xy);
    let base = pixel * 8.0 + vec2<f32>(1.0);
    var color = vec4<f32>(0.0);

    for (var y = 0; y < 4; y = y + 1) {
        for (var x = 0; x < 4; x = x + 1) {
            color += textureSampleLevel(
                source_texture,
                source_sampler,
                (base + vec2<f32>(f32(x * 2), f32(y * 2))) / source_size,
                0.0
            );
        }
    }
    return color * 0.0625;
}

@fragment
fn fs_upsample_composite(in: VertexOut) -> @location(0) vec4<f32> {
    let source_size = vec2<f32>(textureDimensions(source_texture));
    let offset = vec2<f32>(blur_uniform.misc.x) / source_size;
    let color = textureSampleLevel(source_texture, source_sampler, in.uv + offset, 0.0);
    return composite_rounded(in.position.xy, color);
}
"#;

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct TextureVertex {
    position: [f32; 2],
    uv: [f32; 2],
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Pod, Zeroable)]
struct BlurUniform {
    step: [f32; 4],
    weights0: [f32; 4],
    weights1: [f32; 4],
    offsets0: [f32; 4],
    offsets1: [f32; 4],
    misc: [f32; 4],
    rect: [f32; 4],
    params: [f32; 4],
}

#[derive(Debug, Clone, Copy)]
pub(crate) struct BackdropBlurCommand {
    pub(crate) rect: Rect,
    pub(crate) radius: f32,
    pub(crate) corner_radius: f32,
}

#[derive(Debug, Clone, Copy)]
struct AppleGaussianSchedule {
    mip_level: u32,
    remaining_variance: f32,
    pass_count: usize,
}

#[derive(Debug, Clone, Copy)]
struct AppleGaussianKernel {
    weights: [f32; SAMPLE_GROUPS],
    offsets: [f32; SAMPLE_GROUPS],
}

#[derive(Debug, Clone, Copy)]
struct AppleGaussianDiagnostics {
    schedule: AppleGaussianSchedule,
    kernel: AppleGaussianKernel,
}

pub(crate) struct BackdropBlurPipeline {
    bind_group_layout: wgpu::BindGroupLayout,
    texture_pipeline: wgpu::RenderPipeline,
    down2_pipeline: wgpu::RenderPipeline,
    down4_pipeline: wgpu::RenderPipeline,
    down8_pipeline: wgpu::RenderPipeline,
    blur_pipeline: wgpu::RenderPipeline,
    blur_composite_pipeline: wgpu::RenderPipeline,
    upsample_pipeline: wgpu::RenderPipeline,
    sampler: wgpu::Sampler,
    fullscreen_vertices: wgpu::Buffer,
}

#[derive(Default)]
pub(crate) struct ScratchTexturePool {
    textures: Vec<ScratchTexture>,
}

struct ScratchTexture {
    texture: wgpu::Texture,
    width: u32,
    height: u32,
    format: wgpu::TextureFormat,
    usage: wgpu::TextureUsages,
    in_use: bool,
}

impl ScratchTexturePool {
    pub(crate) fn begin_frame(&mut self) {
        for texture in &mut self.textures {
            texture.in_use = false;
        }
    }

    pub(crate) fn get(
        &mut self,
        device: &wgpu::Device,
        width: u32,
        height: u32,
        format: wgpu::TextureFormat,
        usage: wgpu::TextureUsages,
        label: &'static str,
    ) -> usize {
        let width = width.max(1);
        let height = height.max(1);
        if let Some((index, texture)) = self.textures.iter_mut().enumerate().find(|(_, texture)| {
            !texture.in_use
                && texture.width == width
                && texture.height == height
                && texture.format == format
                && texture.usage == usage
        }) {
            texture.in_use = true;
            return index;
        }

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some(label),
            size: texture_extent(width, height),
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format,
            usage,
            view_formats: &[],
        });
        self.textures.push(ScratchTexture {
            texture,
            width,
            height,
            format,
            usage,
            in_use: true,
        });
        self.textures.len() - 1
    }

    pub(crate) fn texture(&self, index: usize) -> &wgpu::Texture {
        &self.textures[index].texture
    }
}

impl BackdropBlurPipeline {
    pub(crate) fn new(device: &wgpu::Device, texture_format: wgpu::TextureFormat) -> Self {
        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("floem-backdrop-blur-bind-group-layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("floem-backdrop-blur-pipeline-layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("floem-backdrop-blur-shader"),
            source: wgpu::ShaderSource::Wgsl(BACKDROP_BLUR_SHADER.into()),
        });
        let vertex = wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            compilation_options: Default::default(),
            buffers: &[wgpu::VertexBufferLayout {
                array_stride: std::mem::size_of::<TextureVertex>() as wgpu::BufferAddress,
                step_mode: wgpu::VertexStepMode::Vertex,
                attributes: &wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2],
            }],
        };
        let replace_target = wgpu::ColorTargetState {
            format: texture_format,
            blend: Some(wgpu::BlendState::REPLACE),
            write_mask: wgpu::ColorWrites::ALL,
        };
        let alpha_target = wgpu::ColorTargetState {
            format: texture_format,
            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
            write_mask: wgpu::ColorWrites::ALL,
        };

        let texture_pipeline = create_pipeline(
            device,
            &pipeline_layout,
            &shader,
            vertex.clone(),
            alpha_target.clone(),
            "fs_texture",
            "floem-texture-composite-pipeline",
        );
        let down2_pipeline = create_pipeline(
            device,
            &pipeline_layout,
            &shader,
            vertex.clone(),
            replace_target.clone(),
            "fs_down2",
            "floem-backdrop-down2-pipeline",
        );
        let down4_pipeline = create_pipeline(
            device,
            &pipeline_layout,
            &shader,
            vertex.clone(),
            replace_target.clone(),
            "fs_down4",
            "floem-backdrop-down4-pipeline",
        );
        let down8_pipeline = create_pipeline(
            device,
            &pipeline_layout,
            &shader,
            vertex.clone(),
            replace_target.clone(),
            "fs_down8",
            "floem-backdrop-down8-pipeline",
        );
        let blur_pipeline = create_pipeline(
            device,
            &pipeline_layout,
            &shader,
            vertex.clone(),
            replace_target,
            "fs_blur",
            "floem-backdrop-blur-pipeline",
        );
        let blur_composite_pipeline = create_pipeline(
            device,
            &pipeline_layout,
            &shader,
            vertex.clone(),
            alpha_target.clone(),
            "fs_blur",
            "floem-backdrop-blur-composite-pipeline",
        );
        let upsample_pipeline = create_pipeline(
            device,
            &pipeline_layout,
            &shader,
            vertex,
            alpha_target,
            "fs_upsample_composite",
            "floem-backdrop-upsample-composite-pipeline",
        );
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("floem-backdrop-blur-sampler"),
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Linear,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            ..Default::default()
        });
        let fullscreen_vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("floem-backdrop-blur-fullscreen-vertices"),
            contents: bytemuck::cast_slice(&fullscreen_quad()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            bind_group_layout,
            texture_pipeline,
            down2_pipeline,
            down4_pipeline,
            down8_pipeline,
            blur_pipeline,
            blur_composite_pipeline,
            upsample_pipeline,
            sampler,
            fullscreen_vertices,
        }
    }

    pub(crate) fn encode_clear(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        target_view: &wgpu::TextureView,
    ) {
        let color_attachment = Some(wgpu::RenderPassColorAttachment {
            view: target_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                store: wgpu::StoreOp::Store,
            },
            depth_slice: None,
        });
        let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("floem-backdrop-clear-target"),
            color_attachments: &[color_attachment],
            ..Default::default()
        });
    }

    pub(crate) fn encode_texture_composite(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        source_view: &wgpu::TextureView,
        target_view: &wgpu::TextureView,
    ) {
        self.encode_fullscreen_pass_with_load(
            device,
            encoder,
            source_view,
            target_view,
            &self.texture_pipeline,
            BlurUniform::identity(),
            wgpu::LoadOp::Load,
            "floem-scene-segment-composite-pass",
        );
    }

    pub(crate) fn encode_blur_and_composite(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        source_view: &wgpu::TextureView,
        target_view: &wgpu::TextureView,
        texture_format: wgpu::TextureFormat,
        scratch_textures: &mut ScratchTexturePool,
        command: BackdropBlurCommand,
        surface_size: Size,
    ) {
        if command.rect.area() <= 0.0 || command.radius <= 0.0 {
            return;
        }

        let sigma = command.radius * SIGMA_PER_RADIUS;
        let diagnostics = AppleGaussianDiagnostics::new(sigma);
        if diagnostics.schedule.pass_count == 0 {
            return;
        }

        let width = surface_size.width.max(1.0).round() as u32;
        let height = surface_size.height.max(1.0).round() as u32;
        let (work_width, work_height) = mip_extent(width, height, diagnostics.schedule.mip_level);
        let work_usage = wgpu::TextureUsages::TEXTURE_BINDING
            | wgpu::TextureUsages::RENDER_ATTACHMENT
            | wgpu::TextureUsages::COPY_SRC
            | wgpu::TextureUsages::COPY_DST;
        let blank = BlurUniform::identity();

        if diagnostics.schedule.mip_level == 0 {
            let horizontal = scratch_textures.get(
                device,
                work_width,
                work_height,
                texture_format,
                work_usage,
                "floem-backdrop-blur-horizontal",
            );
            let horizontal_view = scratch_textures
                .texture(horizontal)
                .create_view(&wgpu::TextureViewDescriptor::default());

            self.encode_fullscreen_pass(
                device,
                encoder,
                source_view,
                &horizontal_view,
                &self.blur_pipeline,
                blur_uniform([1.0 / work_width as f32, 0.0], diagnostics, command, 0.0),
                "floem-backdrop-horizontal-pass",
            );
            self.encode_rect_pass(
                device,
                encoder,
                &horizontal_view,
                target_view,
                &self.blur_composite_pipeline,
                blur_uniform([0.0, 1.0 / work_height as f32], diagnostics, command, 0.0),
                command.rect,
                surface_size,
                "floem-backdrop-vertical-composite-pass",
            );
            return;
        }

        let work_a = scratch_textures.get(
            device,
            work_width,
            work_height,
            texture_format,
            work_usage,
            "floem-backdrop-blur-work-a",
        );
        let work_a_view = scratch_textures
            .texture(work_a)
            .create_view(&wgpu::TextureViewDescriptor::default());
        let work_b = scratch_textures.get(
            device,
            work_width,
            work_height,
            texture_format,
            work_usage,
            "floem-backdrop-blur-work-b",
        );
        let work_b_view = scratch_textures
            .texture(work_b)
            .create_view(&wgpu::TextureViewDescriptor::default());

        match diagnostics.schedule.mip_level {
            1 => self.encode_fullscreen_pass(
                device,
                encoder,
                source_view,
                &work_a_view,
                &self.down2_pipeline,
                blank,
                "floem-backdrop-down2-pass",
            ),
            2 => self.encode_fullscreen_pass(
                device,
                encoder,
                source_view,
                &work_a_view,
                &self.down4_pipeline,
                blank,
                "floem-backdrop-down4-pass",
            ),
            3 => self.encode_fullscreen_pass(
                device,
                encoder,
                source_view,
                &work_a_view,
                &self.down8_pipeline,
                blank,
                "floem-backdrop-down8-pass",
            ),
            _ => {
                let (down8_width, down8_height) = mip_extent(width, height, 3);
                let down8 = scratch_textures.get(
                    device,
                    down8_width,
                    down8_height,
                    texture_format,
                    work_usage,
                    "floem-backdrop-down8-temp",
                );
                let down8_view = scratch_textures
                    .texture(down8)
                    .create_view(&wgpu::TextureViewDescriptor::default());
                self.encode_fullscreen_pass(
                    device,
                    encoder,
                    source_view,
                    &down8_view,
                    &self.down8_pipeline,
                    blank,
                    "floem-backdrop-down8-pass",
                );
                self.encode_fullscreen_pass(
                    device,
                    encoder,
                    &down8_view,
                    &work_a_view,
                    &self.down2_pipeline,
                    blank,
                    "floem-backdrop-down2-after-down8-pass",
                );
            }
        }

        self.encode_fullscreen_pass(
            device,
            encoder,
            &work_a_view,
            &work_b_view,
            &self.blur_pipeline,
            blur_uniform([1.0 / work_width as f32, 0.0], diagnostics, command, 0.0),
            "floem-backdrop-horizontal-pass",
        );
        self.encode_fullscreen_pass(
            device,
            encoder,
            &work_b_view,
            &work_a_view,
            &self.blur_pipeline,
            blur_uniform([0.0, 1.0 / work_height as f32], diagnostics, command, 0.0),
            "floem-backdrop-vertical-pass",
        );
        self.encode_rect_pass(
            device,
            encoder,
            &work_a_view,
            target_view,
            &self.upsample_pipeline,
            blur_uniform([0.0, 0.0], diagnostics, command, 0.0),
            command.rect,
            surface_size,
            "floem-backdrop-upsample-composite-pass",
        );
    }

    fn encode_fullscreen_pass(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        source_view: &wgpu::TextureView,
        target_view: &wgpu::TextureView,
        pipeline: &wgpu::RenderPipeline,
        uniform: BlurUniform,
        label: &'static str,
    ) {
        self.encode_fullscreen_pass_with_load(
            device,
            encoder,
            source_view,
            target_view,
            pipeline,
            uniform,
            wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
            label,
        );
    }

    fn encode_fullscreen_pass_with_load(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        source_view: &wgpu::TextureView,
        target_view: &wgpu::TextureView,
        pipeline: &wgpu::RenderPipeline,
        uniform: BlurUniform,
        load: wgpu::LoadOp<wgpu::Color>,
        label: &'static str,
    ) {
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("floem-backdrop-blur-uniform"),
            contents: bytemuck::bytes_of(&uniform),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let bind_group = self.create_bind_group(device, source_view, &uniform_buffer);
        let color_attachment = Some(wgpu::RenderPassColorAttachment {
            view: target_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load,
                store: wgpu::StoreOp::Store,
            },
            depth_slice: None,
        });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(label),
            color_attachments: &[color_attachment],
            ..Default::default()
        });
        render_pass.set_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.set_vertex_buffer(0, self.fullscreen_vertices.slice(..));
        render_pass.draw(0..6, 0..1);
    }

    fn encode_rect_pass(
        &self,
        device: &wgpu::Device,
        encoder: &mut wgpu::CommandEncoder,
        source_view: &wgpu::TextureView,
        target_view: &wgpu::TextureView,
        pipeline: &wgpu::RenderPipeline,
        uniform: BlurUniform,
        rect: Rect,
        surface_size: Size,
        label: &'static str,
    ) {
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("floem-backdrop-composite-uniform"),
            contents: bytemuck::bytes_of(&uniform),
            usage: wgpu::BufferUsages::UNIFORM,
        });
        let bind_group = self.create_bind_group(device, source_view, &uniform_buffer);
        let vertices = rect_vertices(rect, surface_size);
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("floem-backdrop-composite-vertices"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let color_attachment = Some(wgpu::RenderPassColorAttachment {
            view: target_view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Load,
                store: wgpu::StoreOp::Store,
            },
            depth_slice: None,
        });
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some(label),
            color_attachments: &[color_attachment],
            ..Default::default()
        });
        render_pass.set_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[]);
        render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        render_pass.draw(0..6, 0..1);
    }

    fn create_bind_group(
        &self,
        device: &wgpu::Device,
        source_view: &wgpu::TextureView,
        uniform_buffer: &wgpu::Buffer,
    ) -> wgpu::BindGroup {
        device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("floem-backdrop-blur-bind-group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(source_view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: uniform_buffer.as_entire_binding(),
                },
            ],
        })
    }
}

impl BlurUniform {
    fn identity() -> Self {
        Self {
            step: [0.0; 4],
            weights0: [0.5, 0.0, 0.0, 0.0],
            weights1: [0.0; 4],
            offsets0: [0.0; 4],
            offsets1: [0.0; 4],
            misc: [0.0; 4],
            rect: [0.0; 4],
            params: [0.0; 4],
        }
    }
}

impl AppleGaussianDiagnostics {
    fn new(sigma: f32) -> Self {
        let schedule = apple_gaussian_schedule(sigma);
        let kernel = apple_gaussian_kernel(schedule.remaining_variance);
        Self { schedule, kernel }
    }
}

pub(crate) fn backdrop_blur_command(
    blur: BackdropBlur,
    transform: Affine,
) -> Option<BackdropBlurCommand> {
    if blur.radius <= 0.0 || blur.rect.area() <= 0.0 {
        return None;
    }

    let rect = transform_rect(blur.rect, transform);
    if rect.area() <= 0.0 {
        return None;
    }

    let scale = transform_scale(transform).max(0.001);
    Some(BackdropBlurCommand {
        rect,
        radius: (blur.radius as f32 * scale).max(0.0),
        corner_radius: (blur.corner_radius as f32 * scale).max(0.0),
    })
}

fn apple_gaussian_schedule(sigma: f32) -> AppleGaussianSchedule {
    if !sigma.is_finite() || sigma <= 0.0 {
        return AppleGaussianSchedule {
            mip_level: 0,
            remaining_variance: 0.0,
            pass_count: 0,
        };
    }

    let original_variance = sigma * sigma;
    let mut remaining_variance = original_variance;
    let mut mip_level = 0;
    let t2 = if original_variance > LARGE_SIGMA_VARIANCE {
        T2_LARGE_SIGMA
    } else {
        T2_SMALL_SIGMA
    };

    if remaining_variance > T8 {
        remaining_variance = remaining_variance / 64.0 - C8;
        mip_level = 3;
    }

    if remaining_variance >= T4 && mip_level == 0 {
        remaining_variance = remaining_variance / 16.0 - C4;
        mip_level = 2;
    }

    if remaining_variance >= t2 {
        remaining_variance = remaining_variance / 4.0 - C2;
        if mip_level >= 1 {
            mip_level += 1;
        } else {
            mip_level = 1;
        }
    }

    let pass_count = match mip_level {
        0 => 2,
        4 => 5,
        _ => 4,
    };

    AppleGaussianSchedule {
        mip_level,
        remaining_variance: remaining_variance.max(MIN_VARIANCE),
        pass_count,
    }
}

fn apple_gaussian_kernel(variance: f32) -> AppleGaussianKernel {
    if !variance.is_finite() || variance <= MIN_VARIANCE {
        let mut weights = [0.0; SAMPLE_GROUPS];
        weights[0] = 0.5;
        return AppleGaussianKernel {
            weights,
            offsets: [0.0; SAMPLE_GROUPS],
        };
    }

    let mut gaussian_kernel = [0.0; GAUSSIAN_TAP_COUNT];
    let mut weight_sum = 0.0;
    let coefficient = 1.0 / (TAU * variance).sqrt();

    for (index, weight) in gaussian_kernel.iter_mut().enumerate() {
        let distance = index as f32 - GAUSSIAN_CENTER_INDEX as f32;
        *weight = coefficient * (-(distance * distance) * 0.5 / variance).exp();
        weight_sum += *weight;
    }

    if weight_sum > 0.0 {
        for weight in &mut gaussian_kernel {
            *weight /= weight_sum;
        }
    }

    gaussian_kernel[GAUSSIAN_CENTER_INDEX] *= 0.5;

    let mut weights = [0.0; SAMPLE_GROUPS];
    let mut offsets = [0.0; SAMPLE_GROUPS];

    for group in 1..7 {
        let distance = group * 2;
        let near_index = GAUSSIAN_CENTER_INDEX - distance;
        let far_index = near_index - 1;
        let near_weight = gaussian_kernel[near_index];
        let far_weight = gaussian_kernel[far_index];
        let combined_weight = near_weight + far_weight;

        if combined_weight >= WEIGHT_CULL_THRESHOLD {
            weights[group] = combined_weight;
            offsets[group] = distance as f32 + far_weight / combined_weight;
        }
    }

    weights[0] = (0.5 - weights[1..7].iter().sum::<f32>()).max(0.0);

    let center_weight = gaussian_kernel[GAUSSIAN_CENTER_INDEX];
    let first_weight = gaussian_kernel[GAUSSIAN_CENTER_INDEX - 1];
    let center_pair_weight = center_weight + first_weight;
    if center_pair_weight > 0.0 {
        offsets[0] = first_weight / center_pair_weight;
    }

    AppleGaussianKernel { weights, offsets }
}

fn blur_uniform(
    step: [f32; 2],
    diagnostics: AppleGaussianDiagnostics,
    command: BackdropBlurCommand,
    upsample_offset: f32,
) -> BlurUniform {
    let weights = diagnostics.kernel.weights;
    let offsets = diagnostics.kernel.offsets;
    BlurUniform {
        step: [step[0], step[1], 0.0, 0.0],
        weights0: [weights[0], weights[1], weights[2], weights[3]],
        weights1: [weights[4], weights[5], weights[6], weights[7]],
        offsets0: [offsets[0], offsets[1], offsets[2], offsets[3]],
        offsets1: [offsets[4], offsets[5], offsets[6], offsets[7]],
        misc: [upsample_offset, 0.0, 0.0, 0.0],
        rect: rect_to_array(command.rect),
        params: [command.corner_radius, 0.0, 0.0, 0.0],
    }
}

fn create_pipeline(
    device: &wgpu::Device,
    layout: &wgpu::PipelineLayout,
    shader: &wgpu::ShaderModule,
    vertex: wgpu::VertexState<'_>,
    color_target: wgpu::ColorTargetState,
    fragment_entry: &'static str,
    label: &'static str,
) -> wgpu::RenderPipeline {
    device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some(label),
        layout: Some(layout),
        vertex,
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            ..Default::default()
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: shader,
            entry_point: Some(fragment_entry),
            compilation_options: Default::default(),
            targets: &[Some(color_target)],
        }),
        multiview: None,
        cache: None,
    })
}

fn texture_extent(width: u32, height: u32) -> wgpu::Extent3d {
    wgpu::Extent3d {
        width: width.max(1),
        height: height.max(1),
        depth_or_array_layers: 1,
    }
}

fn mip_extent(width: u32, height: u32, mip_level: u32) -> (u32, u32) {
    let factor = 1_u32 << mip_level;
    ((width / factor).max(1), (height / factor).max(1))
}

fn fullscreen_quad() -> [TextureVertex; 6] {
    [
        TextureVertex {
            position: [-1.0, 1.0],
            uv: [0.0, 0.0],
        },
        TextureVertex {
            position: [-1.0, -1.0],
            uv: [0.0, 1.0],
        },
        TextureVertex {
            position: [1.0, 1.0],
            uv: [1.0, 0.0],
        },
        TextureVertex {
            position: [1.0, 1.0],
            uv: [1.0, 0.0],
        },
        TextureVertex {
            position: [-1.0, -1.0],
            uv: [0.0, 1.0],
        },
        TextureVertex {
            position: [1.0, -1.0],
            uv: [1.0, 1.0],
        },
    ]
}

fn rect_vertices(rect: Rect, surface_size: Size) -> [TextureVertex; 6] {
    let x0 = rect.min_x() as f32;
    let x1 = rect.max_x() as f32;
    let y0 = rect.min_y() as f32;
    let y1 = rect.max_y() as f32;
    let p0 = textured_vertex(x0, y0, surface_size);
    let p1 = textured_vertex(x0, y1, surface_size);
    let p2 = textured_vertex(x1, y0, surface_size);
    let p3 = textured_vertex(x1, y1, surface_size);
    [p0, p1, p2, p2, p1, p3]
}

fn textured_vertex(x: f32, y: f32, surface_size: Size) -> TextureVertex {
    let width = surface_size.width.max(1.0) as f32;
    let height = surface_size.height.max(1.0) as f32;
    TextureVertex {
        position: [(x / width) * 2.0 - 1.0, 1.0 - (y / height) * 2.0],
        uv: [x / width, y / height],
    }
}

fn rect_to_array(rect: Rect) -> [f32; 4] {
    [
        rect.min_x() as f32,
        rect.min_y() as f32,
        rect.max_x() as f32,
        rect.max_y() as f32,
    ]
}

fn transform_rect(rect: Rect, transform: Affine) -> Rect {
    let points = [
        transform * Point::new(rect.x0, rect.y0),
        transform * Point::new(rect.x1, rect.y0),
        transform * Point::new(rect.x0, rect.y1),
        transform * Point::new(rect.x1, rect.y1),
    ];
    let mut x0 = f64::INFINITY;
    let mut y0 = f64::INFINITY;
    let mut x1 = f64::NEG_INFINITY;
    let mut y1 = f64::NEG_INFINITY;
    for point in points {
        x0 = x0.min(point.x);
        y0 = y0.min(point.y);
        x1 = x1.max(point.x);
        y1 = y1.max(point.y);
    }
    Rect::new(x0, y0, x1, y1)
}

fn transform_scale(transform: Affine) -> f32 {
    let origin = transform * Point::ZERO;
    let x = transform * Point::new(1.0, 0.0);
    let y = transform * Point::new(0.0, 1.0);
    let sx = ((x.x - origin.x).powi(2) + (x.y - origin.y).powi(2)).sqrt();
    let sy = ((y.x - origin.x).powi(2) + (y.y - origin.y).powi(2)).sqrt();
    (((sx + sy) * 0.5) as f32).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn apple_schedule_matches_reference_thresholds() {
        let cases = [(1.0, 0), (4.0, 1), (10.0, 2), (22.0, 3), (50.0, 4)];
        for (sigma, mip_level) in cases {
            assert_eq!(apple_gaussian_schedule(sigma).mip_level, mip_level);
        }
    }

    #[test]
    fn apple_kernel_preserves_unit_energy() {
        let kernel = apple_gaussian_kernel(9.0);
        let energy = kernel.weights.iter().sum::<f32>() * 2.0;
        assert!((energy - 1.0).abs() < 0.000_01);
    }
}
