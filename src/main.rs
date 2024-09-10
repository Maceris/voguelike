use std::{thread, time::{Duration, SystemTime}};

use entity::Player;
use map::{Location, Map};

mod entity;
mod item;
mod map;
mod terminal_util;

const FRAMES_PER_SECOND: u8 = 30;
const NANOS_PER_FRAME: u32 = 1_000_000_000 / (FRAMES_PER_SECOND as u32);

fn main() {

    terminal_util::game_drawing_begin();

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

    let mut map = Map {
        player: Player{
            pos_x: 5,
            pos_y: 5
        },
        width: screen.width,
        height: screen.height
    };

    const FRAME_DURATION: Duration = Duration::from_nanos(NANOS_PER_FRAME as u64);

    let mut dx: i16 = 1;
    let mut dy: i16 = 1;

    let start = SystemTime::now();
    while start.elapsed().unwrap().as_secs() < 10 {
        let frame_start = SystemTime::now();
        
        terminal_util::print_screen(&screen, &map);

        let player: &mut Player = &mut map.player;

        if dx > 0 && player.get_x() as i16 + dx >= (map.width - 1) as i16 {
            dx = -dx;
        }
        else if dx < 0 && player.get_x() as i16 + dx <= 0 {
            dx = -dx;
        }
        if dy > 0 && player.get_y() as i16 + dy >= (map.height - 1) as i16 {
            dy = -dy;
        }
        else if dy < 0 && player.get_y() as i16 + dy <= 0 {
            dy = -dy;
        }

        player.set_x((player.get_x() as i16 + dx) as u16);
        player.set_y((player.get_y() as i16 + dy) as u16);

        let elapsed_time = frame_start.elapsed().unwrap();
        let time_to_sleep = FRAME_DURATION - elapsed_time;

        thread::sleep(time_to_sleep);
    }
    
    terminal_util::game_drawing_end();

}
