# Plinth Styles

CSS to WebGPU style mapping for Plinth primitives.

## Features

- **CSS Class Mapping**: Map CSS classes to WebGPU rendering parameters
- **Color Parsing**: Parse CSS color values and convert to WebGPU-compatible formats
- **Runtime Style Resolution**: Apply CSS styles during the render loop
- **Type Safety**: Rust's type system ensures correct style mapping

## Basic Usage

```rust
use plinth_styles::{ClassMapper, CssClass, ColorProperty};
use plinth_primitives::Color;

// Create a class mapper
let mut class_mapper = ClassMapper::new();

// Add CSS classes
class_mapper.add_class(
    CssClass::new("primary-button".to_string())
        .with_color(Color::from_hex(0xFF6B35))
        .with_background_color(Color::from_hex(0xFFFFFF))
);

// Get color for a class
if let Some(color) = class_mapper.get_color_for_class("primary-button", ColorProperty::Color) {
    // Apply the color to your primitive
    println!("Primary button color: {:?}", color);
}
```

## CSS Properties

### Color Properties
- `color`: Text/foreground color
- `background-color`: Background color
- `border-color`: Border color

## Color Formats

Currently supports:
- Hex colors: `#FF6B35`, `#4ECDC4`
- RGB colors: `rgb(255, 107, 53)`
- RGBA colors: `rgba(255, 107, 53, 0.8)`

## Web Integration

When the `web` feature is enabled, the crate can:
- Parse CSS from the DOM
- Extract class definitions from stylesheets
- Apply styles dynamically based on CSS changes

## Dependencies

- `plinth-primitives`: Core primitive types
- `thiserror`: Error handling
