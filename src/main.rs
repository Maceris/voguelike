use std::{thread, time::{Duration, SystemTime}};

use entity::Player;
use game::{DataTables, DebugInfo, Game, GameState};
use gen::map_gen;
use map::{Location, GameMap};
use ringbuffer::{AllocRingBuffer, RingBuffer};
use ui::terminal::terminal_util;

mod action;
mod entity;
mod item;
mod game;
mod gen;
mod map;
mod material;
mod tabletop;
mod tag;
mod ui;

const FRAMES_PER_SECOND: u8 = 30;
const NANOS_PER_FRAME: u32 = 1_000_000_000 / (FRAMES_PER_SECOND as u32);

fn main() {

    terminal_util::game_drawing_begin();

    let size = match crossterm::terminal::size() {
        Ok(value) => value,
        Err(_) => panic!("Cannot obtain terminal size")
    };

    let terminal = terminal_util::Terminal {
        width: size.0,
        height: size.1
    };
   
    let mut render_state = match terminal_util::create_render_state(terminal) {
        Ok(value) => value,
        Err(e) => panic!("{}", e)
    };

    let mut game = Game {
        state: GameState::Menu,
        player: Player{
            pos_x: 5,
            pos_y: 5
        },
        current_map: None,
        data_tables: DataTables {
            tile_map: map::generate_tile_map(),
            tag_map: tag::generate_tag_map(),
            material_map: material::generate_material_map(),
        },
        debug_info: DebugInfo{fps_history: AllocRingBuffer::new(100)}
    };

    game.current_map = Some(Box::new(GameMap::new(render_state.screen.width, render_state.screen.height)));

    map_gen::populate_map(game.current_map.as_mut().unwrap());

    const FRAME_DURATION: Duration = Duration::from_nanos(NANOS_PER_FRAME as u64);

    let y_max: u16 = game.current_map.as_ref().unwrap().height - 1;
    let x_max: u16 = game.current_map.as_ref().unwrap().width - 1;
    let mut dx: i16 = 1;
    let mut dy: i16 = 1;

    let start = SystemTime::now();
    while start.elapsed().unwrap().as_secs() < 10 {
        let frame_start = SystemTime::now();
        
        terminal_util::print_screen(&game, &mut render_state);

        let player: &mut Player = &mut game.player;

        if dx > 0 && player.get_x() as i16 + dx >= x_max as i16 {
            dx = -dx;
        }
        else if dx < 0 && player.get_x() as i16 + dx <= 0 {
            dx = -dx;
        }
        if dy > 0 && player.get_y() as i16 + dy >= y_max as i16 {
            dy = -dy;
        }
        else if dy < 0 && player.get_y() as i16 + dy <= 0 {
            dy = -dy;
        }

        player.set_x((player.get_x() as i16 + dx) as u16);
        player.set_y((player.get_y() as i16 + dy) as u16);

        let elapsed_time = frame_start.elapsed().unwrap();

        let fps: u128 = 1_000_000_000 / u128::max(1, elapsed_time.as_nanos());
        game.debug_info.fps_history.push(fps as u32);

        if elapsed_time < FRAME_DURATION {
            let time_to_sleep = FRAME_DURATION - elapsed_time;
            thread::sleep(time_to_sleep);
        }
        
    }
    
    terminal_util::game_drawing_end();

}
