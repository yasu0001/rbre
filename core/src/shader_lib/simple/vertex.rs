
#[derive(Default, Debug, Clone)]
pub struct SimpleVertex {
    position: [f32;3]
}

vulkano::impl_vertex!(SimpleVertex, position);

impl SimpleVertex {

    pub fn init(position: [f32; 3]) -> Self {
        Self {
            position
        }
    }
}