use vulkano::instance::{Instance, PhysicalDevice};
use vulkano::device::{Queue, Device};
use vulkano::image::SwapchainImage;
use vulkano::swapchain::{Swapchain, Surface, SurfaceTransform, PresentMode, SwapchainAcquireFuture};
use vulkano::swapchain;
use vulkano::format::Format;
use winit::Window;


use std::sync::Arc;

pub struct VulkanoSurfaceContext {
    queue: Arc<Queue>,
    swapchain: Arc<Swapchain<Window>>,
    images: Vec<Arc<SwapchainImage<Window>>>,

    // rendering images
}

impl VulkanoSurfaceContext {
    pub fn initialize(
        instance: &Arc<Instance>,
        surface: &Arc<Surface<Window>>,
        physica_device_index: usize,
        device: &Arc<Device>,
        queue: &Arc<Queue>,
    ) -> Self {
        let queue = queue.clone();
        let (swapchain, images) = Self::create_swapchain(instance, surface, physica_device_index, device, &queue);
        Self {
            queue,
            swapchain,
            images
        }
    }

    fn create_swapchain (
        instance: &Arc<Instance>,
        surface: &Arc<Surface<Window>>,
        physica_device_index: usize,
        device: &Arc<Device>,
        queue: &Arc<Queue>,
    ) -> (Arc<Swapchain<Window>>, Vec<Arc<SwapchainImage<Window>>>) {
        let window = surface.window();
        let physical = PhysicalDevice::from_index(instance, physica_device_index).unwrap();

        let caps = surface
            .capabilities(physical)
            .expect("Failed to get surface");
        let usage = caps.supported_usage_flags;
        let alpha = caps.supported_composite_alpha.iter().next().unwrap();
        let format = caps.supported_formats[0].0;
        let initial_dimensions = if let Some(dimensions) = window.get_inner_size() {
            let dimensions: (u32, u32) = dimensions.to_physical(window.get_hidpi_factor()).into();
            [dimensions.0, dimensions.1]
        } else {
            panic!("window no longer exists");
        };

        Swapchain::new(
            device.clone(),
            surface.clone(),
            caps.min_image_count,
            format,
            initial_dimensions,
            1,
            usage,
            queue,
            SurfaceTransform::Identity,
            alpha,
            PresentMode::Fifo,
            true,
            None,
        )
        .unwrap()
    }

    pub fn format(&self) -> Format {
        self.swapchain.format()
    }

    pub fn dimensions(&self) -> [u32; 2] {
        self.images[0].dimensions()
    }

    pub fn images(&self) -> &[Arc<SwapchainImage<Window>>] {
        &self.images
    }

    pub fn acquire_next_image(&self) -> (usize, SwapchainAcquireFuture<Window>){
        swapchain::acquire_next_image(self.swapchain.clone(),None).unwrap()
    }

    pub fn queue(&self) -> &Arc<Queue> {
        &self.queue
    }

    pub fn swapchain(&self) -> &Arc<Swapchain<Window>> {
        &self.swapchain
    }
}
