use std::{thread, time::{Duration, SystemTime}};

use entity::Player;
use game::{Game, GameState};
use map::{Location, Map};

mod action;
mod entity;
mod item;
mod game;
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

    let mut game = Game {
        state: GameState::Menu,
        player: Player{
            pos_x: 5,
            pos_y: 5
        },
        tile_map: map::generate_tile_map()
    };

    let mut map = Map::new(screen.width, screen.height);

    let y_max: u16 = screen.height - 1;
    let x_max: u16 = screen.width - 1;
    for y in 0..screen.height {
        for x in 0..screen.width {
            if y == 0 || y == y_max || x == 0 || x == x_max {
                map.set(x, y, map::Tile::Wall);
            }
            else {
                map.set(x, y, map::Tile::Floor);
            }
        }
    }

    const FRAME_DURATION: Duration = Duration::from_nanos(NANOS_PER_FRAME as u64);

    let mut dx: i16 = 1;
    let mut dy: i16 = 1;

    let start = SystemTime::now();
    while start.elapsed().unwrap().as_secs() < 10 {
        let frame_start = SystemTime::now();
        
        terminal_util::print_screen(&game, &screen, &map);

        let player: &mut Player = &mut game.player;

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
