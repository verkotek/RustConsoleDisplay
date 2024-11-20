use core::array;
use crate::chunk::Chunk;
use crate::model::object::Object;
use crate::position::V2;

const SIZE: (i32, i32) = (7, 3);
const LIST: usize = 900;
const CUBE: i32 = 30;
// 3^2 = 9
pub struct Map(pub [Chunk; LIST]);
// size 32
impl Map{
    pub fn new() -> Box<Self> {
        let mut ch = Chunk{list: vec![], pos: V2(0,0)};
        let a = array::from_fn::<Chunk,LIST,_>(|i| {let mut a = ch.clone(); a.pos = V2(i as i32 % CUBE, i as i32 / CUBE); a});
        Box::new(Map(a))
    }

    pub fn installation_object(&mut self, object: Object) -> &mut Map {
        let pos = pos_on_chunk(&object); //V2(object.pos.0 / SIZE, object.pos.1 / SIZE);
        if pos.0 >= CUBE || pos.1 >= CUBE { return self;  }
        let pos = (pos.0 + pos.1 * CUBE) as usize;
        // if pos >= self.0.len() { return self }

        self.0[pos].list.push(object);
        self
    }
}

pub fn pos_on_chunk(object: &Object) -> V2{
    V2(object.pos.0 / SIZE.0, object.pos.1 / SIZE.1)
}
