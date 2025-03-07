use std::borrow::Cow;

use wgpu::{
    Color, CommandEncoderDescriptor, FragmentState, LoadOp, Operations, RenderPassColorAttachment,
    RenderPassDescriptor, RenderPipeline, RenderPipelineDescriptor, ShaderModuleDescriptor,
    ShaderSource, StoreOp, TextureViewDescriptor, VertexState,
};
use winit::{event::WindowEvent, event_loop::ActiveEventLoop, window::WindowId};

use crate::plinth::graphics::Graphics;

pub trait PlinthApp: PlinthRenderer {
    fn init(&mut self) {}
    fn before_render(&mut self) {}
    fn after_render(&mut self) {}
    fn event_handler(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        _event: &WindowEvent,
    ) {
    }
    fn on_close(&mut self) {}
}

pub trait PlinthRenderer {
    fn render(&mut self, gfx: &mut Graphics) {
        let frame = gfx
            .surface
            .get_current_texture()
            .expect("Failed to aquire next swap chain texture.");

        let view = frame.texture.create_view(&TextureViewDescriptor::default());

        let mut encoder = gfx
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        {
            let mut r_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::BLACK),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            r_pass.set_pipeline(&gfx.render_pipelines[0]);
            r_pass.draw(0..3, 0..1);
        } // `r_pass` dropped here

        gfx.queue.submit(Some(encoder.finish()));
        frame.present();
    }
    fn create_pipeline(&mut self, gfx: &mut Graphics) -> RenderPipeline {
        let device = &gfx.device;
        let swap_chain_format = gfx.surface_config.format;
        let shader = device.create_shader_module(ShaderModuleDescriptor {
            label: None,
            source: ShaderSource::Wgsl(Cow::Borrowed(include_str!("shaders/shader.wgsl"))),
        });

        device.create_render_pipeline(&RenderPipelineDescriptor {
            label: None,
            layout: None,
            vertex: VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[],
                compilation_options: Default::default(),
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                targets: &[Some(swap_chain_format.into())],
                compilation_options: Default::default(),
            }),
            primitive: Default::default(),
            depth_stencil: None,
            multisample: Default::default(),
            multiview: None,
            cache: None,
        })
    }
}
