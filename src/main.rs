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
    let mut _mouse_pos = 1;
    let size = terminal::size().unwrap();
    let _size_line = (size.0 * size.1) as usize;
    //

    let mut cam = Object{ skin: ' ', color: Reset, pos: V2(1, 0) };
    let mut dis = Display::new();
    let mut map = Map::new();



    let varied = [Red, Yellow, Green, Blue];
    for _i in 0..100 {
        map.0.push(Object{
            skin: '@',
            color: varied[rng::gen(varied.len() as f64) as usize].clone(),
            pos: V2(rng::gen(300.0) as i32-50, rng::gen(100.0) as i32-50)
        });
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


        // dis.fill(&c);
        // dis.draw(FillMode::Point(rng::gen(size.0 as f64) as u16, rng::gen(size.1 as f64) as u16), '@', &Red);
        // dis.draw(FillMode::Point(rng::gen(size.0 as f64) as u16, rng::gen(size.1 as f64) as u16), '@', &Yellow);
        // dis.draw(FillMode::Point(rng::gen(size.0 as f64) as u16, rng::gen(size.1 as f64) as u16), '@', &Green);
        // dis.draw(FillMode::Line(0, 0, rng::gen(10.0) as u16, rng::gen(10.0) as u16), '*', &Yellow);

        // dis.show();


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