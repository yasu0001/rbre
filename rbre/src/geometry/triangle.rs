use core::shaderLib::simple::SimpleVertex;
use crate::object::abstruct_object::AbstructObject;

pub struct Triangle {
    pos: Vec<[f32;3]>,
};

impl Triangle {
    pub fn init() -> Self {
        Self {
            pos
        }
    }
}

impl AbstructObject for Triangle {
    fn init() -> Self {
        let pos = vec![
            [-0.5, -0.25, 0.0],
            [0.0, 0.5, 0.0],
            [0.25, -0.1],
        ];
    }
}