use crate::map::V2;

pub fn gen(max: f64) -> i32 { (rand::random::<f64>() * max) as i32 }

pub fn gen_v2(max1: f64, max2: f64) -> V2{
    V2(gen(max1), gen(max2))
}