use crate::model::object::Object;
use crate::position::V2;
use crate::settings;
#[derive(Clone, Debug)]
pub struct Chunk {
    pub list: Vec<Object>,
    pub pos: V2,
}

impl Chunk{
    pub fn new() -> Self{
        Chunk{list: vec![], pos: V2(0,0)}
    }
}

pub fn pos_on_chunk(pos: &V2) -> V2{
    V2(pos.0 / settings::SIZE.0, pos.1 / settings::SIZE.1)
}