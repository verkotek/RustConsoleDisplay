#[derive(Clone)]
pub enum Color{
    Reset,
    Red,
    Green,
    Yellow,
    Blue,
}

pub fn get_color(color: &Color) -> String{
    match color {
        Color::Reset => {String::from(RESET)}
        Color::Red => {String::from(RED)}
        Color::Green => {String::from(GREEN)}
        Color::Yellow => {String::from(YELLOW)}
        Color::Blue => {String::from(BLUE)}
    }
}
const RESET: &str = "\u{001b}[0m";
const RED: &str = "\u{001b}[31m";
const GREEN: &str = "\u{001b}[32m";
const YELLOW: &str = "\u{001b}[33m";
const BLUE: &str = "\u{001b}[34m";