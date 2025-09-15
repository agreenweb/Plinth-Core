use plinth_primitives::Color;

#[derive(Debug, Clone)]
pub struct CssClass {
    pub name: String,
    pub color: Option<Color>,
    pub background_color: Option<Color>,
    pub border_color: Option<Color>,
}

impl CssClass {
    pub fn new(name: String) -> Self {
        Self {
            name,
            color: None,
            background_color: None,
            border_color: None,
        }
    }

    pub fn with_color(mut self, color: Color) -> Self {
        self.color = Some(color);
        self
    }

    pub fn with_background_color(mut self, color: Color) -> Self {
        self.background_color = Some(color);
        self
    }

    pub fn with_border_color(mut self, color: Color) -> Self {
        self.border_color = Some(color);
        self
    }
}
