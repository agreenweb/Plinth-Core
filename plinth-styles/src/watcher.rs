#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "web")]
use web_sys::{MutationObserver, MutationRecord, Element, HtmlElement, Document};
#[cfg(feature = "web")]
use std::rc::Rc;
#[cfg(feature = "web")]
use std::cell::RefCell;
#[cfg(feature = "web")]
use plinth_primitives::Color;
#[cfg(feature = "web")]
use crate::mapping::{ClassMapper, StyleError};
#[cfg(feature = "web")]
use js_sys;
#[cfg(feature = "web")]
use console_log;

#[cfg(feature = "web")]
pub struct CssWatcher {
    observer: Option<MutationObserver>,
    class_mapper: Rc<RefCell<ClassMapper>>,
    watched_classes: Vec<String>,
    callback: Option<Box<dyn Fn()>>,
}

#[cfg(feature = "web")]
impl CssWatcher {
    pub fn new(class_mapper: Rc<RefCell<ClassMapper>>) -> Self {
        Self {
            observer: None,
            class_mapper,
            watched_classes: Vec::new(),
            callback: None,
        }
    }

    pub fn watch_class(&mut self, class_name: &str) {
        self.watched_classes.push(class_name.to_string());
    }

    pub fn set_callback<F>(&mut self, callback: F) 
    where 
        F: Fn() + 'static 
    {
        self.callback = Some(Box::new(callback));
    }

    pub fn start(&mut self) -> Result<(), StyleError> {
        let window = web_sys::window().ok_or(StyleError::DomAccessFailed)?;
        let document = window.document().ok_or(StyleError::DomAccessFailed)?;
        
        // Find or create a style element to watch
        let style_element = self.get_or_create_style_element(&document)?;
        
        let class_mapper = Rc::clone(&self.class_mapper);
        let watched_classes = self.watched_classes.clone();
        let callback = self.callback.take();
        
        let closure = Closure::wrap(Box::new(move |mutations: &js_sys::Array| {
            for i in 0..mutations.length() {
                if let Some(mutation) = mutations.get(i).dyn_ref::<MutationRecord>() {
                    // Watch for any changes to the style element (including child changes)
                    if mutation.type_() == "childList" || mutation.type_() == "attributes" {
                        // CSS custom property changed, update class mapper
                        // console_log::log(&format!("CSS mutation detected! Updating class mapper..."));
                        Self::update_class_mapper_from_css(&class_mapper, &watched_classes);
                        
                        // Call the callback if set
                        if let Some(ref callback) = callback {
                            callback();
                        }
                    }
                }
            }
        }) as Box<dyn FnMut(&js_sys::Array)>);

        let observer = MutationObserver::new(closure.as_ref().unchecked_ref())
            .map_err(|_| StyleError::DomAccessFailed)?;
        
        let mut init = web_sys::MutationObserverInit::new();
        init.child_list(true);
        init.attributes(true);
        observer.observe_with_options(&style_element, &init);
        
        self.observer = Some(observer);
        closure.forget();
        
        Ok(())
    }

    pub fn stop(&mut self) {
        if let Some(observer) = self.observer.take() {
            observer.disconnect();
        }
    }

    fn get_or_create_style_element(&self, document: &Document) -> Result<Element, StyleError> {
        // Look for existing style element with id "plinth-styles"
        if let Some(element) = document.get_element_by_id("plinth-styles") {
            return Ok(element);
        }
        
        // Create new style element
        let style_element = document.create_element("style")
            .map_err(|_| StyleError::DomAccessFailed)?;
        style_element.set_id("plinth-styles");
        
        // Add it to the document head
        if let Some(head) = document.head() {
            head.append_child(&style_element)
                .map_err(|_| StyleError::DomAccessFailed)?;
        }
        
        Ok(style_element)
    }

    fn update_class_mapper_from_css(class_mapper: &Rc<RefCell<ClassMapper>>, watched_classes: &[String]) {
        // For now, let's use a simpler approach: create test elements with the classes
        // and read their computed styles
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
        for class_name in watched_classes {
            // Create a temporary element with the class to read computed styles
            if let Ok(temp_element) = document.create_element("div") {
                temp_element.set_class_name(class_name);
                if let Some(html_element) = temp_element.dyn_ref::<web_sys::HtmlElement>() {
                    html_element.style().set_property("--color", "transparent").ok();
                }
                
                // Add to body temporarily to get computed styles
                if let Some(body) = document.body() {
                    body.append_child(&temp_element).ok();
                    
                    // Get computed style
                    if let Ok(computed_style) = window.get_computed_style(&temp_element) {
                        if let Some(computed) = computed_style {
                            if let Ok(color_value) = computed.get_property_value("--color") {
                                if !color_value.is_empty() && color_value != "transparent" {
                                    if let Ok(color) = Self::parse_css_color(&color_value) {
                                        let mut mapper = class_mapper.borrow_mut();
                                        mapper.add_class(
                                            crate::types::CssClass::new(class_name.clone())
                                                .with_color(color)
                                        );
                                        // console_log::log(&format!("Updated CSS class {} with color: {:?}", class_name, color));
                                    }
                                }
                            }
                        }
                    }
                    
                    // Remove temporary element
                    body.remove_child(&temp_element).ok();
                }
            }
        }
    }

    fn parse_css_color(color_str: &str) -> Result<Color, StyleError> {
        let color_str = color_str.trim();
        
        if color_str.starts_with('#') {
            let hex = &color_str[1..];
            if hex.len() == 6 {
                let r = u8::from_str_radix(&hex[0..2], 16)
                    .map_err(|_| StyleError::CssParseError("Invalid hex color".to_string()))?;
                let g = u8::from_str_radix(&hex[2..4], 16)
                    .map_err(|_| StyleError::CssParseError("Invalid hex color".to_string()))?;
                let b = u8::from_str_radix(&hex[4..6], 16)
                    .map_err(|_| StyleError::CssParseError("Invalid hex color".to_string()))?;
                return Ok(Color::from_rgba(r, g, b, 255));
            }
        }
        
        Err(StyleError::CssParseError(format!("Unsupported color format: {}", color_str)))
    }
}

#[cfg(feature = "web")]
impl Drop for CssWatcher {
    fn drop(&mut self) {
        self.stop();
    }
}

#[cfg(not(feature = "web"))]
pub struct CssWatcher;

#[cfg(not(feature = "web"))]
impl CssWatcher {
    pub fn new(_class_mapper: std::rc::Rc<std::cell::RefCell<ClassMapper>>) -> Self {
        Self
    }

    pub fn watch_class(&mut self, _class_name: &str) {
        // No-op for non-web targets
    }

    pub fn set_callback<F>(&mut self, _callback: F) 
    where 
        F: Fn() + 'static 
    {
        // No-op for non-web targets
    }

    pub fn start(&mut self) -> Result<(), crate::mapping::StyleError> {
        Ok(())
    }

    pub fn stop(&mut self) {
        // No-op for non-web targets
    }
}
