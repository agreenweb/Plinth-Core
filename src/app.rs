use crate::graphics::{Graphics, Rc};
use crate::plinth_app::PlinthApp;
use std::cell::RefCell;

// Winit imports (always available since it's the default)
#[cfg(feature = "winit")]
use crate::graphics::create_graphics;
#[cfg(feature = "winit")]
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy},
    window::{Window, WindowId},
};

#[cfg(feature = "web-canvas")]
use crate::web_canvas::{WebCanvas, WebRc};
#[cfg(feature = "web-canvas")]
use crate::graphics::create_graphics_web;
#[cfg(feature = "web-canvas")]
use wasm_bindgen::closure::Closure;
#[cfg(feature = "web-canvas")]
use wasm_bindgen::JsCast;

// Winit-specific types (always available since it's the default)
#[cfg(feature = "winit")]
enum State {
    Ready(Graphics),
    Init(Option<EventLoopProxy<Graphics>>),
}

#[cfg(feature = "winit")]
pub struct App {
    _title: String,
    state: State,
    user_app: Rc<RefCell<dyn PlinthApp>>,
}

#[cfg(feature = "winit")]
impl App {
    pub fn new(event_loop: &EventLoop<Graphics>, user_app: Rc<RefCell<dyn PlinthApp>>) -> Self {
        Self {
            _title: "WebGPU Example".to_string(),
            state: State::Init(Some(event_loop.create_proxy())),
            user_app,
        }
    }

    fn draw(&mut self) {
        if let State::Ready(gfx) = &mut self.state {
            self.user_app.borrow_mut().before_render();
            self.user_app.borrow_mut().render(gfx);
            self.user_app.borrow_mut().after_render();
        }
    }

    fn resized(&mut self, size: PhysicalSize<u32>) {
        if let State::Ready(gfx) = &mut self.state {
            gfx.resize(size);
            self.user_app.borrow_mut().render(gfx);
        }
    }

    pub fn _set_title(&mut self, title: &str) {
        self._title = title.to_string();
    }
}

#[cfg(feature = "winit")]
impl ApplicationHandler<Graphics> for App {
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(size) => self.resized(size),
            WindowEvent::RedrawRequested => self.draw(),
            WindowEvent::CloseRequested => {
                self.user_app.borrow_mut().on_close();
                event_loop.exit()
            }
            _ => {}
        }
        self.user_app
            .borrow_mut()
            .event_handler(event_loop, _window_id, &event);
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let State::Init(proxy) = &mut self.state {
            if let Some(proxy) = proxy.take() {
                let mut win_attr = Window::default_attributes();

                #[cfg(not(target_arch = "wasm32"))]
                {
                    win_attr = win_attr.with_title(self._title.as_str());
                }

                       #[cfg(target_arch = "wasm32")]
                       {
                           use winit::platform::web::WindowAttributesExtWebSys;
                    win_attr = win_attr.with_append(true);
                }

                let window = Rc::new(
                    event_loop
                        .create_window(win_attr)
                        .expect("create window err."),
                );

                let user_app = Rc::clone(&self.user_app);

                #[cfg(target_arch = "wasm32")]
                wasm_bindgen_futures::spawn_local(create_graphics(window, proxy, user_app));

                #[cfg(not(target_arch = "wasm32"))]
                pollster::block_on(create_graphics(window, proxy, user_app));
            }
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, graphics: Graphics) {
        self.state = State::Ready(graphics);
        if let State::Ready(gfx) = &mut self.state {
            if let Some(window) = &gfx.window {
                let scale_factor = window.scale_factor();
                let logical_size = window.inner_size();
                       let physical_size = winit::dpi::PhysicalSize::new(
                    (logical_size.width as f64 * scale_factor) as u32,
                    (logical_size.height as f64 * scale_factor) as u32,
                );
                self.resized(physical_size);
            }
        }
        self.user_app.borrow_mut().init();
    }
}

// Winit-based app functions (always available since it's the default)
#[cfg(feature = "winit")]
pub fn start_app(user_app: Rc<RefCell<dyn PlinthApp>>) {
    let event_loop = EventLoop::<Graphics>::with_user_event().build().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let app = App::new(&event_loop, user_app);
    run_app(event_loop, app);
}

#[cfg(feature = "winit")]
#[cfg(target_arch = "wasm32")]
fn run_app(event_loop: EventLoop<Graphics>, app: App) {
    // Sets up panics to go to the console.error in browser environments
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Error).expect("Couldn't initialize logger");

    // Runs the app async via the browsers event loop
           use winit::platform::web::EventLoopExtWebSys;
    wasm_bindgen_futures::spawn_local(async move {
        event_loop.spawn_app(app);
    });
}

#[cfg(feature = "winit")]
#[cfg(not(target_arch = "wasm32"))]
fn run_app(event_loop: EventLoop<Graphics>, mut app: App) {
    // Allows the setting of the log level through RUST_LOG env var.
    // It also allows wgpu logs to be seen.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error")).init();

    // Runs the app on the current thread.
    let _ = event_loop.run_app(&mut app);
}

// Web canvas-based app functions
#[cfg(feature = "web-canvas")]
pub fn start_app_web(canvas_id: &str, user_app: WebRc<RefCell<dyn PlinthApp>>) -> Result<(), wasm_bindgen::JsValue> {
    let canvas = WebCanvas::new(canvas_id)?;
    let canvas_rc = WebRc::new(canvas);
    
    // Initialize web-specific app logic
    wasm_bindgen_futures::spawn_local(async move {
        let graphics = crate::graphics::create_graphics_web(canvas_rc.clone(), user_app.clone()).await;
        let graphics_rc = WebRc::new(RefCell::new(graphics));
        
        // Create and start the event loop
        let mut event_loop = crate::web_canvas::WebEventLoop::new(
            canvas_rc, 
            graphics_rc, 
            user_app
        );
        
        // Set up event listeners
        setup_web_event_listeners(&event_loop);
        
        // Start the render loop
        event_loop.start();
    });
    
    Ok(())
}

#[cfg(feature = "web-canvas")]
fn setup_web_event_listeners(_event_loop: &crate::web_canvas::WebEventLoop) {
    // Set up resize listener
    // Note: This is a simplified implementation for demonstration purposes
    // In a real implementation, you would need to properly handle event listeners
    // and manage the event loop lifecycle
    
    let window = web_sys::window().unwrap();
    let closure = Closure::wrap(Box::new(move |_event: web_sys::Event| {
        // Handle resize events
        // Note: This would need proper implementation for event handling
    }) as Box<dyn FnMut(web_sys::Event)>);
    
    window.add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref()).unwrap();
    closure.forget();
}
