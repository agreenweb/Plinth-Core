use glam::Vec2;
use wgpu::{Device, RenderPipeline, ShaderModuleDescriptor, ShaderSource};
use std::borrow::Cow;

use crate::types::{Color, Transform};
use crate::batch::InstanceBatch;

#[derive(Debug, Clone)]
pub struct Circle {
    pub center: Vec2,
    pub radius: f32,
    pub color: Color,
    pub transform: Transform,
    pub css_class: Option<String>,
}

impl Circle {
    pub fn new(center: Vec2, radius: f32) -> Self {
        Self {
            center,
            radius,
            color: Color::default(),
            transform: Transform::default(),
            css_class: None,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_css_class(mut self, class: impl Into<String>) -> Self {
        self.css_class = Some(class.into());
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    // This method will be called during render to apply CSS class overrides
    pub fn apply_css_override(&mut self, color: Color) {
        self.color = color;
    }
}

// Instance data for instanced rendering
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct CircleInstance {
    pub center: [f32; 2],
    pub radius: f32,
    pub color: [f32; 4],
    pub transform_position: [f32; 2],
    pub transform_scale: [f32; 2],
    pub transform_rotation: f32,
    pub _padding: f32, // For alignment
}

impl From<&Circle> for CircleInstance {
    fn from(circle: &Circle) -> Self {
        Self {
            center: [circle.center.x, circle.center.y],
            radius: circle.radius,
            color: [circle.color.r, circle.color.g, circle.color.b, circle.color.a],
            transform_position: [circle.transform.position.x, circle.transform.position.y],
            transform_scale: [circle.transform.scale.x, circle.transform.scale.y],
            transform_rotation: circle.transform.rotation,
            _padding: 0.0,
        }
    }
}

pub struct CircleBatch {
    instances: Vec<CircleInstance>,
    instance_buffer: Option<wgpu::Buffer>,
    render_pipeline: Option<RenderPipeline>,
    needs_update: bool,
}

impl CircleBatch {
    pub fn new() -> Self {
        Self {
            instances: Vec::new(),
            instance_buffer: None,
            render_pipeline: None,
            needs_update: true,
        }
    }

    pub fn add_circle(&mut self, circle: Circle) {
        self.instances.push(CircleInstance::from(&circle));
        self.needs_update = true;
    }

    pub fn add_circles(&mut self, circles: impl IntoIterator<Item = Circle>) {
        self.instances.extend(circles.into_iter().map(|c| CircleInstance::from(&c)));
        self.needs_update = true;
    }

    pub fn clear(&mut self) {
        self.instances.clear();
        self.needs_update = true;
    }

    pub fn len(&self) -> usize {
        self.instances.len()
    }

    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    pub fn create_pipeline(&mut self, device: &Device, surface_format: wgpu::TextureFormat) -> RenderPipeline {
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("Circle Shader"),
            source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("../shaders/circle.wgsl"))),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Circle Pipeline Layout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Circle Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<CircleInstance>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Instance,
                    attributes: &[
                        wgpu::VertexAttribute {
                            offset: 0,
                            shader_location: 0,
                            format: wgpu::VertexFormat::Float32x2, // center
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 2]>() as wgpu::BufferAddress,
                            shader_location: 1,
                            format: wgpu::VertexFormat::Float32, // radius
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                            shader_location: 2,
                            format: wgpu::VertexFormat::Float32x4, // color
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 7]>() as wgpu::BufferAddress,
                            shader_location: 3,
                            format: wgpu::VertexFormat::Float32x2, // transform_position
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 9]>() as wgpu::BufferAddress,
                            shader_location: 4,
                            format: wgpu::VertexFormat::Float32x2, // transform_scale
                        },
                        wgpu::VertexAttribute {
                            offset: std::mem::size_of::<[f32; 11]>() as wgpu::BufferAddress,
                            shader_location: 5,
                            format: wgpu::VertexFormat::Float32, // transform_rotation
                        },
                    ],
                }],
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(surface_format.into())],
                compilation_options: Default::default(),
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                unclipped_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        });

        self.render_pipeline = Some(render_pipeline);
        self.render_pipeline.clone().unwrap()
    }

    pub fn update_buffer(&mut self, device: &Device, queue: &wgpu::Queue) {
        if self.needs_update && !self.instances.is_empty() {
            let buffer_size = (self.instances.len() * std::mem::size_of::<CircleInstance>()) as wgpu::BufferAddress;
            
            self.instance_buffer = Some(device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("Circle Instance Buffer"),
                size: buffer_size,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }));

            if let Some(ref buffer) = self.instance_buffer {
                queue.write_buffer(buffer, 0, bytemuck::cast_slice(&self.instances));
            }
            
            self.needs_update = false;
        }
    }

    pub fn render(&self, render_pass: &mut wgpu::RenderPass) {
        if let (Some(ref pipeline), Some(ref buffer), count) = (&self.render_pipeline, &self.instance_buffer, self.instances.len()) {
            if count > 0 {
                render_pass.set_pipeline(pipeline);
                render_pass.set_vertex_buffer(0, buffer.slice(..));
                render_pass.draw(0..6, 0..count as u32); // 6 vertices for a quad, count instances
            }
        }
    }
}

impl Default for CircleBatch {
    fn default() -> Self {
        Self::new()
    }
}

impl InstanceBatch for CircleBatch {
    fn len(&self) -> usize {
        self.instances.len()
    }

    fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }

    fn clear(&mut self) {
        self.instances.clear();
        self.needs_update = true;
    }
}
