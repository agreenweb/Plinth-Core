use std::cell::RefCell;

use graphics::Rc;

mod app;
mod graphics;
mod my_app;
mod plinth_app;

fn main() {
    let user_app = Rc::new(RefCell::new(my_app::MyApp {}));
    app::start_app(user_app);
}
