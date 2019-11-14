
use vulkano::instance::{InstanceExtensions, Instance};
use vulkano::swapchain::{Surface};

use winit::{Window, WindowBuilder, EventsLoop};
use vulkano_win::VkSurfaceBuild;


use std::sync::Arc;

pub struct VulkanoWindowContext {
    events_loop: EventsLoop,
    surface: Arc<Surface<Window>>,
}

impl VulkanoWindowContext {
    pub fn initialize(appname: &Option<String>, instance: &Arc<Instance>) -> Self {
        let events_loop = EventsLoop::new();

        let surface = if let Some(name) = appname {
            WindowBuilder::new()
                .with_title(name)
                .build_vk_surface(&events_loop, instance.clone())
                .unwrap()
        } else {
            WindowBuilder::new().build_vk_surface(&events_loop, instance.clone()).unwrap()
        };

        Self {
            events_loop, surface
        }
    }

    pub fn required_extensions() ->  InstanceExtensions {
        vulkano_win::required_extensions()
    }

    pub fn surface(&self) -> &Arc<Surface<Window>> {
        &self.surface
    }

    pub fn events_loop(&self) -> &EventsLoop {
        &self.events_loop
    }
}