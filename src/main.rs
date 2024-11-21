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
use crate::display::camera_pos_center;
use crate::game::Game;

fn main() {

    let mut g = Game::new();
    g.update(
        |s|{
            s.map.update_chunks(&s.camera.pos);
            loop {
                match press() {
                    Ok(KeyCode::Char(c)) => match c {
                        'q' => {return false;}
                        'w' => {s.camera.pos.1 -= 1;}
                        'a' => {s.camera.pos.0 -= 1;}
                        's' => {s.camera.pos.1 += 1;}
                        'd' => {s.camera.pos.0 += 1;}
                        _ => {}
                    }
                    _ => {}
                }

                // let KeyCode::Char(c) = press().unwrap() else { return false; };
                // match c {
                //     'q' => {return false;}
                //     'w' => {s.camera.pos.1 -= 1;}
                //     'a' => {s.camera.pos.0 -= 1;}
                //     's' => {s.camera.pos.1 += 1;}
                //     'd' => {s.camera.pos.0 += 1;}
                //     _ => {}
                // }
                break;
            }


            s.display.show_map(&s.map, &s.camera);
            print!("\x1B[0;0H{:?}",chunk::pos_on_chunk(&camera_pos_center(&s.camera.pos)));
            io::stdout().flush().unwrap();


            return true;
        }
    );

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