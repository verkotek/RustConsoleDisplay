use crate::color::Color::*;
use crate::model::object::Object;
use crate::position::V2;

pub fn load_model() -> Vec<Object>{
    list()
}

fn list() -> Vec<Object> {
    let mut list = Vec::new();
    list.push(Object::from(
        "\
WALLLL*\
w    l*\
W    l*\
WALLLL".to_string(),
        Red,
        V2(0, 0)
    ));
    list.push( Object::from(
        "\
HHHHHHHHHHHH*\
H           *\
H          H*\
HHH  HHHHHHH*\
H    H     H*\
H    H     H*\
H          H*\
H    H     H*\
HHH  HHHHHHH".to_string(),
        Red,
        V2(0,0)
    ));
    list.push( Object::from(
        "  \
  00 00  *\
000000000* \
 0000000 *   \
   000   ".to_string(),
        Red,
        V2(0,0)
    ));

    list
}
