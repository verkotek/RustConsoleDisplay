use std::collections::HashMap;
use rand::seq::SliceRandom;
use crate::chunk::Chunk;
use crate::color::Color::*;
use crate::model::object::Object;
use crate::position::V2;
use crate::{chunk, settings};
use crate::display::camera_pos_center;
use crate::settings::SIZE;

pub struct Map {
    hash_map: HashMap<V2,Chunk>,
}
impl Map{
    pub fn new() -> Self {
        Map{
            hash_map: HashMap::new()
        }
    }

    pub fn get_map(&self) -> &HashMap<V2,Chunk>{
        &self.hash_map
    }

    pub fn get_chunk(&mut self, pos: &V2) -> Option<&mut Chunk> {
        self.hash_map.get_mut(pos)
    }

    pub fn update_chunks(&mut self, pos_cam: &V2){
        let visible = visible_chucks(pos_cam);

        self.hash_map.retain(|p, _| visible.contains(p));

        let mut rng = rand::thread_rng();
        let varied = [Red, Yellow, Green, Blue];
        let mut c = Chunk::new();
        c.list.push(Object::from(
            "@".to_string(),
            Red,
            V2(0,0)
        ));


        for pos in visible {
            if !self.hash_map.contains_key(&pos) {
                c.pos = pos.clone();
                c.list[0].skin.color[0] = varied.choose(&mut rng).unwrap().clone();
                c.list[0].pos = c.pos.clone();
                self.hash_map.insert(pos, c.clone());
            }
        }
    }

}

fn visible_chucks(pos: &V2) -> Vec<V2> {
    let mut chunk = Vec::new();
    let r = settings::CHUNK_RADIUS;
    for x in -r..=r {
        for y in -r..=r {
            chunk.push(  (chunk::pos_on_chunk(&camera_pos_center(&pos)) + V2(x,y))*SIZE)
        }
    }
    chunk
}


