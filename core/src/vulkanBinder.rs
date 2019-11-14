
use vulkano::pipeline::{GraphicsPipelineAbstract};
use vulkano::framebuffer::{FramebufferAbstract};
use vulkano::sync::GpuFuture;

struct VulkanBinder {
    frame: Box<dyn GpuFuture>,
    pipeline: Arc<dyn GraphicsPipelineAbstract + Send + Sync>
    framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>
}

impl VulkanBinder {
    
}