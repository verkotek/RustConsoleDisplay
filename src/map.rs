use std::ops::{Sub};
use crate::object::Object;

pub struct Map(pub Vec<Object>);

impl Map{
    pub fn new() -> Self {
        Map(vec![])
    }
}
pub struct V2(pub i32,pub i32);

impl Sub for V2 {
    type Output = V2;

    fn sub(self, rhs: Self) -> Self::Output {
        V2(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Sub for &V2 {
    type Output = V2;

    fn sub(self, rhs: Self) -> Self::Output {
        V2(self.0 - rhs.0, self.1 - rhs.1)
    }
}