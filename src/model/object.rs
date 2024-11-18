use crate::color::Color;
use crate::position::V2;

#[derive(Debug, Clone)]
pub struct Object{
    pub skin: Skin,
    pub pos: V2,
}

impl Object{
    pub fn new() -> Self{
        let skin = Skin{
            chars: vec![],
            color: vec![]
        };
        Object{
            skin,
            pos: V2(0,0)
        }
    }
    pub fn from(model: String, color: Color, pos: V2) -> Self{
        let skin = Skin{
            chars: model.chars().collect(),
            color: vec![color; model.len()]
        };
        Object{
            skin,
            pos
        }
    }
    pub fn create(object: &Object, color: Color, v2: V2) -> Object{
        let mut obj = object.clone();
        obj.skin.color = vec![color; obj.skin.chars.len()];
        obj.pos = v2;
        obj
    }
}

#[derive(Debug, Clone)]
pub struct Skin{
    pub chars: Vec<char>,
    pub color: Vec<Color>,
}