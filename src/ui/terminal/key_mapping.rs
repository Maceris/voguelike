use crossterm::event::{KeyCode, KeyEvent};

use crate::{action::{Action, ActionRequest, CloseMenu, Go, NavigateMenu, NewGame, Noun, OpenMenu, Quit}, entity::EntityID, game::{Game, GameState}, new_action, ui::menu::{MenuItem, MenuType}};

pub fn map_input(event: KeyEvent, game: &mut Game) -> Option<ActionRequest> {
    return match game.state {
        GameState::Menu(menu)=> map_input_menu(menu, event, game),
        GameState::Paused=> map_input_paused(event, game),
        GameState::Running=> map_input_ingame(event, game),
        GameState::QuitRequested => None
    };
}

fn map_input_menu(menu: MenuType, event: KeyEvent, game: &mut Game) -> Option<ActionRequest> {
    
    let direction = match event.code {
        KeyCode::Up => Some(game.special_entities.north),
        KeyCode::Right => Some(game.special_entities.east),
        KeyCode::Down => Some(game.special_entities.south),
        KeyCode::Left => Some(game.special_entities.west),
        KeyCode::Esc => Some(game.special_entities.up),
        KeyCode::Enter => Some(game.special_entities.down),
        _=> None,
    };

    if direction.is_some() {
        let request = ActionRequest {
            actor: game.special_entities.player,
            action: new_action!(NavigateMenu),
            noun: Noun::Entity(direction.unwrap()),
            second: Noun::Nothing
        };
        return Some(request);
    }

    return match menu {
        MenuType::Character => None,
        MenuType::Main => map_input_main_menu(event, game),
        MenuType::NewCharacter => map_input_new_character(event, game),
        MenuType::Pause => None,
        MenuType::TestMenu => map_input_test_menu(event, game),
    };
}

fn map_input_main_menu(event: KeyEvent, game: &Game) -> Option<ActionRequest> {
    if event.code == KeyCode::Char('p') || event.code == KeyCode::Char('P') {
        let request = ActionRequest {
            actor: game.special_entities.player,
            action: new_action!(NewGame),
            noun: Noun::Nothing,
            second: Noun::Nothing
        };
        return Some(request);
    }
    if event.code == KeyCode::Char('q') || event.code == KeyCode::Char('Q') {
        let request = ActionRequest {
            actor: game.special_entities.player,
            action: new_action!(Quit),
            noun: Noun::Nothing,
            second: Noun::Nothing
        };
        return Some(request);
    }
    if event.code == KeyCode::Char('t') || event.code == KeyCode::Char('T') {
        let request = ActionRequest {
            actor: game.special_entities.player,
            action: new_action!(OpenMenu),
            noun: Noun::Menu(MenuType::TestMenu),
            second: Noun::Nothing
        };
        return Some(request);
    }
    if event.code == KeyCode::Char('n') || event.code == KeyCode::Char('N') {
        let request = ActionRequest {
            actor: game.special_entities.player,
            action: new_action!(OpenMenu),
            noun: Noun::Menu(MenuType::NewCharacter),
            second: Noun::Nothing
        };
        return Some(request);
    }
    
    return None;
}

fn map_input_new_character(event: KeyEvent, game: &Game) -> Option<ActionRequest> {
    if event.code == KeyCode::Esc {
        let request = ActionRequest {
            actor: game.special_entities.player,
            action: new_action!(CloseMenu),
            noun: Noun::Nothing,
            second: Noun::Nothing
        };
        return Some(request);
    }

    return None;
}

fn map_input_paused(_event: KeyEvent, _game: &Game) -> Option<ActionRequest> {
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

fn map_input_test_menu(event: KeyEvent, game: &mut Game) -> Option<ActionRequest> {
    if event.code == KeyCode::Esc && !game.menu_data.test_menu.editing_anything() {
        let request = ActionRequest {
            actor: game.special_entities.player,
            action: new_action!(CloseMenu),
            noun: Noun::Nothing,
            second: Noun::Nothing
        };
        return Some(request);
    }

    let character: Option<char> = match event.code {
        KeyCode::Char(character) => Some(character),
        _ => None,
    };

    if character.is_some() {
        let element: &mut MenuItem = game.menu_data.test_menu.get_currently_selected_element_mut();

        match element {
            MenuItem::TextField(text_field) => {
                if text_field.editing {
                    if text_field.value.len() < text_field.max_length as usize {
                        text_field.value.push(character.unwrap());
                        return None;
                    }
                }
            },
            _ => {},
        };
    }

    if event.code == KeyCode::Backspace {
        let element: &mut MenuItem = game.menu_data.test_menu.get_currently_selected_element_mut();

        match element {
            MenuItem::TextField(text_field) => {
                if text_field.editing {
                    if text_field.value.len() > 0 {
                        text_field.value.truncate(text_field.value.len() - 1);
                        return None;
                    }
                }
            },
            _ => {},
        };
    }

    return None;
}

