
use core::draw_buffer::DrawBuffer;
use core::draw_buffer::AutoCommandBuffer;

use crate::object::abstruct_object::AbstructObject;
use crate::application::Application;

use std::sync::Arc;

pub struct Object3D {
    pos: Vec<[f32;3]>,
    buffer: DrawBuffer,
}

impl Object3D {
    pub fn new(app: &Application) -> Self {
        let pos = vec![
            [0.0, 0.0, 0.0],
            [-1.0, 0.0, 0.0],
            [-0.3, -0.5, 0.0],
        ];
        let buffer =Self::init(app, &pos);
        Self {
            pos,
            buffer
        }
    }

    pub fn set_pos(&mut self, pos: Vec<[f32;3]>, app: &Application) {
        self.pos = pos;
        self.buffer = Self::init(app, &self.pos);
    }
}

impl AbstructObject for Object3D {
    fn draw(&self) -> &DrawBuffer {
        &self.buffer
    }
}
