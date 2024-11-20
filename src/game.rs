
use crate::chunk::{pos_on_chunk};
use crate::display::Display;
use crate::model::object::Object;
use crate::map::Map;
use crate::position::V2;

pub struct Game{
    pub display: Display,
    pub map: Map,

    pub camera: Object,
}

impl Game{
    pub fn new() -> Self{
        let mut camera =Object::new();
        camera.pos = V2(0,4);
        Game{
            display: Display::new(),
            map: Map::new(),

            camera
        }
    }
    pub fn installation(&mut self, object: Object) -> &mut Game {
        let pos = pos_on_chunk(&object.pos);
        let chunk = self.map.get_chunk(&pos);
        match chunk {
            None => { return self }
            Some(a) => { a.list.push(object);}
        }

        self
    }

    pub fn update(&mut self, f: fn(&mut Game) -> bool){
        loop {
            // match f(self) {
            //     0 => {break}
            //     _ => {}
            // }
            if !f(self) { break; }
        }
    }
}