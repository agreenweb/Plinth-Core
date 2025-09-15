# Plinth Primitives

A WebGPU primitives library for the Plinth framework with CSS class support.

## Features

- **Simple Primitives**: Circle, Rectangle, Triangle with easy-to-use APIs
- **Instanced Rendering**: Efficient rendering of many primitives using WebGPU instancing
- **CSS Class Support**: Override primitive properties using CSS classes
- **Type Safety**: Rust's type system ensures correct parameter passing
- **Performance**: Optimized for rendering large numbers of primitives

## Basic Usage

```rust
use plinth_primitives::{Circle, Color, PrimitiveRenderer};
use plinth_styles::{ClassMapper, CssClass, ColorProperty};
use glam::Vec2;

// Create a circle
let circle = Circle::new(Vec2::new(100.0, 100.0), 50.0)
    .with_color(Color::RED)
    .with_css_class("primary-button");

// Create a CSS class mapper
let mut class_mapper = ClassMapper::new();
class_mapper.add_class(
    CssClass::new("primary-button".to_string())
        .with_color(Color::from_hex(0xFF6B35))
);

// Apply CSS overrides during render
if let Some(override_color) = class_mapper.get_color_for_class("primary-button", ColorProperty::Color) {
    circle.apply_css_override(override_color);
}
```

## Primitives

### Circle
- Position and radius
- Color and CSS class support
- Transform support (position, scale, rotation)

### Rectangle
- Position and size
- Color and CSS class support
- Transform support

### Triangle
- Three vertices
- Color and CSS class support
- Transform support

## CSS Integration

The library supports CSS class-based styling through the `plinth-styles` crate:

- **Color Properties**: `color`, `background-color`, `border-color`
- **Class Mapping**: Map CSS classes to WebGPU parameters
- **Runtime Overrides**: Apply CSS styles during the render loop

## Performance

- Uses WebGPU instanced rendering for efficient batch drawing
- Single draw call for multiple instances of the same primitive type
- Optimized shaders with anti-aliasing support

## Dependencies

- `wgpu`: WebGPU rendering
- `glam`: Vector math
- `bytemuck`: Zero-copy serialization
- `thiserror`: Error handling
