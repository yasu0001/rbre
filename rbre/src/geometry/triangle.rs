
use core::draw_buffer::DrawBuffer;
use core::draw_buffer::AutoCommandBuffer;

use crate::object::abstruct_object::AbstructObject;
use crate::application::Application;

use std::sync::Arc;

pub struct Triangle {
    pos: Vec<[f32;3]>,
    buffer: DrawBuffer,
}

impl Triangle {
    pub fn new(app: &Application) -> Self {
        let pos = vec![
            [0.0, 0.0, 1.0],
            [0.3, 0.1, 0.0], 
            [0.3, 0.3, 0.0]
        ];
        let buffer =Self::init(app, &pos);
        Self {
            pos,
            buffer
        }
    }
}

impl AbstructObject for Triangle {
    fn draw(&mut self) -> &mut DrawBuffer {
        &mut self.buffer
    }
}