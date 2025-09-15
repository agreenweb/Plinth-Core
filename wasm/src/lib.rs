use plinth_core::{plinth_app::PlinthApp, plinth_app::PlinthRenderer, web_canvas::{WebCanvas, WebEventLoop, WebRc}};
use std::cell::RefCell;
use wasm_bindgen::prelude::*;

// Import the `console.log` function from the `console` module
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// Define a macro to make console.log work like println!
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

struct TestApp {
    frame_count: u32,
}

impl TestApp {
    fn new() -> Self {
        Self { frame_count: 0 }
    }
}

impl PlinthApp for TestApp {
    fn init(&mut self) {
        console_log!("Test app initialized!");
    }

    fn before_render(&mut self) {
        self.frame_count += 1;
        if self.frame_count % 60 == 0 {
            console_log!("Frame: {}", self.frame_count);
        }
    }

    fn after_render(&mut self) {
        // Optional: Add any post-render logic here
    }

    fn on_close(&mut self) {
        console_log!("Test app closing after {} frames", self.frame_count);
    }
}

impl PlinthRenderer for TestApp {
    // The default render implementation from the trait will render the triangle
    // We can override it if we want custom rendering
}

#[wasm_bindgen]
pub struct WasmApp {
    event_loop: Option<WebEventLoop>,
}

#[wasm_bindgen]
impl WasmApp {
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmApp {
        // Set up panic hook for better error messages
        console_error_panic_hook::set_once();
        // Only initialize console_log once globally
        let _ = console_log::init();
        
        WasmApp {
            event_loop: None,
        }
    }

    #[wasm_bindgen]
    pub async fn new_with_canvas(canvas_id: &str) -> Result<WasmApp, JsValue> {
        console_error_panic_hook::set_once();
        let _ = console_log::init();
        
        let mut app = WasmApp {
            event_loop: None,
        };
        
        app.start(canvas_id).await?;
        Ok(app)
    }

    #[wasm_bindgen]
    pub async fn start(&mut self, canvas_id: &str) -> Result<(), JsValue> {
        console_log!("Starting WASM app with canvas: {}", canvas_id);
        
        // Create the canvas
        let canvas = WebCanvas::new(canvas_id)?;
        let canvas_rc: WebRc<WebCanvas> = WebRc::new(canvas);
        
        // Create the test app
        let test_app = TestApp::new();
        let app_rc: WebRc<RefCell<dyn PlinthApp>> = WebRc::new(RefCell::new(test_app));
        
        // Create graphics
        let graphics = plinth_core::graphics::create_graphics_web(canvas_rc.clone(), app_rc.clone()).await;
        let graphics_rc: WebRc<RefCell<plinth_core::graphics::Graphics>> = WebRc::new(RefCell::new(graphics));
        
        // Create and start the event loop
        let mut event_loop = WebEventLoop::new(canvas_rc, graphics_rc, app_rc);
        event_loop.start();
        
        self.event_loop = Some(event_loop);
        
        console_log!("WASM app started successfully!");
        Ok(())
    }

    #[wasm_bindgen]
    pub fn stop(&mut self) {
        if let Some(mut event_loop) = self.event_loop.take() {
            event_loop.stop();
            console_log!("WASM app stopped");
        }
        // Reset the event loop to None to allow restart
        self.event_loop = None;
    }
}
