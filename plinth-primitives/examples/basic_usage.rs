use plinth_primitives::{Circle, Color, PrimitiveRenderer, Transform};
use plinth_styles::{ClassMapper, CssClass, ColorProperty};
use glam::Vec2;

fn main() {
    println!("Plinth Primitives Basic Usage Example");
    
    // Create some circles with different properties
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

    // Create a CSS class mapper
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

    // Apply CSS class overrides
    let mut styled_circles = circles;
    for circle in &mut styled_circles {
        if let Some(ref class_name) = circle.css_class {
            if let Some(override_color) = class_mapper.get_color_for_class(class_name, ColorProperty::Color) {
                circle.apply_css_override(override_color);
                println!("Applied CSS class '{}' with color {:?}", class_name, override_color);
            }
        }
    }

    println!("Created {} circles with CSS styling", styled_circles.len());
}
