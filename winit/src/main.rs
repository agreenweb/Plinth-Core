use plinth_core::{app::start_app, plinth_app::PlinthApp, plinth_app::PlinthRenderer};
use std::cell::RefCell;
use std::sync::Arc;

struct TestApp {
    frame_count: u32,
}

impl TestApp {
    fn new() -> Self {
        Self { frame_count: 0 }
    }
}

impl PlinthApp for TestApp {
    fn init(&mut self) {
        println!("Test app initialized!");
    }

    fn before_render(&mut self) {
        self.frame_count += 1;
        if self.frame_count % 60 == 0 {
            println!("Frame: {}", self.frame_count);
        }
    }

    fn after_render(&mut self) {
        // Optional: Add any post-render logic here
    }

    fn on_close(&mut self) {
        println!("Test app closing after {} frames", self.frame_count);
    }
}

impl PlinthRenderer for TestApp {
    // The default render implementation from the trait will render the triangle
    // We can override it if we want custom rendering
}

fn main() {
    env_logger::init();
    
    let test_app = TestApp::new();
    let app_rc = Arc::new(RefCell::new(test_app));
    start_app(app_rc);
}
