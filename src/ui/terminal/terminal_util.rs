use crossterm::{cursor, event::{self, poll, read, Event, KeyEvent, KeyEventKind, KeyboardEnhancementFlags}, execute, queue, style::Color, terminal};
use ringbuffer::RingBuffer;
use std::{error::Error, fmt, io::{self, Write}, time::Duration};

use crossterm::style;

use crate::{action::ActionRequest, component::Position, game::{DebugInfo, Game, GameState}, map::Tile, tabletop::Race, ui::menu::{Dropdown, MenuType}, FRAMES_PER_SECOND};

use super::{icons, key_mapping};

pub const MIN_WIDTH: u16 = 80;
pub const MIN_HEIGHT: u16 = 24;

const DEFAULT_BACKGROUND: Color = Color::Black;
const DEFAULT_FOREGROUND: Color= Color::White;

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
    pub background: style::Color,
    pub color: style::Color,
    pub icon: char
}

struct ScreenBuffer {
    width: u16,
    tiles: Vec<DrawInfo>
}

impl ScreenBuffer {
    pub fn new(width: u16, height: u16) -> Self {
        let mut result = Self {
            width,
            tiles: Vec::with_capacity((width * height).into())
        };

        for _ in 0..width*height {
            result.tiles.push(DrawInfo{background: DEFAULT_BACKGROUND, color: DEFAULT_FOREGROUND, icon: ' '});
        }

        return result;
    }

    fn set_by_index(&mut self, index: usize, tile: &DrawInfo) {
        self.tiles[index].color = tile.color;
        self.tiles[index].icon = tile.icon;
    }

