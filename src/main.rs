mod display;
mod rng;
mod color;
mod map;
mod object;

use std::io::{self, Write};
use crossterm::event::*;
use crossterm::*;
use crate::color::Color::*;
use crate::display::{Display};
use crate::map::{Map, V2};
use crate::object::Object;

fn main() {
    let home = Object::from(
        "\
WALLLL*\
w    l*\
W    l*\
WALLLL".to_string(),
        Red,
        V2(rng::gen(300.0) as i32-50, rng::gen(100.0) as i32-50)
    );
    let big_home = Object::from(
        "\
HHHHHHHHHHHH*\
H           *\
H          H*\
HHH  HHHHHHH*\
H    H*\
H    H*\
H    H*\
H    H*\
HHHHHH".to_string(),
        Red,
        V2(rng::gen(300.0) as i32-50, rng::gen(100.0) as i32-50)
    );


    let mut _mouse_pos = 1;
    let size = terminal::size().unwrap();
    let _size_line = (size.0 * size.1) as usize;
    //

    let mut cam = Object::new();
    let mut dis = Display::new();
    let mut map = Map::new();



    let varied = [Red, Yellow, Green, Blue];
    for _i in 0..1000 {
        map.0.push(Object::from(
            "@".to_string(),
            varied[rng::gen(varied.len() as f64) as usize].clone(),
            V2(rng::gen(500.0) as i32-250, rng::gen(500.0) as i32-250))
        );
    }
    for _i in 0..50 {
        map.0.push(Object::from(home.skin.chars.iter().collect(), varied[rng::gen(varied.len() as f64) as usize].clone(), V2(rng::gen(500.0) as i32-250, rng::gen(500.0) as i32-250)));
        map.0.push(
            Object::from(big_home.skin.chars.iter().collect(),
                         varied[rng::gen(varied.len() as f64) as usize].clone(),
                         V2(rng::gen(50.0) as i32, rng::gen(50.0) as i32)));
    }





    loop {
        let KeyCode::Char(c) = press().unwrap() else { return; };
        match c {
            'q' => {break;}
            'w' => {cam.pos.1 -= 1;}
            'a' => {cam.pos.0 -= 1;}
            's' => {cam.pos.1 += 1;}
            'd' => {cam.pos.0 += 1;}
            _ => {}
        }
        // if c == '`' { _mouse_pos = (rand::random::<f64>() * (size.1-1) as f64) as usize }
        // if c == 'q' { break; }

        dis.show_map(&map, &cam);


        io::stdout().flush().unwrap();
    }


}

pub fn press() -> io::Result<KeyCode> {
    terminal::enable_raw_mode()?;
    loop {
        if let Event::Key(KeyEvent {code, kind, ..}) = read()? {
            if KeyEventKind::Press != kind { continue }
            return match code {
                key => {
                    terminal::disable_raw_mode()?;
                    Ok(key)
                }
            }
        }
    }
}