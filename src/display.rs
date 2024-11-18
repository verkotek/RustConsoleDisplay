use std::io::{self, Write};
use std::string::ToString;
use crossterm::*;
use crate::color::*;
use crate::color::Color::*;
use crate::display::FillMode::*;
use crate::map::Map;
use crate::position::V2;
use crate::model::object::Object;

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

    pub fn draw(&mut self, fill_mode: FillMode, pos_camera: &V2) {

        match fill_mode {
            Line(x1,y1,x2,y2, c, col) => {
                let mut x = x1 as i32;
                let mut y = y1 as i32;
                let dx= (x2 - x1).abs();
                let dy= (y2 - y1).abs();
                let sx: i32 = if x1 < x2 { 1 } else { -1 };
                let sy: i32 = if y1 < y2 { 1 } else { -1 };
                let mut err = dx - dy;

                loop {
                    self.draw(Point(x, y, c, col.clone()), pos_camera);

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
            Point(x1,y1, c, col) => {
                if min_max_v2(0, self.size_xy , pos_camera ) { return; }
                let color_ = get_color(&col);
                let p = {self.size_xy.0 as i32 * y1 + x1} as usize;

                self.screen[p] = format!("{}{}{}", color_, c, get_color(&Reset));
            }
            Model(m) => {
                let mut i = (-1,-1);
                for char in m.skin.chars.iter() {
                    i.0+=1;
                    if char == &'*' { i.1+=1; i.0 = -1; continue; }
                    let pos = pos_camera.clone() + V2(i.0, i.1);
                    if min_max_v2(0, self.size_xy, &pos) { continue; }
                    let p = {self.size_xy.0 * (pos_camera.1 + i.1) as u16 + (pos_camera.0  + i.0) as u16} as usize;
                    self.screen[p] = format!("{}{}{}", get_color(&m.skin.color[i.0 as usize]), char, get_color(&Reset));

                }
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
            self.draw(Model(b.clone()), &pos);
        }
        self.show();
    }
}
fn set_mouse_pos(x:u16, y:u16){
    print!("\x1B[{y};{x}H");
    io::stdout().flush().unwrap();
}

fn min_max_v2(min: i32, max: (u16, u16), value: &V2) -> bool{
    if value.0 < min || value.0 > max.0 as i32 - 1  || value.1 < min || value.1 > max.1 as i32 - 1 { return true }
    false
}

#[derive(Debug)]
pub enum FillMode{
    Line(i32,i32,i32,i32, char, Color),
    Point(i32,i32, char, Color),
    Model(Object)
}



