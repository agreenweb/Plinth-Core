use plinth::graphics::Rc;
use std::cell::RefCell;
mod my_app;
mod my_rendering;
mod plinth;

fn main() {
    let user_app = Rc::new(RefCell::new(my_app::MyApp {}));
    plinth::app::start_app(user_app);
}
