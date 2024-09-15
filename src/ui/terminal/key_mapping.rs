use crossterm::event::{KeyCode, KeyEvent};

use crate::{action::{Action, Quit}, game::GameState};

pub fn map_input(event: KeyEvent, game_state: GameState) -> Option<Action> {
    if event.code == KeyCode::Esc {
        return Some(Action::Quit(Quit{}));
    }
    return None;
}
