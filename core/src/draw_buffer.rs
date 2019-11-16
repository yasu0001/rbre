use vulkano::device::Queue;
use vulkano::pipeline::{GraphicsPipeline, GraphicsPipelineAbstract};
use vulkano::command_buffer::{AutoCommandBuffer, AutoCommandBufferBuilder, BufferAccess};

use std::sync::Arc;

pub struct DrawBuffer {
    gfx_queue: Arc<Queue>,
    vertex_buffer: Arc<dyn BufferAccess + Send + Sync>,
    pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
}

impl DrawBuffer {
    pub fn new<R, T>(gfx_queue: &Arc<Queue>, subpass: Subpass<R>) -> DrawBuffer 
        where R: Arc<GraphicsPipelineAbstract + Send + Sync> {
    }
}