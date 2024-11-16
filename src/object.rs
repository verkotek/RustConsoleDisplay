use crate::color::Color;
use crate::map::V2;

pub struct Object{
    pub skin: char,
    pub color: Color,
    pub pos: V2,
}

pub struct Skin{
    chars: Vec<char>,
    color: Color,
}