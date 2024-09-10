pub enum GameState {
    Menu,
    Paused,
    Running
}

pub struct Game {
    pub state: GameState
}