use core::draw_buffer::DrawBuffer;
use core::draw_buffer::AutoCommandBuffer;
use crate::application::Application;

use std::sync::Arc;

pub trait AbstructObject {
    //fn draw(&mut self)-> AutoCommandBuffer ;
    fn init(app: &Application, position: &Vec<[f32; 3]>) -> DrawBuffer where Self: Sized {
        DrawBuffer::new(app.vc.queue(), app.vc.subpass(), position)
    }
    fn draw(&mut self) -> &mut DrawBuffer;
}