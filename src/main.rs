use std::time::{Duration, SystemTime};

use crossterm::execute;
use entity::Player;
use map::Map;

mod entity;
mod item;
mod map;
mod terminal_util;

fn main() {

    terminal_util::enter_alternate_screen();

    let size = match crossterm::terminal::size() {
        Ok(value) => value,
        Err(_) => panic!("Cannot obtain terminal size")
    };

    let sample_term = terminal_util::Terminal {
        width: size.0,
        height: size.1
    };
   
    let screen = match terminal_util::create_screen(sample_term) {
        Ok(value) => value,
        Err(e) => panic!("{}", e)
    };

    let map = Map {
        player: Player{
            pos_x: 5,
            pos_y: 5
        }
    };

    let start = SystemTime::now();

    while start.elapsed().unwrap().as_secs() < 5 {
        terminal_util::print_screen(&screen, &map);
    }
    
    terminal_util::exit_alternate_screen();

}
