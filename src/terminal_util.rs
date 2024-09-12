use crossterm::{cursor, execute, queue, style::Color, terminal};
use std::{error::Error, fmt, io::{self, Write}};

use crossterm::style;

use crate::{entity::Player, game::Game, map::{Location, Tile}};

pub const MIN_WIDTH: u16 = 77;
pub const MIN_HEIGHT: u16 = 25;

macro_rules! panic_on_error {
    ($expression:expr) => {
        if $expression.is_err() {
            panic!("Terminal error")
        }
    };
}

macro_rules! run_commands {
    ($first_command:expr $(, $command:expr)* $(,)?) => {
        panic_on_error!(execute!(io::stdout(), $first_command $(, $command)*));
    };
}

#[derive(Debug, Clone)]
pub struct TerminalError {
    message: String
}

impl Error for TerminalError {}

impl fmt::Display for TerminalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

// The actual terminal we are using
pub struct Terminal {
    pub width: u16,
    pub height: u16
}

// The virtual screen we are working with
pub struct Screen {
    pub width: u16,
    pub height: u16
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct DrawInfo {
    pub color: style::Color,
    pub icon: char
}

pub trait Drawable {
    fn get_color(&self) -> style::Color;
    fn get_icon(&self) -> char;
}

#[macro_export]
macro_rules! create_drawable {
    ($class:ty, $color:expr, $icon:expr) => {
        impl Drawable for $class {
            fn get_color(&self) -> Color {
                return $color;
            }
            fn get_icon(&self) -> char {
                return $icon;
            }
        }
    };
}

struct ScreenBuffer {
    width: u16,
    height: u16,
    tiles: Vec<DrawInfo>
}

impl ScreenBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        let mut result = Self {
            width,
            height,
            tiles: Vec::with_capacity((width * height).into())
        };

        for _ in 0..width*height {
            result.tiles.push(DrawInfo{color: Color::Black, icon: ' '});
        }

        return result;
    }

    fn set_by_index(&mut self, index: usize, tile: &DrawInfo) {
        self.tiles[index].color = tile.color;
        self.tiles[index].icon = tile.icon;
    }

    pub fn set_color(&mut self, x: u16, y: u16, color: Color) {
        let index: usize = (y * self.width + x).into();
        self.tiles[index].color = color;
    }

    pub fn set_icon(&mut self, x: u16, y: u16, icon: char) {
        let index: usize = (y * self.width + x).into();
        self.tiles[index].icon = icon;
    }
}

pub struct RenderState {
    pub screen: Screen,
    last_frame: ScreenBuffer,
    current_frame: ScreenBuffer,
    frame_diff: Vec<bool>
}

impl RenderState {
    fn new(width: u16, height: u16) -> Self {

        let mut result = Self {
            screen: Screen{width, height},
            last_frame: ScreenBuffer::new(width, height),
            current_frame: ScreenBuffer::new(width, height),
            frame_diff: Vec::with_capacity((width * height).into())
        };

        for _ in 0..result.frame_diff.capacity() {
            result.frame_diff.push(true);
        }

        return result;
    }
}

fn diff(old: &ScreenBuffer, new: &ScreenBuffer, diff: &mut Vec<bool>) {
    let length: usize = old.tiles.len();

    for i in 0..length {
        diff[i] = old.tiles[i] != new.tiles[i];
    }
}

fn generate_frame(render_state: &mut RenderState, game: &Game) {
    if game.current_map.is_some() {
        for y in 0..render_state.screen.height {
            for x in 0..render_state.screen.width {
                let tile: Tile = game.current_map.as_ref().unwrap().get(x, y);
                let draw_info: &DrawInfo = &game.tile_map[tile].draw_info;
                render_state.current_frame.set_color(x, y, draw_info.color);
                render_state.current_frame.set_icon(x, y, draw_info.icon);
            }
        }

        let player: &Player = &game.player;

        render_state.current_frame.set_color(player.get_x(), player.get_y(), player.get_color());
        render_state.current_frame.set_icon(player.get_x(), player.get_y(), player.get_icon());
    }
    //TODO(ches) headers, footers, menus, etc
}

pub fn create_render_state(terminal: Terminal) -> Result<RenderState, TerminalError> {
    if terminal.width < MIN_WIDTH {
        return Err(TerminalError{message: format!("Terminal width of {} is below the minimum of {} characters", terminal.width, MIN_WIDTH)});
    }
    if terminal.height < MIN_HEIGHT {
        return Err(TerminalError{message: format!("Terminal height of {} is below the minimum of {} characters", terminal.height, MIN_HEIGHT)});
    }

    let result = RenderState::new(terminal.width, terminal.height);
    
    Ok(result)
}

pub fn game_drawing_begin() {
    run_commands!(
        terminal::EnterAlternateScreen,
        cursor::Hide
    );
}

pub fn game_drawing_end() {
    run_commands!(
        terminal::LeaveAlternateScreen,
        cursor::Show
    );
}

fn _print_screen(game: &Game, render_state: &mut RenderState) -> Result<(), std::io::Error> {
    generate_frame(render_state, game);

    diff(&render_state.last_frame, &render_state.current_frame, &mut render_state.frame_diff);

    let wrap_point: u16 = render_state.screen.width;

    for i in 0..render_state.current_frame.tiles.len() {
        if !render_state.frame_diff[i] {
            continue;
        }
        let x: u16 = i as u16 % wrap_point;
        let y: u16 = i as u16 / wrap_point;
        
        let draw_info: &DrawInfo = &render_state.current_frame.tiles[i];
        queue!(
            io::stdout(), cursor::MoveTo(x, y),
            style::SetForegroundColor(draw_info.color),
            style::Print(draw_info.icon)
        )?;
        render_state.last_frame.set_by_index(i, draw_info);
    }

    io::stdout().flush()?;
    Ok(())
}

pub fn print_screen(game: &Game, render_state: &mut RenderState) {
    panic_on_error!(_print_screen(game, render_state));
}

