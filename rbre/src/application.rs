use core::vulkano_context::VulkanoContext;
use std::time::Duration;
use std::thread;

use crate::geometry::object3d::Object3D;
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
        let object = Box::new(Object3D::new(&app)) as Box<dyn AbstructObject>;
        app.obj.push(object);
        let mut object = Box::new(Object3D::new(&app));
        object.set_pos(vec![
            [0.0, 0.0, 1.0],
            [0.3, 0.1, 0.0], 
            [0.3, 0.3, 0.0]
        ], &app);
        app.obj.push(object);
        app
    }

    pub fn run(&mut self) {
        loop {

            let buffer = self.obj.iter().map(|o| {
                o.draw()
            }).collect::<Vec<_>>();
            self.vc.draw_frame(buffer);
            thread::sleep(Duration::from_millis(330));
        }
    }
}