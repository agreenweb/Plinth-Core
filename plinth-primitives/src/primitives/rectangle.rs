use glam::Vec2;
use crate::types::{Color, Transform};

#[derive(Debug, Clone)]
pub struct Rectangle {
    pub position: Vec2,
    pub size: Vec2,
    pub color: Color,
    pub transform: Transform,
    pub css_class: Option<String>,
}

impl Rectangle {
    pub fn new(position: Vec2, size: Vec2) -> Self {
        Self {
            position,
            size,
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

impl Default for Rectangle {
    fn default() -> Self {
        Self::new(Vec2::ZERO, Vec2::ONE)
    }
}
