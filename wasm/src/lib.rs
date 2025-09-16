use plinth_core::{plinth_app::PlinthApp, plinth_app::PlinthRenderer, web_canvas::{WebCanvas, WebEventLoop, WebRc}};
use plinth_primitives::{Circle, Color, Transform, PrimitiveRenderer};
use plinth_styles::{ClassMapper, CssClass, CssWatcher};
use plinth_styles::mapping::ColorProperty;

use std::cell::RefCell;
use wasm_bindgen::prelude::*;

// Helper function to parse hex color strings
fn parse_hex_color(hex: &str) -> Result<Color, String> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return Err(format!("Invalid hex color length: {}", hex));
    }
    
    let r = u8::from_str_radix(&hex[0..2], 16)
        .map_err(|_| "Invalid hex color")?;
    let g = u8::from_str_radix(&hex[2..4], 16)
        .map_err(|_| "Invalid hex color")?;
    let b = u8::from_str_radix(&hex[4..6], 16)
        .map_err(|_| "Invalid hex color")?;
    
    Ok(Color::from_rgba(r, g, b, 255))
}
use glam::Vec2;

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

struct PrimitivesTestApp {
    frame_count: u32,
    circles: Vec<Circle>,
    class_mapper: WebRc<RefCell<ClassMapper>>,
    time: f32,
    primitive_renderer: Option<PrimitiveRenderer>,
    css_watcher: Option<CssWatcher>,
}

impl PrimitivesTestApp {
    fn new() -> Self {
        let class_mapper = WebRc::new(RefCell::new(ClassMapper::new()));
        
        // Add default CSS classes
        {
            let mut mapper = class_mapper.borrow_mut();
            mapper.add_class(
                CssClass::new("primary-button".to_string())
                    .with_color(Color::from_hex(0xFF6B6B)) // Red
            );
            
            mapper.add_class(
                CssClass::new("secondary-button".to_string())
                    .with_color(Color::from_hex(0x4ECDC4)) // Teal
            );
            
            mapper.add_class(
                CssClass::new("accent-button".to_string())
                    .with_color(Color::from_hex(0x45B7D1)) // Blue
            );
        }

        let circles = vec![
            Circle::new(Vec2::new(0.0, 0.0), 0.15)
                .with_color(Color::RED)
                .with_css_class("primary-button"),
            Circle::new(Vec2::new(0.4, 0.0), 0.12)
                .with_color(Color::BLUE)
                .with_css_class("secondary-button"),
            Circle::new(Vec2::new(-0.4, 0.0), 0.13)
                .with_color(Color::GREEN)
                .with_css_class("accent-button")
                .with_transform(Transform::new(Vec2::new(0.0, 0.0), Vec2::new(1.0, 1.0), 0.0)),
        ];

        Self {
            frame_count: 0,
            circles,
            class_mapper,
            time: 0.0,
            primitive_renderer: None,
            css_watcher: None,
        }
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

impl PlinthApp for PrimitivesTestApp {
    fn init(&mut self) {
        console_log!("Primitives test app initialized!");
        
        // Initialize CSS watcher
        let mut css_watcher = CssWatcher::new(WebRc::clone(&self.class_mapper));
        css_watcher.watch_class("primary-button");
        css_watcher.watch_class("secondary-button");
        css_watcher.watch_class("accent-button");
        
        // Set up callback to log when CSS changes
        css_watcher.set_callback(|| {
            console_log!("CSS custom properties changed, class mapper updated");
        });
        
        if let Err(e) = css_watcher.start() {
            console_log!("Failed to start CSS watcher: {:?}", e);
        } else {
            console_log!("CSS watcher started successfully");
        }
        
        self.css_watcher = Some(css_watcher);
    }

    fn before_render(&mut self) {
        self.frame_count += 1;
        
        // Apply CSS class overrides
        for circle in &mut self.circles {
            if let Some(ref class_name) = circle.css_class {
                if let Some(override_color) = self.class_mapper.borrow().get_color_for_class(class_name, ColorProperty::Color) {
                    console_log!("Applying CSS override for {}: color={:?}", class_name, override_color);
                    circle.apply_css_override(override_color);
                } else {
                    console_log!("No CSS override found for class: {}", class_name);
                }
            }
        }
        
        // Animate the circles
        for (i, circle) in self.circles.iter_mut().enumerate() {
            let offset = (self.time + i as f32 * 0.5) * 0.5;
            circle.center.x = 0.0 + 0.4 * offset.sin();
            circle.center.y = 0.0 + 0.3 * offset.cos();
        }
        
        // Update time for animation
        self.time += 0.016; // ~60 FPS
        
        if self.frame_count % 60 == 0 {
            console_log!("Primitives frame: {} - time: {:.2} - rendering {} circles", 
                self.frame_count, self.time, self.circles.len());
            for (i, circle) in self.circles.iter().enumerate() {
                console_log!("Circle {}: pos=({:.2}, {:.2}) radius={:.2}", 
                    i, circle.center.x, circle.center.y, circle.radius);
            }
        }
    }

    fn after_render(&mut self) {
        // Optional: Add any post-render logic here
    }

    fn on_close(&mut self) {
        console_log!("Primitives test app closing after {} frames", self.frame_count);
    }
}

impl PlinthRenderer for TestApp {
    // Use the default triangle rendering
    // The default implementation from the trait will be used
}

impl PlinthRenderer for PrimitivesTestApp {
    // Override the render method to draw primitives
    fn render(&mut self, graphics: &mut plinth_core::graphics::Graphics) {
        // Initialize primitive renderer if not already done
        if self.primitive_renderer.is_none() {
            self.primitive_renderer = Some(PrimitiveRenderer::new(&graphics.device, graphics.surface_config.format));
            console_log!("Primitive renderer initialized!");
        }
        
        // Log primitives data
        if self.frame_count % 60 == 0 {
            console_log!("Rendering {} primitives with CSS styling", self.circles.len());
            for (i, circle) in self.circles.iter().enumerate() {
                console_log!("Circle {}: center=({:.1}, {:.1}), radius={:.1}, color=({:.1}, {:.1}, {:.1}, {:.1})", 
                    i, circle.center.x, circle.center.y, circle.radius, 
                    circle.color.r, circle.color.g, circle.color.b, circle.color.a);
            }
        }
        
        // Add circles to the primitive renderer
        if let Some(ref mut primitive_renderer) = self.primitive_renderer {
            primitive_renderer.clear_circles();
            
            // Log the actual colors being sent to the renderer
            for (i, circle) in self.circles.iter().enumerate() {
                console_log!("Sending circle {} to renderer: color=({:.3}, {:.3}, {:.3}, {:.3})", 
                    i, circle.color.r, circle.color.g, circle.color.b, circle.color.a);
            }
            
            primitive_renderer.add_circles(self.circles.clone());
            
            console_log!("About to render {} circles", self.circles.len());
            
            // Render the circles using the primitive renderer
            primitive_renderer.render(&graphics.device, &graphics.queue, &graphics.surface, &graphics.surface_config);
            
            console_log!("Circle rendering completed");
        } else {
            console_log!("Primitive renderer not initialized!");
        }
    }
}


#[wasm_bindgen]
pub struct WasmApp {
    event_loop: Option<WebEventLoop>,
    primitives_app: Option<WebRc<RefCell<PrimitivesTestApp>>>,
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
            primitives_app: None,
        }
    }

