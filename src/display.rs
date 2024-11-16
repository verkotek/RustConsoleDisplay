use std::io::{self, Write};
use std::string::ToString;
use crossterm::*;
use crate::color::*;
use crate::color::Color::*;
use crate::display::FillMode::*;
use crate::map::{Map};
use crate::object::Object;

pub struct Display{
    size_xy:(u16,u16),
    size: usize,

    screen: Vec<String>,
}

impl Display{
    pub fn new() -> Self{
        let size_xy = terminal::size().unwrap();
        let mut screen :Vec<String> = Vec::new();
        screen.push(String::new());
        let mut d = Display{
            size_xy,
            size: (size_xy.0 * size_xy.1) as usize,
            screen,
        };
        d.fill(&' ');
        d
    }

    pub fn fill(&mut self, char_: &char){
        let text = String::from(char_.clone()).repeat(self.size);
        self.screen = text.chars().map(|c| c.to_string()).collect();
    }

    pub fn draw(&mut self, fill_mode: FillMode, char_: char, color: &Color) {
        let color_ = get_color(color);
        match fill_mode {
            Line(x1,y1,x2,y2) => {
                let mut x = x1 as i32;
                let mut y = y1 as i32;
                let dx= (x2 as i32 - x1 as i32).abs();
                let dy= (y2 as i32 - y1 as i32).abs();
                let sx: i32 = if x1 < x2 { 1 } else { -1 };
                let sy: i32 = if y1 < y2 { 1 } else { -1 };
                let mut err = dx - dy;

                loop {
                    self.draw(Point(x as u16, y as u16), char_, color);

                    if x == x2.into() && y == y2.into() { break; }

                    let e2 = err * 2;
                    if e2 > -dy {
                        err -= dy;
                        x += sx;
                    }
                    if e2 < dx {
                        err += dx;
                        y += sy;
                    }
                }
            },
            Point(x1,y1) => {
                let p = {self.size_xy.0 * y1 + x1} as usize;
                self.screen[p] = format!("{}{}{}", color_, char_, get_color(&Reset));
            }
        }
        io::stdout().flush().unwrap();
    }

    pub fn show(&self){
        let text: String = self.screen.join("");
        set_mouse_pos(0,0);
        print!("{}",text);
    }

    pub fn show_map(&mut self, map: &Map, camera: &Object){
        self.fill(&' ');
        for b in map.0.iter() {
            let pos = &b.pos - &camera.pos;
            if pos.0 < 0 || pos.1 < 0 || pos.0 > self.size_xy.0 as i32 - 1 || pos.1 > self.size_xy.1 as i32 -1  { continue; }
            self.draw(Point(pos.0 as u16, pos.1 as u16), b.skin, &b.color);
        }
        self.show();
    }
}
fn set_mouse_pos(x:u16, y:u16){
    print!("\x1B[{y};{x}H");
    io::stdout().flush().unwrap();
}

#[derive(Debug)]
pub enum FillMode{
    Line(u16,u16,u16,u16),
    Point(u16,u16),
}



