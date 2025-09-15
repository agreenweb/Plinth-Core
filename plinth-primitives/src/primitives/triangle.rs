use glam::Vec2;
use crate::types::{Color, Transform};

#[derive(Debug, Clone)]
pub struct Triangle {
    pub vertices: [Vec2; 3],
    pub color: Color,
    pub transform: Transform,
    pub css_class: Option<String>,
}

impl Triangle {
    pub fn new(vertices: [Vec2; 3]) -> Self {
        Self {
            vertices,
            color: Color::default(),
            transform: Transform::default(),
            css_class: None,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    pub fn with_css_class(mut self, class: impl Into<String>) -> Self {
        self.css_class = Some(class.into());
        self
    }

    pub fn with_transform(mut self, transform: Transform) -> Self {
        self.transform = transform;
        self
    }

    pub fn apply_css_override(&mut self, color: Color) {
        self.color = color;
    }
}

impl Default for Triangle {
    fn default() -> Self {
        Self::new([
            Vec2::new(0.0, 0.5),
            Vec2::new(-0.5, -0.5),
            Vec2::new(0.5, -0.5),
        ])
    }
}
