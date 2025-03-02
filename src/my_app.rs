use crate::plinth_app::PlinthApp;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct MyApp {}

impl PlinthApp for MyApp {
    fn init(&mut self) {
        log("Hello from Rust!");
    }
}
