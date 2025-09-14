#[cfg(feature = "web-canvas")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "web-canvas")]
use web_sys::HtmlCanvasElement;
#[cfg(feature = "web-canvas")]
use std::rc::Rc;
#[cfg(feature = "web-canvas")]
use std::cell::RefCell;

#[cfg(feature = "web-canvas")]
#[derive(Debug, Clone, Copy)]
pub struct WebSize {
    pub width: u32,
    pub height: u32,
}

#[cfg(feature = "web-canvas")]
impl WebSize {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

#[cfg(feature = "web-canvas")]
#[derive(Debug)]
pub struct WebCanvas {
    pub canvas: HtmlCanvasElement,
    pub width: u32,
    pub height: u32,
}

#[cfg(feature = "web-canvas")]
impl WebCanvas {
    pub fn new(canvas_id: &str) -> Result<Self, JsValue> {
        let window = web_sys::window().ok_or("No window")?;
        let document = window.document().ok_or("No document")?;
        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or("Canvas not found")?
            .dyn_into::<HtmlCanvasElement>()
            .map_err(|_| "Element is not a canvas")?;

        let width = canvas.client_width() as u32;
        let height = canvas.client_height() as u32;

        Ok(WebCanvas {
            canvas,
            width,
            height,
        })
    }

    pub fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
        self.canvas.set_width(width);
        self.canvas.set_height(height);
    }

    pub fn get_canvas(&self) -> &HtmlCanvasElement {
        &self.canvas
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }
}

#[cfg(feature = "web-canvas")]
pub type WebRc<T> = Rc<T>;

#[cfg(not(feature = "web-canvas"))]
pub type WebRc<T> = ();

// Web-specific event handling and render loop
#[cfg(feature = "web-canvas")]
pub struct WebEventLoop {
    canvas: Rc<WebCanvas>,
    graphics: Option<Rc<RefCell<crate::graphics::Graphics>>>,
    user_app: Rc<RefCell<dyn crate::plinth_app::PlinthApp>>,
    animation_frame_id: Option<i32>,
}

#[cfg(feature = "web-canvas")]
impl WebEventLoop {
    pub fn new(
        canvas: Rc<WebCanvas>, 
        graphics: Rc<RefCell<crate::graphics::Graphics>>,
        user_app: Rc<RefCell<dyn crate::plinth_app::PlinthApp>>
    ) -> Self {
        Self { 
            canvas, 
            graphics: Some(graphics),
            user_app,
            animation_frame_id: None,
        }
    }

    pub fn start(&mut self) {
        self.request_redraw();
    }

    pub fn stop(&mut self) {
        if let Some(id) = self.animation_frame_id.take() {
            let window = web_sys::window().unwrap();
            window.cancel_animation_frame(id).ok();
        }
    }

    fn request_redraw(&mut self) {
        let graphics = self.graphics.clone().unwrap();
        let user_app: Rc<RefCell<dyn crate::plinth_app::PlinthApp>> = Rc::clone(&self.user_app);
        let canvas = Rc::clone(&self.canvas);
        
        // Create closure for requestAnimationFrame
        let closure = Closure::wrap(Box::new(move || {
            // This is the render loop - parallel to winit's draw() method
            let mut gfx = graphics.borrow_mut();
            user_app.borrow_mut().before_render();
            user_app.borrow_mut().render(&mut gfx);
            user_app.borrow_mut().after_render();
        }) as Box<dyn FnMut()>);

        let window = web_sys::window().unwrap();
        let id = window
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .expect("Failed to request animation frame");
        
        self.animation_frame_id = Some(id);
        closure.forget(); // Prevent closure from being dropped
    }

    // Handle canvas resize (parallel to winit's resized method)
    pub fn handle_resize(&mut self, width: u32, height: u32) {
        // Note: Canvas resize would need proper implementation
        // For now, we'll just update the graphics context if available
        
        if let Some(graphics) = &self.graphics {
            let mut gfx = graphics.borrow_mut();
            gfx.resize_web(WebSize::new(width, height));
        }
    }

    // Handle web events (parallel to winit's event_handler)
    pub fn handle_event(&mut self, event: &web_sys::Event) {
        // This is where you'd handle mouse, keyboard, touch events
        // Applications can override this method for custom event handling
        match event.type_().as_str() {
            "resize" => {
                if let Some(target) = event.target() {
                    if let Ok(canvas) = target.dyn_into::<HtmlCanvasElement>() {
                        self.handle_resize(canvas.client_width() as u32, canvas.client_height() as u32);
                    }
                }
            }
            _ => {
                // Let the user app handle other events
                // Note: This would need proper implementation for web event handling
                // For now, we'll just ignore other events
            }
        }
    }
}

// Add web event handling to PlinthApp trait
#[cfg(feature = "web-canvas")]
pub trait WebPlinthApp: crate::plinth_app::PlinthApp {
    fn handle_web_event(&mut self, _event: &web_sys::Event) {
        // Default implementation - applications can override
    }
}
