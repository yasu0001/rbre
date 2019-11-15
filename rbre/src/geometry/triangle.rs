use core::shaderLib::simple::SimpleVertex;

pub struct Triangle {
    pos: Vec<[f32;3]>,
};

impl Triangle {
    pub fn init() -> Self {
        let pos = vec![
            [-0.5, -0.25, 0.0],
            [0.0, 0.5, 0.0],
            [0.25, -0.1],
        ];
        Self {
            pos
        }
    }
}