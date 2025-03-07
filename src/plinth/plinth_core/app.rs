use crate::plinth::graphics::{create_graphics, Graphics, Rc};
use crate::plinth::plinth_app::PlinthApp;
use std::cell::RefCell;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop, EventLoopProxy},
    window::{Window, WindowId},
};

enum State {
    Ready(Graphics),
    Init(Option<EventLoopProxy<Graphics>>),
}

pub struct App {
    _title: String,
    state: State,
    user_app: Rc<RefCell<dyn PlinthApp>>,
}

impl App {
    pub fn new(event_loop: &EventLoop<Graphics>, user_app: Rc<RefCell<dyn PlinthApp>>) -> Self {
        Self {
            _title: "WebGPU Example".to_string(),
            state: State::Init(Some(event_loop.create_proxy())),
            user_app,
        }
    }

    fn draw(&mut self) {
        if let State::Ready(gfx) = &mut self.state {
            self.user_app.borrow_mut().before_render();
            self.user_app.borrow_mut().render(gfx);
            self.user_app.borrow_mut().after_render();
        }
    }

    fn resized(&mut self, size: PhysicalSize<u32>) {
        if let State::Ready(gfx) = &mut self.state {
            gfx.resize(size);
            self.user_app.borrow_mut().render(gfx);
        }
    }

    pub fn _set_title(&mut self, title: &str) {
        self._title = title.to_string();
    }
}

impl ApplicationHandler<Graphics> for App {
    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::Resized(size) => self.resized(size),
            WindowEvent::RedrawRequested => self.draw(),

            WindowEvent::CloseRequested => {
                self.user_app.borrow_mut().on_close();
                event_loop.exit()
            }
            _ => {}
        }
        self.user_app
            .borrow_mut()
            .event_handler(event_loop, _window_id, &event);
    }

    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let State::Init(proxy) = &mut self.state {
            if let Some(proxy) = proxy.take() {
                let mut win_attr = Window::default_attributes();

                #[cfg(not(target_arch = "wasm32"))]
                {
                    win_attr = win_attr.with_title(self._title.as_str());
                }

                #[cfg(target_arch = "wasm32")]
                {
                    use winit::platform::web::WindowAttributesExtWebSys;
                    win_attr = win_attr.with_append(true);
                }

                let window = Rc::new(
                    event_loop
                        .create_window(win_attr)
                        .expect("create window err."),
                );

                let user_app = Rc::clone(&self.user_app);

                #[cfg(target_arch = "wasm32")]
                wasm_bindgen_futures::spawn_local(create_graphics(window, proxy, user_app));

                #[cfg(not(target_arch = "wasm32"))]
                pollster::block_on(create_graphics(window, proxy, user_app));
            }
        }
    }

    fn user_event(&mut self, _event_loop: &ActiveEventLoop, graphics: Graphics) {
        self.state = State::Ready(graphics);
        if let State::Ready(gfx) = &mut self.state {
            let scale_factor = gfx.window.scale_factor();
            let logical_size = gfx.window.inner_size();
            let physical_size = winit::dpi::PhysicalSize::new(
                (logical_size.width as f64 * scale_factor) as u32,
                (logical_size.height as f64 * scale_factor) as u32,
            );
            self.resized(physical_size);
        }
        self.user_app.borrow_mut().init();
    }
}

pub fn start_app(user_app: Rc<RefCell<dyn PlinthApp>>) {
    let event_loop = EventLoop::<Graphics>::with_user_event().build().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let app = App::new(&event_loop, user_app);
    run_app(event_loop, app);
}

#[cfg(target_arch = "wasm32")]
fn run_app(event_loop: EventLoop<Graphics>, app: App) {
    // Sets up panics to go to the console.error in browser environments
    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    console_log::init_with_level(log::Level::Error).expect("Couldn't initialize logger");

    // Runs the app async via the browsers event loop
    use winit::platform::web::EventLoopExtWebSys;
    wasm_bindgen_futures::spawn_local(async move {
        event_loop.spawn_app(app);
    });
}

#[cfg(not(target_arch = "wasm32"))]
fn run_app(event_loop: EventLoop<Graphics>, mut app: App) {
    // Allows the setting of the log level through RUST_LOG env var.
    // It also allows wgpu logs to be seen.
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("error")).init();

    // Runs the app on the current thread.
    let _ = event_loop.run_app(&mut app);
}
