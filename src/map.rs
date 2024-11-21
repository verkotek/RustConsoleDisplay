use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash, Hasher};
use rand::{Rng, rngs::StdRng, SeedableRng, seq::SliceRandom};
use crate::chunk::Chunk;
use crate::color::Color::*;
use crate::model::object::Object;
use crate::position::V2;
use crate::{chunk, settings};
use crate::display::camera_pos_center;
use crate::model::list::load_model;
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

        let seed = 1;

        let varied = [Red, Yellow, Green, Blue];

        let mut chunk = Chunk::new();

        for pos in visible {
            if self.hash_map.contains_key(&pos) { continue; }

            let mut def = DefaultHasher::new();
            chunk.pos = pos.clone();
            let obj = load_model().get(2).unwrap().clone();
            for x in 0..SIZE.0 {
                for y in 0..SIZE.1 {

                    pos.hash(&mut def);
                    seed.hash(&mut def);
                    V2(x,y).hash(&mut def);
                    let mut rng = StdRng::seed_from_u64(def.finish());
                    let f = rng.gen::<f64>();
                    if f > 0.999 {
                        chunk.list.push(Object::create(
                            &obj,
                            varied.choose(&mut rng).unwrap().clone(),
                            pos.clone() + V2(x, y)
                        ));
                    }
                    else if f > 0.9 {
                        chunk.list.push(Object::from(
                            "@".to_string(),
                            varied.choose(&mut rng).unwrap().clone(),
                            pos.clone() + V2(x, y)
                        ));
                    }
                }
            }
            self.hash_map.insert(pos, chunk.clone());
        }
    }

}

fn visible_chucks(pos: &V2) -> Vec<V2> {
    let mut chunk = Vec::new();
    let r = settings::CHUNK_RADIUS;
    for x in -r..=r {
        for y in -r..=r {
            chunk.push((chunk::pos_on_chunk(&camera_pos_center(&pos)) + V2(x,y))*SIZE)
        }
    }
    chunk
}

fn gen_structure(i: usize){

}


