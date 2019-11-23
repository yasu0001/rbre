use vulkano::buffer::{BufferAccess, BufferUsage, CpuAccessibleBuffer};
pub use vulkano::command_buffer::AutoCommandBuffer;
use vulkano::command_buffer::{AutoCommandBufferBuilder, DynamicState};
use vulkano::device::Queue;
use vulkano::framebuffer::{RenderPassAbstract, Subpass};
use vulkano::pipeline::{viewport::Viewport, GraphicsPipeline, GraphicsPipelineAbstract};

use std::sync::Arc;

use crate::shader_lib::simple::SimpleVertex;

pub struct DrawBuffer {
    gfx_queue: Arc<Queue>,
    vertex_buffer: Arc<dyn BufferAccess + Send + Sync>,
    pipeline: Arc<GraphicsPipelineAbstract + Send + Sync>,
}

impl DrawBuffer {
    pub fn new<R>(gfx_queue: Arc<Queue>, subpass: Subpass<R>, position: &Vec<[f32; 3]>) -> Self
    where
        R: RenderPassAbstract + Send + Sync + 'static,
    {
        mod vs {
            vulkano_shaders::shader! {
                ty: "vertex",
                path: "../shaders/simple.vert"
            }
        }

        mod fs {
            vulkano_shaders::shader! {
                ty: "fragment",
                path: "../shaders/simple.frag"
            }
        }

        let vertex_buffer = {
            let vertex: Vec<SimpleVertex> =
                position.iter().map(|p| SimpleVertex::init(*p)).collect();
            CpuAccessibleBuffer::from_iter(
                gfx_queue.device().clone(),
                BufferUsage::all(),
                vertex.iter().cloned(),
            )
            .expect("Failed to create buffer")
        };

        let pipeline = {
            let vs = vs::Shader::load(gfx_queue.device().clone())
                .expect("Failed to create shader module");
            let fs = fs::Shader::load(gfx_queue.device().clone())
                .expect("Failed to create shader module");

            Arc::new(
                GraphicsPipeline::start()
                    .vertex_input_single_buffer::<SimpleVertex>()
                    .vertex_shader(vs.main_entry_point(), ())
                    .triangle_list()
                    .viewports_dynamic_scissors_irrelevant(1)
                    .fragment_shader(fs.main_entry_point(), ())
                    .depth_stencil_simple_depth()
                    .render_pass(subpass)
                    .build(gfx_queue.device().clone())
                    .unwrap(),
            )
        };

        Self {
            gfx_queue,
            vertex_buffer,
            pipeline,
        }
    }

    pub fn draw_buffer(&self, viewport_dimensions: [u32; 2]) -> AutoCommandBuffer {
        AutoCommandBufferBuilder::secondary_graphics(
            self.gfx_queue.device().clone(),
            self.gfx_queue.family(),
            self.pipeline.clone().subpass(),
        )
        .unwrap()
        .draw(
            self.pipeline.clone(),
            &DynamicState {
                viewports: Some(vec![Viewport {
                    origin: [0.0, 0.0],
                    dimensions: [viewport_dimensions[0] as f32, viewport_dimensions[1] as f32],
                    depth_range: 0.0..1.0,
                }]),
                ..DynamicState::none()
            },
            vec![self.vertex_buffer.clone()],
            (),
            (),
        )
        .unwrap()
        .build()
        .unwrap()
    }
}
