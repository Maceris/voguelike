use std::{error::Error, fmt, io::{self, Write}};

use crossterm::{ cursor, execute, style, terminal, ExecutableCommand, QueueableCommand };

use crate::map::{Location, Map};

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

pub fn clear_screen() {
    run_commands!(
        terminal::Clear(terminal::ClearType::Purge),
        cursor::MoveTo(0, 0)
    );
}

pub fn create_screen(terminal: Terminal) -> Result<Screen, TerminalError> {
    if terminal.width < MIN_WIDTH {
        return Err(TerminalError{message: format!("Terminal width of {} is below the minimum of {} characters", terminal.width, MIN_WIDTH)});
    }
    if terminal.height < MIN_HEIGHT {
        return Err(TerminalError{message: format!("Terminal height of {} is below the minimum of {} characters", terminal.height, MIN_HEIGHT)});
    }

    let result = Screen {
        width: terminal.width,
        height: terminal.height
    };
    
    Ok(result)
}

pub fn enter_alternate_screen() {
    run_commands!(terminal::EnterAlternateScreen);
}

pub fn exit_alternate_screen() {
    run_commands!(terminal::LeaveAlternateScreen);
}

pub fn print_screen(screen: &Screen, map: &Map) {
    clear_screen();

    let y_max: u16 = screen.height - 1;
    let x_max: u16 = screen.width - 1;

    for y in 0..screen.height {
        for x in 0..screen.width {
            if y == 0 || y == y_max || x == 0 || x == x_max {
                print!("*");
            }
            else {
                print!(" ");
            }
        }
        println!();
    }

    run_commands!(
        cursor::MoveTo(map.player.pos_x, map.player.pos_y),
        style::SetForegroundColor(map.player.get_color()),
        style::Print(map.player.get_icon())
    );

}