    pub fn set_background(&mut self, x: u16, y: u16, background: Color) {
        let index: usize = (y * self.width + x).into();
        self.tiles[index].background = background;
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

fn clear_frame(render_state: &mut RenderState) {
    for y in 0..render_state.screen.height {
        for x in 0..render_state.screen.width {
            render_state.current_frame.set_background(x, y, DEFAULT_BACKGROUND);
            render_state.current_frame.set_color(x, y, DEFAULT_FOREGROUND);
            render_state.current_frame.set_icon(x, y, ' ');
        }
    }
}

fn diff(old: &ScreenBuffer, new: &ScreenBuffer, diff: &mut Vec<bool>) {
    let length: usize = old.tiles.len();

    for i in 0..length {
        diff[i] = old.tiles[i] != new.tiles[i];
    }
}

fn draw_dropdown(render_state: &mut RenderState, dropdown: &Dropdown, x: u16, y: u16) {
    let selection = dropdown.choices.get(dropdown.selected_item).unwrap(); 
    
    let foreground = if !dropdown.editing { DEFAULT_FOREGROUND } else { DEFAULT_BACKGROUND };
    let background = if !dropdown.editing { DEFAULT_BACKGROUND } else { DEFAULT_FOREGROUND };

    draw_dropdown_line(render_state, x, y, foreground, background, selection.as_str(), dropdown.size);

    if dropdown.editing {
        for line in 0..dropdown.selected_item {
            let pos_y: i32 = y as i32 + line as i32 - dropdown.selected_item as i32;
            
            if pos_y >= 0 {
                draw_dropdown_line(render_state, x, pos_y as u16, foreground, Color::Grey, dropdown.choices.get(line).unwrap().as_str(), dropdown.size);
            }
        }
        for line in dropdown.selected_item + 1..dropdown.choices.len() {
            let pos_y: u16 = y + line as u16 - dropdown.selected_item as u16;
            
            if pos_y < render_state.screen.height {
                draw_dropdown_line(render_state, x, pos_y, foreground, Color::Grey, dropdown.choices.get(line).unwrap().as_str(), dropdown.size);
            }
        }
    }
}

fn draw_dropdown_line(render_state: &mut RenderState, x: u16, y: u16, foreground: Color, background: Color, text: &str, width: usize) {
    draw_text_with_background(render_state, text, background, foreground, x, y);

    let current_size = text.len();
    let remainder = width - current_size;
    for position in 0..remainder {
        render_state.current_frame.set_background(position as u16 + current_size as u16 + x, y, background);
        render_state.current_frame.set_color(position as u16 + current_size as u16 + x, y, foreground);
        render_state.current_frame.set_icon(position as u16 + current_size as u16 + x, y, ' ');
    }
}

fn draw_ingame(render_state: &mut RenderState, game: &Game) {
    for y in 0..render_state.screen.height {
        for x in 0..render_state.screen.width {
            let tile: &Tile = game.current_map.as_ref().get(x, y);
            render_state.current_frame.set_color(x, y, icons::tile_color(tile));
            render_state.current_frame.set_icon(x, y, icons::tile_icon(tile));
        }
    }

    let characters = game.components.get_character_components();
    let monsters = game.components.get_monster_components();
    let objects = game.components.get_object_components();

    for i in 0..characters.get_size() {
        if !characters.alive[i].alive || characters.map_index[i].map != game.current_map.id {
            continue;
        }
        let pos: &Position = &characters.position[i];
        let race: &Race = &characters.creature[i].race;

        render_state.current_frame.set_color(pos.x, pos.y, icons::creature_color(race));
        render_state.current_frame.set_icon(pos.x, pos.y,  icons::creature_icon(race));
    }

    for i in 0..monsters.get_size() {
        if !monsters.alive[i].alive || monsters.map_index[i].map != game.current_map.id {
            continue;
        }
        let pos: &Position = &monsters.position[i];
        let race: &Race = &characters.creature[i].race;

        render_state.current_frame.set_color(pos.x, pos.y, icons::creature_color(race));
        render_state.current_frame.set_icon(pos.x, pos.y,  icons::creature_icon(race));
    }
    //TODO(ches) Draw objects
}

fn draw_menu(menu_type: MenuType, render_state: &mut RenderState, game: &Game) {
    clear_frame(render_state);
    match menu_type {
        MenuType::Character => (),
        MenuType::Main => draw_main_menu(render_state, game),
        MenuType::NewCharacter => (),
        MenuType::Pause => (),
        MenuType::TestMenu => draw_test_menu(render_state, game),
    };
}

fn draw_main_menu(render_state: &mut RenderState, _game: &Game) {

    draw_text(render_state, "P", Color::Yellow, 3, 1);
    draw_text(render_state, "Play game", Color::White, 5, 1);
    
    draw_text(render_state, "Q", Color::Yellow, 3, 2);
    draw_text(render_state, "Quit", Color::White, 5, 2);

    draw_text(render_state, "T", Color::Yellow, 3, 3);
    draw_text(render_state, "Test Menu", Color::White, 5, 3);
}

fn draw_fps_counter(render_state: &mut RenderState, game: &Game) {
    let fps: u32 = u32::max(1, get_average_fps(&game.debug_info));

    let text = format!("{}", fps);
    let color: Color = if fps < FRAMES_PER_SECOND as u32 {
        Color::Red
    } else {
        Color::Green
    };

    draw_text(render_state, text.as_str(), color, 0, 0);
}

fn draw_text_with_background(render_state: &mut RenderState, text: &str, background: Color, color: Color, x: u16, y: u16) {
    for (position, character) in text.char_indices() {
        render_state.current_frame.set_background(position as u16 + x, y, background);
        render_state.current_frame.set_color(position as u16 + x, y, color);
        render_state.current_frame.set_icon(position as u16 + x, y, character);
    }
}

fn draw_text(render_state: &mut RenderState, text: &str, color: Color, x: u16, y: u16) {
    draw_text_with_background(render_state, text, Color::Black, color, x, y);
}

fn draw_test_menu(render_state: &mut RenderState, game: &Game) {
    draw_text(render_state, "Test Menu", Color::White, 40, 0);
    draw_text(render_state, "Dropdown: ", Color::White, 0, 2);

    draw_dropdown(render_state, &game.menu_data.test_menu.dropdown, 11, 2);
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

    if !cfg!(windows) {
        panic_on_error!(execute!(io::stdout(), event::PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)));
    }

    panic_on_error!(terminal::enable_raw_mode());
}

pub fn game_drawing_end() {
    panic_on_error!(terminal::disable_raw_mode());
    
    if !cfg!(windows) {
        panic_on_error!(execute!(io::stdout(), event::PopKeyboardEnhancementFlags));
    }

    run_commands!(
        terminal::LeaveAlternateScreen,
        cursor::Show
    );
}

fn get_average_fps(debug_info: &DebugInfo) -> u32 {
    let mut sum: f64 = 0.0;

    for i in 0..debug_info.fps_history.len() {
        sum += *debug_info.fps_history.get(i).unwrap() as f64;
    }

    return (sum / debug_info.fps_history.len() as f64).round() as u32;
}

fn generate_frame(render_state: &mut RenderState, game: &Game) {
    match game.state {
        GameState::Menu(menu_type) => draw_menu(menu_type, render_state, game),
        GameState::Paused => (),
        GameState::Running => draw_ingame(render_state, game),
        GameState::QuitRequested => (),
    };

    draw_fps_counter(render_state, game);
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
            style::SetBackgroundColor(draw_info.background),
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

fn handle_key(event: KeyEvent, game: &mut Game) {
    if event.kind != KeyEventKind::Press {
        return;
    }

    let action: Option<ActionRequest> = key_mapping::map_input(event, game);
 
    if action.is_some() {
        game.action_queue.push_back(action.unwrap());
    }
}

fn _read_input(game: &mut Game) -> Result<(), std::io::Error> {
    const IMMEDIATELY: Duration = Duration::from_secs(0);

    if poll(IMMEDIATELY)? {
        match read()? {
            //Event::FocusGained => println!("FocusGained"),
            //Event::FocusLost => println!("FocusLost"),
            Event::Key(event) => handle_key(event, game),
            //Event::Mouse(event) => println!("{:?}", event),
            //Event::Paste(data) => println!("Pasted {:?}", data),
            //Event::Resize(width, height) => println!("New size {}x{}", width, height),
            _ => {}
        }
    }

    Ok(())
}

pub fn read_input(game: &mut Game) {
    panic_on_error!(_read_input(game));
}

pub fn refresh_back_buffer(render_state: &mut RenderState) {
    for y in 0..render_state.screen.height {
        for x in 0..render_state.screen.width {
            render_state.current_frame.set_background(x, y, DEFAULT_BACKGROUND);
            render_state.last_frame.set_color(x, y, DEFAULT_FOREGROUND);
            render_state.last_frame.set_icon(x, y, ' ');
        }
    }
}
