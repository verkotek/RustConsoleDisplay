use std::ops::{Add, AddAssign, Mul, Sub};

#[derive(Debug, Clone, Hash, Eq)]
pub struct V2(pub i32, pub i32);

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
impl Add for V2 {
    type Output = V2;

    fn add(self, rhs: Self) -> Self::Output {
        V2(self.0 + rhs.0, self.1 + rhs.1)
    }
}
impl PartialEq for V2 {
    fn eq(&self, other: &Self) -> bool {
        if self.0 == other.0 && self.1 == other.1 { return true }
        false
    }
}
impl AddAssign<(u16, u16)> for V2 {
    fn add_assign(&mut self, rhs: (u16, u16)) {
        self.0 += rhs.0 as i32;
        self.1 += rhs.1 as i32;
    }
}
impl Mul<V2> for V2 {
    type Output = V2;

    fn mul(self, rhs: V2) -> Self::Output {
        V2(self.0 * rhs.0, self.1 * rhs.1)
    }
}