use plinth_core::{PlinthApp, PlinthRenderer, start_app, Graphics};
use plinth_primitives::{Circle, Color, PrimitiveRenderer, Transform};
use plinth_styles::{ClassMapper, CssClass, ColorProperty};
use glam::Vec2;
use std::cell::RefCell;
use std::rc::Rc;

struct PrimitivesExample {
    primitive_renderer: PrimitiveRenderer,
    class_mapper: ClassMapper,
    circles: Vec<Circle>,
}

impl PrimitivesExample {
    fn new(gfx: &mut Graphics) -> Self {
        let primitive_renderer = PrimitiveRenderer::new(&gfx.device, gfx.surface_config.format);
        
        let mut class_mapper = ClassMapper::new();
        
        // Add some CSS classes
        class_mapper.add_class(
            CssClass::new("primary-button".to_string())
                .with_color(Color::from_hex(0xFF6B35)) // Orange
        );
        
        class_mapper.add_class(
            CssClass::new("secondary-button".to_string())
                .with_color(Color::from_hex(0x4ECDC4)) // Teal
        );

        // Create some circles
        let circles = vec![
            Circle::new(Vec2::new(100.0, 100.0), 50.0)
                .with_color(Color::RED)
                .with_css_class("primary-button"),
            Circle::new(Vec2::new(200.0, 100.0), 30.0)
                .with_color(Color::BLUE)
                .with_css_class("secondary-button"),
            Circle::new(Vec2::new(300.0, 100.0), 40.0)
                .with_color(Color::GREEN)
                .with_transform(Transform::new(Vec2::new(0.0, 0.0), Vec2::new(1.5, 1.5), 0.0)),
        ];

        Self {
            primitive_renderer,
            class_mapper,
            circles,
        }
    }
}

impl PlinthApp for PrimitivesExample {
    fn init(&mut self) {
        println!("Primitives example initialized!");
    }

    fn before_render(&mut self) {
        // Apply CSS class overrides
        for circle in &mut self.circles {
            if let Some(ref class_name) = circle.css_class {
                if let Some(override_color) = self.class_mapper.get_color_for_class(class_name, ColorProperty::Color) {
                    circle.apply_css_override(override_color);
                }
            }
        }
    }

    fn after_render(&mut self) {
        // Could add any post-render logic here
    }
}

impl PlinthRenderer for PrimitivesExample {
    fn render(&mut self, gfx: &mut Graphics) {
        // Clear the primitive renderer
        self.primitive_renderer.clear_circles();
        
        // Add circles to the renderer
        self.primitive_renderer.add_circles(self.circles.clone());
        
        // Render using the primitive renderer
        self.primitive_renderer.render(&gfx.device, &gfx.queue, &gfx.surface, &gfx.surface_config);
    }

    fn create_pipeline(&mut self, gfx: &mut Graphics) -> wgpu::RenderPipeline {
        // We don't need to create a pipeline here since the primitive renderer handles it
        // Just return a dummy pipeline or the first one from the graphics context
        gfx.render_pipelines[0].clone()
    }
}

fn main() {
    let app = Rc::new(RefCell::new(PrimitivesExample {
        primitive_renderer: PrimitiveRenderer::new(&wgpu::Device::default(), wgpu::TextureFormat::Bgra8UnormSrgb),
        class_mapper: ClassMapper::new(),
        circles: vec![],
    }));
    
    // Note: This is a simplified example. In practice, you'd need to properly initialize
    // the graphics context before creating the primitive renderer
    println!("This example shows how to integrate plinth-primitives with plinth-core");
    println!("To run a full example, you'd need to properly initialize the graphics context");
}
