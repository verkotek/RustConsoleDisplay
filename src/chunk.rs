use crate::model::object::Object;
use crate::position::V2;

#[derive(Clone)]
pub struct Chunk {
    pub list: Vec<Object>,
    pub pos: V2,
}

