mod display;
mod color;
mod map;
mod model;
mod chunk;
mod position;
mod game;
mod settings;

use std::io::{self, Write};
use crossterm::event::*;
use crossterm::*;
use crate::color::Color::*;
use crate::display::camera_pos_center;
use crate::game::Game;
use crate::settings::SIZE;

fn main() {

    let mut g = Game::new();
    g.update(
        |s|{
            s.map.update_chunks(&s.camera.pos);
            loop {
                let KeyCode::Char(c) = press().unwrap() else { return false; };
                match c {
                    'q' => {return false;}
                    'w' => {s.camera.pos.1 -= 1;}
                    'a' => {s.camera.pos.0 -= 1;}
                    's' => {s.camera.pos.1 += 1;}
                    'd' => {s.camera.pos.0 += 1;}
                    _ => {}
                }
                break;
            }


            s.display.show_map(&s.map, &s.camera);
            print!("\x1B[0;0H{:?}",chunk::pos_on_chunk(&camera_pos_center(&s.camera.pos))*SIZE);
            io::stdout().flush().unwrap();


            return true;
        }
    );

    // let size = terminal::size().unwrap();
    // let _size_line = (size.0 * size.1) as usize;
    //

    // let mut cam = Object::new();
    // let mut dis = Display::new();
    // let mut map = Map::new();


    // let mut rng = rand::thread_rng();
    //
    //
    // let varied = [Red, Yellow, Green, Blue];
    // for _ in 0..10000 {
    //     g.map.installation_object(Object::from(
    //         "@".to_string(),
    //         varied[rng.gen_range(0..varied.len())].clone(),
    //         V2(rng.gen_range(0..200),rng.gen_range(0..200)),
    //     ));
    // }




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