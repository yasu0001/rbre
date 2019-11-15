

use vulkano::instance::{Instance, PhysicalDevice, RawInstanceExtensions, InstanceCreationError, InstanceExtensions};
use vulkano::device::{Device, DeviceExtensions, QueuesIter, Queue};
use vulkano::format::Format;
use vulkano::swapchain::{Surface, Swapchain, PresentMode, SurfaceTransform};
use vulkano::image::{SwapchainImage};
use vulkano::framebuffer::{RenderPassAbstract, FramebufferAbstract, Subpass, Framebuffer};
use vulkano::swapchain;
use vulkano::pipeline::{GraphicsPipelineAbstract, viewport::Viewport, GraphicsPipeline, 
    vertex::BufferlessDefinition, vertex::BufferlessVertices};
use vulkano_win::VkSurfaceBuild;
use vulkano::command_buffer::{AutoCommandBuffer, AutoCommandBufferBuilder, DynamicState};
use vulkano::image::attachment::AttachmentImage;
use vulkano::sync::GpuFuture;
use vulkano::buffer::{BufferUsage, CpuAccessibleBuffer, BufferAccess};

use std::sync::Arc;
use std::iter;

use crate::vulkanoWindowContext::VulkanoWindowContext;
use crate::vulkanoSurfaceContext::VulkanoSurfaceContext;
use crate::shaderLib::simple::SimpleVertex;


use winit::{Window, WindowBuilder, EventsLoop};

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
    render_pass: Arc<RenderPassAbstract + Send + Sync>,

    // TODO: move valid structure
    graphics_pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
    framebuffers: Vec<Arc<FramebufferAbstract + Send + Sync>>,
    command_buffers: Vec<Arc<AutoCommandBuffer>>,
    vertex_buffer: Arc<BufferAccess + Send + Sync>,
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
        let graphics_pipeline = Self::create_graphics_pipeline(&device, surface_context.dimensions(), &render_pass);
        let framebuffers = Self::create_framebuffers(&device, surface_context.dimensions(), surface_context.images(), &render_pass);
        let vertex_buffer = Self::create_vertexbuffer(&device);
        let mut app = VulkanoContext {
            instance,
            window_context,
            physical_device_index,
            device,
            queues,
            queue,
            surface_context,
            render_pass,
            graphics_pipeline,
            framebuffers,
            vertex_buffer,
            command_buffers: vec![],
        };
        app.create_commandbuffer();
        app

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
 
    fn create_renderpass(device: &Arc<Device>, color_format: Format) -> Arc<RenderPassAbstract + Send + Sync> {
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

    fn create_vertexbuffer(device: &Arc<Device>) -> Arc<BufferAccess + Send + Sync> {
        let vertex = [
            SimpleVertex::init([0.0, 0.0, 1.0]),
            SimpleVertex::init([0.3, 0.1, 0.0]),
            SimpleVertex::init([0.3, 0.3, 0.0]),
            SimpleVertex::init([-1.0, -0.5, 0.0]),
            SimpleVertex::init([0.0, -1.0, 0.0]),
            SimpleVertex::init([-0.5, -0.5, 0.0]),
            ];
        CpuAccessibleBuffer::from_iter(device.clone(), 
            BufferUsage::all(), 
            vertex.iter().cloned()).unwrap()
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

    fn create_commandbuffer(&mut self) {
        self.command_buffers = self.framebuffers.iter().map(
            |framebuffer| {
                let vertices = BufferlessVertices {vertices: 3, instances: 1};
                Arc::new(AutoCommandBufferBuilder::primary_simultaneous_use(
                    self.device.clone(), self.queue.family()).unwrap()
                    .begin_render_pass(framebuffer.clone(), false, vec![[1.0, 1.0, 1.0, 1.0].into(), 1f32.into()])
                    .unwrap()
                    .draw(self.graphics_pipeline.clone(),&DynamicState::none(), 
                        vec![self.vertex_buffer.clone()], (), ())
                    .unwrap()
                    .end_render_pass()
                    .unwrap()
                    .build().unwrap())
            }
        ).collect();
    }
    fn create_graphics_pipeline(
        device: &Arc<Device>,
        swap_chain_extent: [u32; 2],
        render_pass: &Arc<RenderPassAbstract + Send + Sync>,
    ) -> Arc<GraphicsPipelineAbstract + Send + Sync> {
        mod vertex_shader {
            vulkano_shaders::shader! {
               ty: "vertex",
               path: "../shaders/simple.vert"
            }
        }

        mod fragment_shader {
            vulkano_shaders::shader! {
                ty: "fragment",
                path: "../shaders/simple.frag"
            }
        }

        let vert_shader_module = vertex_shader::Shader::load(device.clone())
            .expect("failed to create vertex shader module!");
        let frag_shader_module = fragment_shader::Shader::load(device.clone())
            .expect("failed to create fragment shader module!");

        let dimensions = [swap_chain_extent[0] as f32, swap_chain_extent[1] as f32];
        let viewport = Viewport {
            origin: [0.0, 0.0],
            dimensions,
            depth_range: 0.0 .. 1.0,
        };

        Arc::new(GraphicsPipeline::start()
            .vertex_input_single_buffer::<SimpleVertex>()
            .vertex_shader(vert_shader_module.main_entry_point(), ())
            .triangle_list()
            .primitive_restart(false)
            .viewports(vec![viewport]) // NOTE: also sets scissor to cover whole viewport
            .fragment_shader(frag_shader_module.main_entry_point(), ())
            .depth_clamp(false)
            // NOTE: there's an outcommented .rasterizer_discard() in Vulkano...
            .polygon_mode_fill() // = default
            .line_width(1.0) // = default
            .cull_mode_back()
            .front_face_clockwise()
            // NOTE: no depth_bias here, but on pipeline::raster::Rasterization
            .blend_pass_through() // = default
            .render_pass(Subpass::from(render_pass.clone(), 0).unwrap())
            .build(device.clone())
            .unwrap()
        )
    }

    pub fn draw_frame(&mut self) {
        let (image_index, acquire_future) = self.surface_context.acquire_next_image();
        let command_buffer = self.command_buffers[image_index].clone();
        let queue = self.surface_context.queue();
        let future = acquire_future
            .then_execute(queue.clone(), command_buffer).unwrap()
            .then_swapchain_present(queue.clone(), self.surface_context.swapchain().clone(), image_index)
            .then_signal_fence_and_flush()
            .unwrap();
        future.wait(None).unwrap();
    }
}