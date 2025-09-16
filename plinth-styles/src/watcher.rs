#[cfg(feature = "web")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "web")]
use web_sys::{HtmlElement, Element, MutationObserver, MutationObserverInit, MutationRecord, Document};
#[cfg(feature = "web")]
use std::rc::Rc;
#[cfg(feature = "web")]
use std::cell::RefCell;
#[cfg(feature = "web")]
use plinth_primitives::Color;
#[cfg(feature = "web")]
use crate::mapping::{ClassMapper, StyleError};
#[cfg(feature = "web")]
use js_sys::Array;
#[cfg(feature = "web")]
use std::collections::HashMap;

#[cfg(feature = "web")]
pub struct CssWatcher {
    observer: Option<MutationObserver>,
    class_mapper: Rc<RefCell<ClassMapper>>,
    watched_classes: Vec<String>,
    callback: Option<Rc<dyn Fn()>>,
    cached_values: Rc<RefCell<HashMap<String, String>>>,
}

#[cfg(feature = "web")]
impl CssWatcher {
    pub fn new(class_mapper: Rc<RefCell<ClassMapper>>) -> Self {
        Self {
            observer: None,
            class_mapper,
            watched_classes: Vec::new(),
            callback: None,
            cached_values: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn watch_class(&mut self, class_name: &str) {
        self.watched_classes.push(class_name.to_string());
    }

    pub fn set_callback<F>(&mut self, callback: F) 
    where 
        F: Fn() + 'static 
    {
        self.callback = Some(Rc::new(callback));
    }
    // ... new(), watch_class(), set_callback() unchanged ...

    pub fn start(&mut self) -> Result<(), StyleError> {
        web_sys::console::log_1(&"CSS Watcher: Starting watcher...".into());
        let window = web_sys::window().ok_or(StyleError::DomAccessFailed)?;
        let document = window.document().ok_or(StyleError::DomAccessFailed)?;

        // Initial update (now sampling computed "color")
        self.update_class_mapper_from_elements(&document)?;

        // Head observer for <style>/<link> (stylesheet mutations)
        self.setup_head_observer(document.clone())?;

        // Body observer for class/style attribute flips & childList changes
        self.setup_mutation_observer(document)?;

        // Media query flips (dark mode, etc.)
        self.setup_media_query_listeners()?;

        web_sys::console::log_1(&"CSS Watcher: Event-driven watcher started".into());
        Ok(())
    }

    fn setup_head_observer(&mut self, document: Document) -> Result<(), StyleError> {
        let head = document.head().ok_or(StyleError::DomAccessFailed)?;
        let class_mapper = Rc::clone(&self.class_mapper);
        let watched_classes = self.watched_classes.clone();
        let cached_values = Rc::clone(&self.cached_values);
        let callback = self.callback.clone();

        let closure = Closure::wrap(Box::new(move |_muts: js_sys::Array, _obs: MutationObserver| {
            // Any stylesheet change can affect colors—resample all watched elements
            let changed = Self::check_and_update_styles(&class_mapper, &watched_classes, &cached_values);
            if changed {
                if let Some(ref cb) = callback { cb(); }
            }
        }) as Box<dyn FnMut(js_sys::Array, MutationObserver)>);

        let observer = MutationObserver::new(closure.as_ref().unchecked_ref())
            .map_err(|_| StyleError::DomAccessFailed)?;

        let opts = MutationObserverInit::new();
        opts.set_child_list(true);
        opts.set_subtree(true);
        observer.observe_with_options(&head, &opts).map_err(|_| StyleError::DomAccessFailed)?;

        // Keep alive
        closure.forget();
        // We don't store this observer; it's okay to let DOM keep it alive, or add another field if you prefer.
        Ok(())
    }

    fn setup_media_query_listeners(&mut self) -> Result<(), StyleError> {
        let window = web_sys::window().ok_or(StyleError::DomAccessFailed)?;
        let class_mapper = Rc::clone(&self.class_mapper);
        let watched_classes = self.watched_classes.clone();
        let cached_values = Rc::clone(&self.cached_values);
        let callback = self.callback.clone();

        // Only handle one media query for now to avoid move issues
        if let Some(mql) = window.match_media("(prefers-color-scheme: dark)").map_err(|_| StyleError::DomAccessFailed)? {
            let mql: web_sys::MediaQueryList = mql.dyn_into().map_err(|_| StyleError::DomAccessFailed)?;
            let cb = Closure::wrap(Box::new(move |_evt: web_sys::Event| {
                let changed = Self::check_and_update_styles(&class_mapper, &watched_classes, &cached_values);
                if changed {
                    if let Some(ref cb) = callback { cb(); }
                }
            }) as Box<dyn FnMut(web_sys::Event)>);
            mql.add_event_listener_with_callback("change", cb.as_ref().unchecked_ref())
                .map_err(|_| StyleError::DomAccessFailed)?;
            cb.forget();
        }
        Ok(())
    }

    fn setup_mutation_observer(&mut self, document: Document) -> Result<(), StyleError> {
        let class_mapper = Rc::clone(&self.class_mapper);
        let watched_classes = self.watched_classes.clone();
        let callback = self.callback.clone();
        let cached_values = Rc::clone(&self.cached_values);

        let closure = Closure::wrap(Box::new(move |mutations: Array, _observer: MutationObserver| {
            let mut relevant = false;

            web_sys::console::log_1(&"CSS Watcher: Mutation detected".into());

            for i in 0..mutations.length() {
                if let Some(mutation) = mutations.get(i).dyn_into::<MutationRecord>().ok() {
                    let t = mutation.type_();
                    web_sys::console::log_2(&"Mutation type:".into(), &t.clone().into());

                    // Any class/style attribute change or childList can affect colors.
                    if t == "attributes" || t == "childList" {
                        relevant = true;
                        web_sys::console::log_1(&"Relevant mutation detected".into());
                        break;
                    }
                }
            }

            if relevant {
                web_sys::console::log_1(&"Checking for style changes...".into());
                let any_changes = Self::check_and_update_styles(&class_mapper, &watched_classes, &cached_values);
                if any_changes {
                    web_sys::console::log_1(&"Style changes detected, calling callback".into());
                    if let Some(ref callback) = callback { callback(); }
                } else {
                    web_sys::console::log_1(&"No style changes found".into());
                }
            }
        }) as Box<dyn FnMut(Array, MutationObserver)>);

        let observer = MutationObserver::new(closure.as_ref().unchecked_ref())
            .map_err(|_| StyleError::DomAccessFailed)?;

        let options = MutationObserverInit::new();
        options.set_child_list(true);
        options.set_attributes(true);
        options.set_subtree(true);

        // Filter to common style-affecting attrs
        let attribute_filter = Array::new();
        attribute_filter.push(&"style".into());
        attribute_filter.push(&"class".into());
        options.set_attribute_filter(&attribute_filter);

        observer.observe_with_options(&document, &options)
            .map_err(|_| StyleError::DomAccessFailed)?;

        self.observer = Some(observer);
        closure.forget();
        Ok(())
    }

    fn check_and_update_styles(
        class_mapper: &Rc<RefCell<ClassMapper>>,
        watched_classes: &[String],
        cached_values: &Rc<RefCell<HashMap<String, String>>>
    ) -> bool {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let mut any_changes = false;

        for class_name in watched_classes {
            if let Ok(elements) = document.query_selector_all(&format!(".{}", class_name)) {
                for k in 0..elements.length() {
                    if let Some(node) = elements.get(k) {
                        if let Some(el) = node.dyn_ref::<Element>() {
                            if let Ok(cs_opt) = window.get_computed_style(el) {
                                if let Some(cs) = cs_opt {
                                    // *** Sample the CSS custom property --color ***
                                    if let Ok(color_value) = cs.get_property_value("--color") {
                                        if !color_value.is_empty() {
                                            // *** Cache per element to avoid index drift ***
                                            let el_id = Self::ensure_element_cache_id(el);
                                            let cache_key = format!("{}#{}", class_name, el_id);

                                            let mut cache = cached_values.borrow_mut();
                                            let changed = match cache.get(&cache_key) {
                                                Some(old) => old != &color_value,
                                                None => true,
                                            };

                                            if changed {
                                                cache.insert(cache_key, color_value.clone());

                                                web_sys::console::log_3(
                                                    &"CSS Watcher: --color changed for class".into(),
                                                    &class_name.into(),
                                                    &color_value.clone().into()
                                                );

                                                if let Ok(color) = Self::parse_css_color(&color_value) {
                                                    let mut mapper = class_mapper.borrow_mut();
                                                    mapper.add_class(
                                                        crate::types::CssClass::new(class_name.clone())
                                                            .with_color(color)
                                                    );
                                                    any_changes = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        any_changes
    }

    fn ensure_element_cache_id(el: &Element) -> String {
        // Use a stable per-element id in data attribute to avoid NodeList index drift
        let key = "data-colorwatch-id";
        if let Some(id) = el.get_attribute(key) {
            return id;
        }
        // Generate a simple unique id
        let id = js_sys::Math::random().to_string();
        let _ = el.set_attribute(key, &id);
        id
    }

    fn update_class_mapper_from_elements(&mut self, document: &Document) -> Result<(), StyleError> {
        let window = web_sys::window().unwrap();

        for class_name in &self.watched_classes {
            if let Ok(elements) = document.query_selector_all(&format!(".{}", class_name)) {
                for k in 0..elements.length() {
                    if let Some(node) = elements.get(k) {
                        if let Some(el) = node.dyn_ref::<Element>() {
                            let _ = Self::ensure_element_cache_id(el);
                            if let Ok(cs_opt) = window.get_computed_style(el) {
                                if let Some(cs) = cs_opt {
                                    // *** Initial read of CSS custom property --color ***
                                    if let Ok(color_value) = cs.get_property_value("--color") {
                                        if !color_value.is_empty() {
                                            let el_id = Self::ensure_element_cache_id(el);
                                            let cache_key = format!("{}#{}", class_name, el_id);
                                            self.cached_values.borrow_mut().insert(cache_key, color_value.clone());

                                            if let Ok(color) = Self::parse_css_color(&color_value) {
                                                let mut mapper = self.class_mapper.borrow_mut();
                                                mapper.add_class(
                                                    crate::types::CssClass::new(class_name.clone())
                                                        .with_color(color)
                                                );
                                                // Initial class mapping completed
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }

    fn parse_css_color(color_str: &str) -> Result<Color, StyleError> {
        let s = color_str.trim();

        // #rrggbb
        if let Some(hex) = s.strip_prefix('#') {
            if hex.len() == 6 {
                let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| StyleError::CssParseError("Invalid hex".into()))?;
                let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| StyleError::CssParseError("Invalid hex".into()))?;
                let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| StyleError::CssParseError("Invalid hex".into()))?;
                return Ok(Color::from_rgba(r, g, b, 255));
            }
        }

        // rgb/rgba(...) common path from getComputedStyle
        if s.starts_with("rgb(") || s.starts_with("rgba(") {
            // crude but effective parse
            let inside = s.trim_start_matches("rgba(").trim_start_matches("rgb(").trim_end_matches(')');
            let parts: Vec<&str> = inside.split(',').map(|p| p.trim()).collect();
            if parts.len() >= 3 {
                let r: u8 = parts[0].parse().map_err(|_| StyleError::CssParseError("rgb r".into()))?;
                let g: u8 = parts[1].parse().map_err(|_| StyleError::CssParseError("rgb g".into()))?;
                let b: u8 = parts[2].parse().map_err(|_| StyleError::CssParseError("rgb b".into()))?;
                let a: u8 = if parts.len() == 4 {
                    // alpha is 0..1 float
                    let af: f32 = parts[3].parse().unwrap_or(1.0);
                    (af.clamp(0.0,1.0) * 255.0).round() as u8
                } else { 255 };
                return Ok(Color::from_rgba(r, g, b, a));
            }
        }

        Err(StyleError::CssParseError(format!("Unsupported color format: {}", s)))
    }

    pub fn stop(&mut self) {
        if let Some(observer) = self.observer.take() {
            observer.disconnect();
        }
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
    pub fn new(_class_mapper: Rc<RefCell<ClassMapper>>) -> Self {
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