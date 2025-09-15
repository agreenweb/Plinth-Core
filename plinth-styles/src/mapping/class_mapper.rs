use std::collections::HashMap;
use plinth_primitives::Color;
use crate::types::CssClass;

pub struct ClassMapper {
    classes: HashMap<String, CssClass>,
}

impl ClassMapper {
    pub fn new() -> Self {
        Self {
            classes: HashMap::new(),
        }
    }

    pub fn add_class(&mut self, class: CssClass) {
        self.classes.insert(class.name.clone(), class);
    }

    pub fn get_class(&self, class_name: &str) -> Option<&CssClass> {
        self.classes.get(class_name)
    }

    pub fn get_color_for_class(&self, class_name: &str, property: ColorProperty) -> Option<Color> {
        self.classes.get(class_name).and_then(|class| match property {
            ColorProperty::Color => class.color,
            ColorProperty::BackgroundColor => class.background_color,
            ColorProperty::BorderColor => class.border_color,
        })
    }

    #[cfg(feature = "web")]
    pub fn load_from_dom(&mut self) -> Result<(), StyleError> {
        // This will parse CSS from the DOM and extract class definitions
        // For now, we'll implement a simple version
        Ok(())
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ColorProperty {
    Color,
    BackgroundColor,
    BorderColor,
}

#[derive(Debug, thiserror::Error)]
pub enum StyleError {
    #[error("Failed to parse CSS: {0}")]
    CssParseError(String),
    #[error("Color not found for class: {0}")]
    ColorNotFound(String),
    #[error("DOM access failed")]
    DomAccessFailed,
}

impl Default for ClassMapper {
    fn default() -> Self {
        Self::new()
    }
}
