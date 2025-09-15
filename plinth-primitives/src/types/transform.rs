use glam::Vec2;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
}

impl Transform {
    pub fn new(position: Vec2, scale: Vec2, rotation: f32) -> Self {
        Self {
            position,
            scale,
            rotation,
        }
    }

    pub fn identity() -> Self {
        Self {
            position: Vec2::ZERO,
            scale: Vec2::ONE,
            rotation: 0.0,
        }
    }

    pub fn with_position(mut self, position: Vec2) -> Self {
        self.position = position;
        self
    }

    pub fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }

    pub fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self::identity()
    }
}
