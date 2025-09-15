use wgpu::{Device, Queue, Surface, SurfaceConfiguration, RenderPassDescriptor, RenderPassColorAttachment, Operations, LoadOp, StoreOp, Color};

use crate::primitives::CircleBatch;

pub struct PrimitiveRenderer {
    circle_batch: CircleBatch,
    surface_format: wgpu::TextureFormat,
}

impl PrimitiveRenderer {
    pub fn new(device: &Device, surface_format: wgpu::TextureFormat) -> Self {
        let mut circle_batch = CircleBatch::new();
        circle_batch.create_pipeline(device, surface_format);
        
        Self {
            circle_batch,
            surface_format,
        }
    }

    pub fn add_circle(&mut self, circle: crate::primitives::Circle) {
        self.circle_batch.add_circle(circle);
    }

    pub fn add_circles(&mut self, circles: impl IntoIterator<Item = crate::primitives::Circle>) {
        self.circle_batch.add_circles(circles);
    }

    pub fn clear_circles(&mut self) {
        self.circle_batch.clear();
    }

    pub fn render(&mut self, device: &Device, queue: &Queue, surface: &Surface, _surface_config: &SurfaceConfiguration) {
        // Update instance buffers
        self.circle_batch.update_buffer(device, queue);

        let frame = surface.get_current_texture().expect("Failed to acquire next swap chain texture");
        let view = frame.texture.create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        {
            let mut render_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::TRANSPARENT),
                        store: StoreOp::Store,
                    },
                    depth_slice: None,
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            // Render all primitive batches
            self.circle_batch.render(&mut render_pass);
        }

        queue.submit(Some(encoder.finish()));
        frame.present();
    }

    pub fn get_circle_batch_mut(&mut self) -> &mut CircleBatch {
        &mut self.circle_batch
    }
}
