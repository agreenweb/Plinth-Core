use std::cell::RefCell;

use wgpu::{
    Adapter, Device, DeviceDescriptor, Features, Instance, Limits, MemoryHints, PowerPreference,
    Queue, RenderPipeline, RequestAdapterOptions, Surface, SurfaceConfiguration, SurfaceTargetUnsafe,
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
    let instance = Instance::default();
    
    // Get the canvas element
    let canvas_element = canvas.get_canvas();
    
    // Step 1: Create a wrapper that implements the required traits
    struct CanvasWrapper<'a>(&'a web_sys::HtmlCanvasElement);
    
    // SAFETY: This is only used on WASM targets where there's only one thread
    unsafe impl<'a> Send for CanvasWrapper<'a> {}
    unsafe impl<'a> Sync for CanvasWrapper<'a> {}
    
    impl<'a> raw_window_handle::HasWindowHandle for CanvasWrapper<'a> {
        fn window_handle(&self) -> Result<raw_window_handle::WindowHandle<'_>, raw_window_handle::HandleError> {
            use raw_window_handle::WebCanvasWindowHandle;
            use std::ptr::NonNull;
            use std::ffi::c_void;
            
            // Get the canvas as a raw pointer
            let canvas_ptr = self.0.as_ref() as *const web_sys::HtmlCanvasElement as *mut c_void;
            let handle = WebCanvasWindowHandle::new(NonNull::new(canvas_ptr).unwrap());
            Ok(unsafe { raw_window_handle::WindowHandle::borrow_raw(handle.into()) })
        }
    }
    
    impl<'a> raw_window_handle::HasDisplayHandle for CanvasWrapper<'a> {
        fn display_handle(&self) -> Result<raw_window_handle::DisplayHandle<'_>, raw_window_handle::HandleError> {
            use raw_window_handle::WebDisplayHandle;
            let handle = WebDisplayHandle::new();
            Ok(unsafe { raw_window_handle::DisplayHandle::borrow_raw(handle.into()) })
        }
    }
    
    let canvas_wrapper = CanvasWrapper(&canvas_element);
    
    // Step 2: Create SurfaceTargetUnsafe from the wrapper
    let surface_target = unsafe { SurfaceTargetUnsafe::from_window(&canvas_wrapper) }
        .expect("Failed to create SurfaceTargetUnsafe from canvas");
    
    // Step 3: Create surface using the unsafe method
    let surface = unsafe {
        instance.create_surface_unsafe(surface_target)
    }.expect("Failed to create surface from canvas");
    
    // Step 2: Get the adapter and device
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
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

    // Step 3: Configure surface
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

    #[cfg(feature = "web-canvas")]
    pub fn resize_web(&mut self, new_size: crate::web_canvas::WebSize) {
        self.surface_config.width = new_size.width.max(1);
        self.surface_config.height = new_size.height.max(1);
        self.surface.configure(&self.device, &self.surface_config);
    }
}
