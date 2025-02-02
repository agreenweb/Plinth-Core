use wgpu::{
    Adapter, Color, CommandEncoderDescriptor, Device, DeviceDescriptor, Features, Instance, Limits,
    LoadOp, MemoryHints, Operations, PowerPreference, Queue, RenderPassColorAttachment,
    RenderPassDescriptor, RequestAdapterOptions, StoreOp, Surface, SurfaceConfiguration,
    TextureViewDescriptor,
};
use winit::{dpi::PhysicalSize, event_loop::EventLoopProxy, window::Window};

#[cfg(target_arch = "wasm32")]
pub type Rc<T> = std::rc::Rc<T>;

#[cfg(not(target_arch = "wasm32"))]
pub type Rc<T> = std::sync::Arc<T>;

pub async fn create_graphics(window: Rc<Window>, proxy: EventLoopProxy<Graphics>) {
    let instance = Instance::default();
    let surface = instance.create_surface(Rc::clone(&window)).unwrap();
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(), // Power preference for the device
            force_fallback_adapter: false, // Indicates that only a fallback ("software") adapter can be used
            compatible_surface: Some(&surface), // Guarantee that the adapter can render to this surface
        })
        .await
        .expect("Could not get an adapter (GPU).");

    log::info!("Got adapter: {:?}", adapter.get_info());

    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: None,
                required_features: Features::empty(), // Specifies the required features by the device request. Fails if the adapt er can't provide them.
                required_limits: Limits::downlevel_webgl2_defaults()
                    .using_resolution(adapter.limits()),
                memory_hints: MemoryHints::Performance,
            },
            None,
        )
        .await
        .expect("Failed to get device");

    // Get physical pixel dimensiosn inside the window
    let size = window.inner_size();
    // Make the dimensions at least size 1, otherwise wgpu would panic
    let width = size.width.max(1);
    let height = size.height.max(1);
    let surface_config = surface.get_default_config(&adapter, width, height).unwrap();
    surface.configure(&device, &surface_config);

    let gfx = Graphics {
        window: window.clone(),
        instance,
        surface,
        surface_config,
        adapter,
        device,
        queue,
    };

    let _ = proxy.send_event(gfx);
}

#[derive(Debug)]
pub struct Graphics {
    window: Rc<Window>,
    instance: Instance,
    surface: Surface<'static>,
    surface_config: SurfaceConfiguration,
    adapter: Adapter,
    device: Device,
    queue: Queue,
}

impl Graphics {
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.surface_config.width = new_size.width.max(1);
        self.surface_config.height = new_size.height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }

    pub fn draw(&mut self) {
        let surface_texture = self
            .surface
            .get_current_texture()
            .expect("Failed to aquire next swap chain texture.");

        let texture_view = surface_texture
            .texture
            .create_view(&TextureViewDescriptor::default());

        let mut encoder = self
            .device
            .create_command_encoder(&CommandEncoderDescriptor { label: None });

        {
            let _r_pass = encoder.begin_render_pass(&RenderPassDescriptor {
                label: None,
                color_attachments: &[Some(RenderPassColorAttachment {
                    view: &texture_view,
                    resolve_target: None,
                    ops: Operations {
                        load: LoadOp::Clear(Color::GREEN),
                        store: StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
        } // This is important to drop the render pass

        self.queue.submit(Some(encoder.finish()));
        surface_texture.present();
    }
}
