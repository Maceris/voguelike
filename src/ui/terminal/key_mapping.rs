use crossterm::event::{KeyCode, KeyEvent};

use crate::{action::{Action, ActionRequest, ExitMenu, Quit}, entity, game::{Game, GameState}, new_action};

pub fn map_input(event: KeyEvent, game: &Game) -> Option<ActionRequest> {
    return match game.state {
        GameState::Menu=> map_input_menu(event, game),
        GameState::Paused=> map_input_paused(event, game),
        GameState::Running=> map_input_ingame(event, game),
        GameState::QuitRequested => None
    };
}

fn map_input_menu(event: KeyEvent, game: &Game) -> Option<ActionRequest> {
    if event.code == KeyCode::Esc {
        let request = ActionRequest {
            actor: game.player.id,
            action: new_action!(Quit),
            noun: entity::ID_NO_ENTITY,
            second: entity::ID_NO_ENTITY
        };
        return Some(request);
    }
    return None;
}

fn map_input_paused(event: KeyEvent, game: &Game) -> Option<ActionRequest> {
    return None;
}

fn map_input_ingame(event: KeyEvent, game: &Game) -> Option<ActionRequest> {
    if event.code == KeyCode::Esc {
        let request = ActionRequest {
            actor: game.player.id,
            action: new_action!(Quit),
            noun: entity::ID_NO_ENTITY,
            second: entity::ID_NO_ENTITY
        };
        return Some(request);
    }
    if event.code == KeyCode::Char('7') {
        let request = ActionRequest {
            actor: game.player.id,
            action: new_action!(Quit),
            noun: entity::ID_NO_ENTITY,
            second: entity::ID_NO_ENTITY
        };
        return Some(request);
    }
    
    return None;
}
