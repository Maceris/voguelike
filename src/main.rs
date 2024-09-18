use std::{collections::VecDeque, thread, time::{Duration, Instant}};

use action::ActionRequest;
use component::Components;
use entity::Entity;
use game::{DataTables, DebugInfo, Game, GameState};
use gen::map_gen;
use map::{Location, GameMap};
use ringbuffer::{AllocRingBuffer, RingBuffer};
use ui::terminal::terminal_util;

mod action;
mod component;
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
        action_queue: VecDeque::with_capacity(1000),
        components: Components::new(),
        current_map: Box::new(GameMap::empty_map()),
        data_tables: DataTables {
            tile_map: map::generate_tile_map(),
            tag_map: tag::generate_tag_map(),
            material_map: material::generate_material_map(),
        },
        debug_info: DebugInfo{fps_history: AllocRingBuffer::new(100)},
        player: Entity{
            id: entity::ID_PLAYER,
            race: tabletop::Race::Human,
            pos_x: 5,
            pos_y: 5
        },
        state: GameState::Menu,
    };

    game.current_map = Box::new(GameMap::new(render_state.screen.width, render_state.screen.height));

    map_gen::populate_map(game.current_map.as_mut());

    const FRAME_DURATION: Duration = Duration::from_nanos(NANOS_PER_FRAME as u64);

    let y_max: u16 = game.current_map.as_ref().height - 1;
    let x_max: u16 = game.current_map.as_ref().width - 1;
    let mut dx: i16 = 1;
    let mut dy: i16 = 1;

    game.state = GameState::Running;

    let mut frame_start = Instant::now();
    let mut first_frame: bool = true;
    while game.state != GameState::QuitRequested {
        if first_frame {
            first_frame = false;
        } else {
            let time_since_last_frame_start = frame_start.elapsed();
            let fps: f64 = 1_000_000_000.0 / f64::max(1.0, time_since_last_frame_start.as_nanos() as f64);
            game.debug_info.fps_history.push(fps.round() as u32);
        }

        frame_start = Instant::now();
        
        terminal_util::read_input(&mut game);

        if !game.action_queue.is_empty() {
            let action:ActionRequest = game.action_queue.pop_front().unwrap();
            
            action::execute_action(&mut game, action.action, None::<Entity>, None::<Entity>);
        }

        terminal_util::print_screen(&game, &mut render_state);

        let player: &mut Entity = &mut game.player;

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

        let elapsed_time = frame_start.elapsed();

        if elapsed_time < FRAME_DURATION {
            let time_to_sleep = FRAME_DURATION - elapsed_time;
            thread::sleep(time_to_sleep.mul_f64(0.95));
        }
        
    }

    terminal_util::game_drawing_end();

}
