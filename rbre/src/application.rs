use core::vulkano_context::VulkanoContext;
use std::time::Duration;
use std::thread;

use crate::geometry::triangle::Triangle;
use crate::object::abstruct_object::AbstructObject;

use std::boxed::Box;

pub struct Application {
    pub vc: VulkanoContext,
    obj : Vec<Box<dyn AbstructObject>>
}

impl Application {
    pub fn init() -> Self {
        let vc = VulkanoContext::initialize();
        let mut app = Self {
            vc,
            obj: vec![]
        };
        let object = Box::new(Triangle::new(&app)) as Box<dyn AbstructObject>;
        app.obj.push(object);
        app
    }

    pub fn run(&mut self) {
        loop {
            let mut buffer = vec![(self.obj[0].draw())];
            self.vc.draw_frame(&mut buffer);
            thread::sleep(Duration::from_millis(330));
        }
    }
}