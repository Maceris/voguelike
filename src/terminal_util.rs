use std::{error::Error, fmt, io};

use crossterm::{
    ExecutableCommand,
    terminal, cursor, style::{self, Stylize}
};

pub const MIN_WIDTH: u16 = 77;
pub const MIN_HEIGHT: u16 = 25;

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

pub fn clear_screen() {
    let mut stdout = io::stdout();
    
    let result = stdout.execute(terminal::Clear(terminal::ClearType::All));
    
    if result.is_err() {
        panic!("Unable to clear terminal")
    }
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

pub fn print_screen(screen: Screen) {
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
}
