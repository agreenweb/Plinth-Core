use std::cell::RefCell;

use wgpu::{
    Adapter, Device, DeviceDescriptor, Features, Instance, Limits, MemoryHints, PowerPreference,
    Queue, RenderPipeline, RequestAdapterOptions, Surface, SurfaceConfiguration,
};
use winit::{dpi::PhysicalSize, event_loop::EventLoopProxy, window::Window};

use crate::plinth::plinth_app::PlinthApp;

#[cfg(target_arch = "wasm32")]
pub type Rc<T> = std::rc::Rc<T>;

#[cfg(not(target_arch = "wasm32"))]
pub type Rc<T> = std::sync::Arc<T>;

pub async fn create_graphics(
    window: Rc<Window>,
    proxy: EventLoopProxy<Graphics>,
    user_app: Rc<RefCell<dyn PlinthApp>>,
) {
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

    let (device, queue) = adapter
        .request_device(
            &DeviceDescriptor {
                label: None,
                required_features: Features::empty(), // Specifies the required features by the device request. Fails if the adapter can't provide them.
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

    #[cfg(not(target_arch = "wasm32"))]
    surface.configure(&device, &surface_config);

    let render_pipelines = vec![];

    let mut gfx = Graphics {
        window: window.clone(),
        _instance: instance,
        surface,
        surface_config,
        _adapter: adapter,
        device,
        queue,
        render_pipelines,
    };

    let render_pipeline = user_app.borrow_mut().create_pipeline(&mut gfx);

    gfx.render_pipelines.push(render_pipeline);

    let _ = proxy.send_event(gfx);
}

#[derive(Debug)]
pub struct Graphics {
    pub window: Rc<Window>,
    pub _instance: Instance,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
    pub _adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub render_pipelines: Vec<RenderPipeline>,
}

impl Graphics {
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.surface_config.width = new_size.width.max(1);
        self.surface_config.height = new_size.height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }
}