    #[wasm_bindgen]
    pub async fn new_with_canvas(canvas_id: &str) -> Result<WasmApp, JsValue> {
        console_error_panic_hook::set_once();
        let _ = console_log::init();
        
        let mut app = WasmApp {
            event_loop: None,
            primitives_app: None,
        };
        
        app.start(canvas_id).await?;
        Ok(app)
    }

    #[wasm_bindgen]
    pub async fn new_primitives_with_canvas(canvas_id: &str) -> Result<WasmApp, JsValue> {
        console_error_panic_hook::set_once();
        let _ = console_log::init();
        
        let mut app = WasmApp {
            event_loop: None,
            primitives_app: None,
        };
        
        app.start_primitives(canvas_id).await?;
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
    pub async fn start_primitives(&mut self, canvas_id: &str) -> Result<(), JsValue> {
        console_log!("Starting WASM primitives app with canvas: {}", canvas_id);
        
        // Create the canvas
        let canvas = WebCanvas::new(canvas_id)?;
        let canvas_rc: WebRc<WebCanvas> = WebRc::new(canvas);
        
        // Create the primitives test app
        let primitives_app = PrimitivesTestApp::new();
        let primitives_app_rc: WebRc<RefCell<PrimitivesTestApp>> = WebRc::new(RefCell::new(primitives_app));
        let app_rc: WebRc<RefCell<dyn PlinthApp>> = WebRc::clone(&primitives_app_rc) as WebRc<RefCell<dyn PlinthApp>>;
        
        // Create graphics
        let graphics = plinth_core::graphics::create_graphics_web(canvas_rc.clone(), app_rc.clone()).await;
        let graphics_rc: WebRc<RefCell<plinth_core::graphics::Graphics>> = WebRc::new(RefCell::new(graphics));
        
        // Create and start the event loop
        let mut event_loop = WebEventLoop::new(canvas_rc, graphics_rc, app_rc);
        event_loop.start();
        
        self.event_loop = Some(event_loop);
        self.primitives_app = Some(primitives_app_rc);
        
        console_log!("WASM primitives app started successfully!");
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
    
    #[wasm_bindgen]
    pub fn request_redraw(&mut self) {
        if let Some(ref mut event_loop) = self.event_loop {
            // This will trigger a single frame render
            // We'll need to call this repeatedly from JavaScript for animation
            event_loop.request_redraw();
        }
    }
    
    #[wasm_bindgen]
    pub fn update_css_colors(&mut self, primary_color: &str, secondary_color: &str, accent_color: &str) {
        // This method will be called from JavaScript when colors change
        console_log!("Updating CSS colors: primary={}, secondary={}, accent={}", 
            primary_color, secondary_color, accent_color);
        
        if let Some(ref primitives_app) = self.primitives_app {
            let mut app = primitives_app.borrow_mut();
            
            // Parse hex colors and update the class mapper
            if let Ok(primary) = parse_hex_color(primary_color) {
                console_log!("Parsed primary color: {:?}", primary);
                app.class_mapper.borrow_mut().add_class(
                    CssClass::new("primary-button".to_string())
                        .with_color(primary)
                );
            } else {
                console_log!("Failed to parse primary color: {}", primary_color);
            }
            
            if let Ok(secondary) = parse_hex_color(secondary_color) {
                console_log!("Parsed secondary color: {:?}", secondary);
                app.class_mapper.borrow_mut().add_class(
                    CssClass::new("secondary-button".to_string())
                        .with_color(secondary)
                );
            } else {
                console_log!("Failed to parse secondary color: {}", secondary_color);
            }
            
            if let Ok(accent) = parse_hex_color(accent_color) {
                console_log!("Parsed accent color: {:?}", accent);
                app.class_mapper.borrow_mut().add_class(
                    CssClass::new("accent-button".to_string())
                        .with_color(accent)
                );
            } else {
                console_log!("Failed to parse accent color: {}", accent_color);
            }
        } else {
            console_log!("No primitives app found!");
        }
    }
}

