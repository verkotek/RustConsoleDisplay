use crate::model::object::Object;

pub struct Map(pub Vec<Object>);

impl Map{
    pub fn new() -> Self {
        Map(vec![])
    }
}
