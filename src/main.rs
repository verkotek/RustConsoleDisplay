mod display;
mod rng;
mod color;
mod map;
mod model;
mod chunk;
mod position;

use std::io::{self, Write};
use std::usize;
use crossterm::event::*;
use crossterm::*;
use crate::color::Color::*;
use crate::display::{Display};
use crate::map::{Map};
use crate::model::list::load_model;
use crate::model::object::Object;

fn main() {
    let list_model = load_model();

    let size = terminal::size().unwrap();
    let _size_line = (size.0 * size.1) as usize;
    //

    let mut cam = Object::new();
    let mut dis = Display::new();
    let mut map = Map::new();

    // map.installation_object(Object::create(
    //             &list_model[2],
    //             Red,
    //             V2(31,31))
    // ).installation_object(Object::create(
    //     &list_model[2],
    //     Green,
    //     V2(40,10))
    // );

    let varied = [Red, Yellow, Green, Blue];
    for _ in 0..10000 {
        map.installation_object(Object::from(
            "@".to_string(),
            varied[rng::gen(1usize, varied.len() as f64)].clone(),
            rng::gen_v2(200.0,200.0))
        );
    }
    for _ in 0..50 {
        map.installation_object(Object::create(
            &list_model[0],
            varied[rng::gen(1usize, varied.len() as f64)].clone(),
            rng::gen_v2(500.0,500.0))
        );
        map.installation_object(Object::create(
            &list_model[1],
            varied[rng::gen(1usize, varied.len() as f64)].clone(),
            rng::gen_v2(500.0,500.0))
        );
    }
    for _ in 0..10 {
        map.installation_object(Object::create(
            &list_model[2],
            varied[rng::gen(1usize, varied.len() as f64)].clone(),
            rng::gen_v2(500.0,500.0))
        );
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