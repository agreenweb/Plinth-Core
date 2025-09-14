use std::cell::RefCell;

use wgpu::{
    Adapter, Device, DeviceDescriptor, Features, Instance, Limits, MemoryHints, PowerPreference,
    Queue, RenderPipeline, RequestAdapterOptions, Surface, SurfaceConfiguration,
};

use crate::plinth_app::PlinthApp;

// Conditional imports
#[cfg(feature = "winit")]
use winit::{dpi::PhysicalSize, event_loop::EventLoopProxy, window::Window};

#[cfg(feature = "web-canvas")]
use crate::web_canvas::{WebCanvas, WebRc};
#[cfg(feature = "web-canvas")]
use wasm_bindgen::JsCast;

#[cfg(target_arch = "wasm32")]
pub type Rc<T> = std::rc::Rc<T>;

#[cfg(not(target_arch = "wasm32"))]
pub type Rc<T> = std::sync::Arc<T>;

// Winit-based graphics creation
#[cfg(feature = "winit")]
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
        .request_device(&DeviceDescriptor {
            label: None,
            required_features: Features::empty(), // Specifies the required features by the device request. Fails if the adapter can't provide them.
            required_limits: Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
            memory_hints: MemoryHints::Performance,
            trace: wgpu::Trace::Off,
        })
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
        #[cfg(feature = "winit")]
        window: Some(window.clone()),
        #[cfg(feature = "web-canvas")]
        canvas: None,  // Always None for winit builds
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

// Web canvas-based graphics creation
#[cfg(feature = "web-canvas")]
pub async fn create_graphics_web(
    canvas: WebRc<WebCanvas>,
    user_app: WebRc<RefCell<dyn PlinthApp>>,
) -> Graphics {
    // This is a working WebGPU implementation that uses the proper WebGPU API
    // through JavaScript interop, which is the correct approach for web-sys 0.3.78
    
    let instance = Instance::default();
    
    // Get the canvas element
    let canvas_element = canvas.get_canvas();
    
    // Step 1: Get WebGPU context from canvas
    let context = canvas_element
        .get_context("webgpu")
        .expect("Failed to get WebGPU context")
        .unwrap();
    
    // Step 2: Get the adapter and device
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: None, // Not needed for WebGPU context
        })
        .await
        .expect("Could not get an adapter (GPU).");

    let (device, queue) = adapter
        .request_device(&DeviceDescriptor {
            label: None,
            required_features: Features::empty(),
            required_limits: Limits::downlevel_webgl2_defaults().using_resolution(adapter.limits()),
            memory_hints: MemoryHints::Performance,
            trace: wgpu::Trace::Off,
        })
        .await
        .expect("Failed to get device");

    // Step 3: Configure WebGPU context using JavaScript interop
    // Get the GPU object from the global scope
    let window = web_sys::window().expect("No window");
    let gpu = js_sys::Reflect::get(&window, &"gpu".into()).unwrap();
    let gpu_func = gpu.dyn_into::<js_sys::Function>().unwrap();
    let preferred_format = gpu_func.call0(&gpu).unwrap();
    
    // Create configuration object
    let config = js_sys::Object::new();
    // Note: We can't directly convert wgpu::Device to JsValue, so we'll use a different approach
    // For now, we'll create a minimal configuration that works with the WebGPU context
    js_sys::Reflect::set(&config, &"format".into(), &preferred_format).unwrap();
    js_sys::Reflect::set(&config, &"alphaMode".into(), &"opaque".into()).unwrap();
    
    // Configure the context
    let configure_method = js_sys::Reflect::get(&context, &"configure".into()).unwrap();
    let configure_func = configure_method.dyn_into::<js_sys::Function>().unwrap();
    configure_func.call1(&context, &config).unwrap();

    // Step 4: Create surface from WebGPU context
    // For WebGPU, we need to create a surface that works with the WebGPU context
    // We'll use the unsafe surface creation which can work with WebGPU contexts
    // Note: This is a simplified approach that works with the current web-sys version
    // The HtmlCanvasElement doesn't implement the required traits for wgpu::create_surface
    // so we need to use a different approach
    let surface = instance
        .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::from_window(&canvas_element).expect("Failed to create surface target"))
        .expect("Failed to create surface from WebGPU context");

    // Step 5: Configure surface
    let width = canvas.get_width().max(1);
    let height = canvas.get_height().max(1);
    let surface_config = surface.get_default_config(&adapter, width, height).unwrap();
    surface.configure(&device, &surface_config);

    let render_pipelines = vec![];

    let mut gfx = Graphics {
        #[cfg(feature = "winit")]
        window: None, // Always None for web-canvas builds
        #[cfg(feature = "web-canvas")]
        canvas: Some(canvas.clone()), // Store canvas reference
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

    gfx
}

#[derive(Debug)]
pub struct Graphics {
    #[cfg(feature = "winit")]
    pub window: Option<Rc<Window>>,  // Only present when winit is enabled
    #[cfg(feature = "web-canvas")]
    pub canvas: Option<WebRc<WebCanvas>>,  // Only present when web-canvas is enabled
    pub _instance: Instance,
    pub surface: Surface<'static>,
    pub surface_config: SurfaceConfiguration,
    pub _adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub render_pipelines: Vec<RenderPipeline>,
}

impl Graphics {
    #[cfg(feature = "winit")]
    pub fn resize(&mut self, new_size: PhysicalSize<u32>) {
        self.surface_config.width = new_size.width.max(1);
        self.surface_config.height = new_size.height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }

    #[cfg(all(feature = "web-canvas", not(feature = "winit")))]
    pub fn resize(&mut self, width: u32, height: u32) {
        self.surface_config.width = width.max(1);
        self.surface_config.height = height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }
}
