use core;

struct Vertex {
    position: [f32; 3]
}

core::vulkano::impl_vertex!(Vertex, position);

pub struct ShaderObject<T> {
    vs: String,
    fs: String,
    data: <T>
}

impl<T> ShaderObject<T> {
    pub fn new(vs: String, fs: String, data: T) {
    }

    pub fn draw() {
        
    }
}