use crate::log;
use crate::plinth_app::PlinthApp;
pub struct MyApp {}

impl PlinthApp for MyApp {
    fn init(&mut self) {
        log!("Hello from Rust!");
    }
}
