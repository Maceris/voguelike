use std::{thread, time::{Duration, Instant}};

use action::ActionRequest;
use game::{Game, GameState};
use gen::map_gen;
use map::GameMap;
use ringbuffer::RingBuffer;
use tabletop::{Alignment, Class, Race, Size};
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

    let mut game = Game::new();

    initialize_player(&mut game);

    game.current_map = Box::new(GameMap::new(0, render_state.screen.width, render_state.screen.height));

    map_gen::populate_map(game.current_map.as_mut());

    const FRAME_DURATION: Duration = Duration::from_nanos(NANOS_PER_FRAME as u64);

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
            
            action::execute_action(&mut game, action);
        }

        terminal_util::print_screen(&game, &mut render_state);

        let elapsed_time = frame_start.elapsed();

        if elapsed_time < FRAME_DURATION {
            let time_to_sleep = FRAME_DURATION - elapsed_time;
            thread::sleep(time_to_sleep.mul_f64(0.95));
        }
        
    }

    terminal_util::game_drawing_end();

}

fn initialize_player(game: &mut Game) {
    let player_creature = game.components.get_creature_mut(game.special_entities.player).unwrap();

    player_creature.alignment = Alignment::NeutralGood;
    player_creature.race = Race::Human;
    player_creature.size = Size::Medium;
    player_creature.stats.charisma = 10;
    player_creature.stats.constitution = 10;
    player_creature.stats.dexterity = 10;
    player_creature.stats.intelligence = 10;
    player_creature.stats.strength = 10;
    player_creature.stats.wisdom = 10;

    let character = game.components.get_character_mut(game.special_entities.player).unwrap();
    character.class = Class::Fighter;

    let map_location = game.components.get_map_index_mut(game.special_entities.player).unwrap();
    map_location.map = 0;

    let position = game.components.get_position_mut(game.special_entities.player).unwrap();
    position.x = 5;
    position.y = 5;
}