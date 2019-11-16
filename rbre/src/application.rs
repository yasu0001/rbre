use core::vulkano_context::VulkanoContext;
use std::time::Duration;
use std::thread;

use crate::object;

pub struct Application {
    vc: VulkanoContext,
}

impl Application {
    pub fn init() -> Self {
        let vc = VulkanoContext::initialize();
        Self {
            vc
        }
    }

    pub fn run(&mut self) {
        loop {
            self.vc.draw_frame();
            thread::sleep(Duration::from_millis(330));
        }
    }
}