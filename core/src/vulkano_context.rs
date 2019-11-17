

use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::device::{Device, DeviceExtensions, QueuesIter, Queue};
use vulkano::format::Format;
use vulkano::swapchain::{Surface};
use vulkano::image::{SwapchainImage};
use vulkano::framebuffer::{RenderPassAbstract, FramebufferAbstract, Subpass, Framebuffer};
use vulkano::pipeline::{GraphicsPipelineAbstract, viewport::Viewport, GraphicsPipeline, 
    vertex::BufferlessVertices};

use vulkano::command_buffer::{AutoCommandBuffer, AutoCommandBufferBuilder, DynamicState};
use vulkano::image::attachment::AttachmentImage;
use vulkano::sync::GpuFuture;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, BufferAccess};

use std::sync::Arc;

use crate::vulkano_window_context::VulkanoWindowContext;
use crate::vulkano_surface_context::VulkanoSurfaceContext;
use crate::shader_lib::simple::SimpleVertex;
use crate::draw_buffer::DrawBuffer;

use winit::{Window};

pub struct VulkanoContext {
    instance: Arc<Instance>,

    // window surface info
    pub window_context: VulkanoWindowContext,

    // Device Initialization
    physical_device_index: usize,
    device: Arc<Device>,
    queues: QueuesIter,
    queue: Arc<Queue>,

    // Swapchain Context
    pub surface_context: VulkanoSurfaceContext,

    // renderpass
    render_pass: Arc<dyn RenderPassAbstract + Send + Sync>,

    // TODO: move valid structure
    framebuffers: Vec<Arc<dyn FramebufferAbstract + Send + Sync>>,
    framebuffer: Arc<dyn FramebufferAbstract + Send + Sync>,
    command_buffer: Option<AutoCommandBufferBuilder>,
}

impl VulkanoContext {
    pub fn initialize() -> Self {
        let instance = Self::create_instance();
        let window_context = VulkanoWindowContext::initialize(&Some(String::from("application")), &instance);
        let surface = window_context.surface();

        let physical_device_index = Self::pick_physical_device(&instance);
        let (device, mut queues) = Self::create_device(&instance, &surface, physical_device_index);
        let queue = queues.next().unwrap();
        let surface_context = VulkanoSurfaceContext::initialize(&instance, &surface, physical_device_index, &device, &queue);
        let render_pass = Self::create_renderpass(&device, surface_context.format());
        let framebuffers = Self::create_framebuffers(&device, surface_context.dimensions(), surface_context.images(), &render_pass);
        let framebuffer = framebuffers[0].clone();
        let command_buffer = Self::create_commandbuffer(&queue, &framebuffer);
        Self {
            instance,
            window_context,
            physical_device_index,
            device,
            queues,
            queue,
            surface_context,
            render_pass,
            framebuffers,
            framebuffer,
            command_buffer,
        }

    }
    
    fn create_instance() -> Arc<Instance> {
        Instance::new(None, &VulkanoWindowContext::required_extensions(), None).expect("Failed to create Instance")
    }

        // TODO: select suitable device
    fn pick_physical_device(instance: &Arc<Instance>) -> usize {
        PhysicalDevice::enumerate(&instance).next().unwrap().index()
    }
    
    fn create_device(instance: &Arc<Instance>, surface: &Arc<Surface<Window>>, physical_device_index: usize) -> (Arc<Device>, QueuesIter){
        let physical_device = PhysicalDevice::from_index(&instance, physical_device_index).unwrap();
        let device_ext = DeviceExtensions {khr_swapchain: true, .. DeviceExtensions::none()};
        let queue_family = physical_device.queue_families().find(|&q| {
            q.supports_graphics() && surface.is_supported(q).unwrap_or(false)
        }).unwrap();

        Device::new(physical_device, &physical_device.supported_features(), &device_ext, [(queue_family, 0.5)].iter().cloned()).unwrap()
    }
 
    fn create_renderpass(device: &Arc<Device>, color_format: Format) -> Arc<dyn RenderPassAbstract + Send + Sync> {
        Arc::new(vulkano::single_pass_renderpass!(
            device.clone(),
            attachments: {
                color: {
                    load: Clear,
                    store: Store,
                    format: color_format,
                    samples: 1,
                },
                depth: {
                    load: Clear,
                    store: Store,
                    format: Format::D16Unorm,
                    samples: 1,
                }
            },
            pass : {
                color: [color],
                depth_stencil: {depth}
            }
        ).unwrap())
    }

    fn create_framebuffers(device: &Arc<Device>, dimensions: [u32;2], images: &[Arc<SwapchainImage<Window>>],render_pass: &Arc<dyn RenderPassAbstract + Send + Sync>) 
        ->  Vec<Arc<dyn FramebufferAbstract + Send + Sync>> {
            println!("Called here");
            let depth_buffer = AttachmentImage::transient(device.clone(), dimensions, Format::D16Unorm).unwrap();
            println!("{}", images.len());
            images.iter().map(|image| {
                Arc::new(
                    Framebuffer::start(render_pass.clone())
                        .add(image.clone()).unwrap()
                        .add(depth_buffer.clone()).unwrap()
                        .build().unwrap()
                ) as Arc<dyn FramebufferAbstract + Send + Sync>
            }).collect::<Vec<_>>()
    }

    fn create_command_buffer_self(&mut self) {
        self.command_buffer = Self::create_commandbuffer(&self.queue, &self.framebuffer);
    }

    fn create_commandbuffer(queue: &Arc<Queue>, framebuffer: &Arc<dyn FramebufferAbstract + Send + Sync>) -> Option<AutoCommandBufferBuilder> {
        Some(AutoCommandBufferBuilder::primary_one_time_submit(queue.device().clone(), 
            queue.family()).unwrap()
            .begin_render_pass(framebuffer.clone(), false, 
                vec![[1.0, 1.0, 1.0, 1.0].into(), 1.0f32.into()]).unwrap())
    }

    pub fn draw_frame(&mut self, buffer: &mut Vec<&mut DrawBuffer>) {
        let (image_index, acquire_future) = self.surface_context.acquire_next_image();
        
        self.framebuffer = self.framebuffers[image_index].clone();

        self.create_command_buffer_self();

        unsafe {
            let command_buffer = buffer[0].draw_buffer(self.surface_context.dimensions());
            self.command_buffer =Some(self.command_buffer.take().unwrap().execute_commands(command_buffer).unwrap());
        }

        let command_buffer = self.command_buffer.take().unwrap().end_render_pass().unwrap().build().unwrap();
        let queue = self.surface_context.queue();


        let future = acquire_future
            .then_execute(self.queue.clone(), command_buffer).unwrap()
            .then_swapchain_present(queue.clone(), self.surface_context.swapchain().clone(), image_index)
            .then_signal_fence_and_flush()
            .unwrap();
        future.wait(None).unwrap();
    }

    pub fn queue(&self) -> Arc<Queue> {
        self.queue.clone()
    }

    pub fn subpass(&self) -> Subpass<Arc<dyn RenderPassAbstract + Send + Sync>> {
        Subpass::from(self.render_pass.clone(), 0).unwrap()
    }
}