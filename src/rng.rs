use crate::position::V2;

trait Rand{
    fn gen(&self, max: f64) -> Self;
}

impl Rand for i32 {
    fn gen(&self, max: f64) -> Self {
        (r() * max) as i32
    }
}
impl Rand for f64{
    fn gen(&self, max: f64) -> Self {
        r() * max
    }
}
impl Rand for usize{
    fn gen(&self, max: f64) -> Self {
        (r() * max) as usize
    }
}
fn r () -> f64{
    rand::random::<f64>()
}
pub fn gen<T: Rand>(value: T, max: f64) -> T { value.gen(max) }

// pub fn gen(max: f64) -> i32 { (rand::random::<f64>() * max) as i32 }

pub fn gen_v2(max1: f64, max2: f64) -> V2{
    V2(gen(0,max1), gen(0,max2))
}