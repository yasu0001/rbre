extern crate rbre;

use rbre::application::Application;

use core;



fn main() {
    let mut app = Application::init();
    app.run();

    struct Vertex {
        position: [f32; 3]
    }
}