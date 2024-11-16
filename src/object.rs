use crate::color::Color;
use crate::map::V2;

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
}

#[derive(Debug, Clone)]
pub struct Skin{
    pub chars: Vec<char>,
    pub color: Vec<Color>,
}