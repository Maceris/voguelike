use crossterm::event::{KeyCode, KeyEvent};

use crate::{action::{Action, ActionRequest, Go, Noun, Quit}, entity::EntityID, game::{Game, GameState}, new_action};

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
            actor: game.special_entities.player,
            action: new_action!(Quit),
            noun: Noun::Nothing,
            second: Noun::Nothing
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
            actor: game.special_entities.player,
            action: new_action!(Quit),
            noun: Noun::Nothing,
            second: Noun::Nothing
        };
        return Some(request);
    }

    let direction_key: Option<EntityID> = match event.code {
        KeyCode::Char('7') => Some(game.special_entities.north_west),
        KeyCode::Char('8') => Some(game.special_entities.north),
        KeyCode::Char('9') => Some(game.special_entities.north_east),
        KeyCode::Char('4') => Some(game.special_entities.west),
        KeyCode::Char('6') => Some(game.special_entities.east),
        KeyCode::Char('1') => Some(game.special_entities.south_west),
        KeyCode::Char('2') => Some(game.special_entities.south),
        KeyCode::Char('3') => Some(game.special_entities.south_east),
        _ => None,
    };

    if direction_key.is_some() {
        let request = ActionRequest {
            actor: game.special_entities.player,
            action: new_action!(Go),
            noun: Noun::Entity(direction_key.unwrap()),
            second: Noun::Nothing
        };
        return Some(request);
    }
    
    return None;
}
