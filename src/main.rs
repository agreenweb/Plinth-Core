use graphics::Rc;
use std::cell::RefCell;

mod app;
mod graphics;
mod my_app;
mod plinth_app;
mod util;

fn main() {
    let user_app = Rc::new(RefCell::new(my_app::MyApp {}));
    app::start_app(user_app);
}
