use traits::create_action;

use crate::{component::Position, entity::EntityID, game::{Game, GameState}, ui::menu::{MenuNavigation, MenuType}};

create_action!(CloseMenu);
create_action!(NavigateMenu);
create_action!(NewGame);
create_action!(OpenMenu);
create_action!(Quit);
create_action!(Restart);
create_action!(Restore);
create_action!(Save);

create_action!(Answer);
create_action!(Ask);
create_action!(AskFor);
create_action!(Attack);
create_action!(Blow);
create_action!(Burn);
create_action!(Buy);
create_action!(Clean);
create_action!(Climb);
create_action!(Close);
create_action!(Consult);
create_action!(Crush);
create_action!(Cut);
create_action!(Dig);
create_action!(Disrobe);
create_action!(Drink);
create_action!(Drop);
create_action!(Eat);
create_action!(Empty);
create_action!(Enter);
create_action!(Examine);
create_action!(Exit);
create_action!(Fill);
create_action!(GetOff);
create_action!(Give);
create_action!(Go);
create_action!(Insert);
create_action!(Inventory);
create_action!(Jump);
create_action!(JumpOver);
create_action!(Kiss);
create_action!(Listen);
create_action!(LetGo);
create_action!(Lock);
create_action!(Look);
create_action!(LookUnder);
create_action!(NotUnderstood);
create_action!(Open);
create_action!(Order);
create_action!(Pray);
create_action!(Pull);
create_action!(Push);
create_action!(PushDir);
create_action!(PutOn);
create_action!(Receive);
create_action!(Remove);
create_action!(Search);
create_action!(Set);
create_action!(SetTo);
create_action!(Show);
create_action!(Sing);
create_action!(Sleep);
create_action!(Smell);
create_action!(Swim);
create_action!(Swing);
create_action!(SwitchOff);
create_action!(SwitchOn);
create_action!(Take);
create_action!(Taste);
create_action!(Tell);
create_action!(Think);
create_action!(ThrowAt);
create_action!(ThrownAt);
create_action!(Tie);
create_action!(Touch);
create_action!(Turn);
create_action!(Unlock);
create_action!(Wait);
create_action!(Wake);
create_action!(WakeOther);
create_action!(Wave);
create_action!(WaveHands);
create_action!(Wear);

#[derive(Clone, Copy, Debug)]
pub enum Action {
    // Meta actions
    CloseMenu(CloseMenu),
    NavigateMenu(NavigateMenu),
    NewGame(NewGame),
    OpenMenu(OpenMenu),
    Quit(Quit),
    Restart(Restart),
    Restore(Restore),
    Save(Save),

    // Regular actions
    Answer(Answer),
    Ask(Ask),
    AskFor(AskFor),
    Attack(Attack),
    Blow(Blow),
    Burn(Burn),
    Buy(Buy),
    Clean(Clean),
    Climb(Climb),
    Close(Close),
    Consult(Consult),
    Crush(Crush),
    Cut(Cut),
    Dig(Dig),
    Disrobe(Disrobe),
    Drink(Drink),
    Drop(Drop),
    Eat(Eat),
    Empty(Empty),
    Enter(Enter),
    Examine(Examine),
    Exit(Exit),
    Fill(Fill),
    GetOff(GetOff),
    Give(Give),
    Go(Go),
    Insert(Insert),
    Inventory(Inventory),
    Jump(Jump),
    JumpOver(JumpOver),
    Kiss(Kiss),
    LetGo(LetGo),
    Listen(Listen),
    Lock(Lock),
    Look(Look),
    LookUnder(LookUnder),
    NotUnderstood(NotUnderstood),
    Open(Open),
    Order(Order),
    Pray(Pray),
    Pull(Pull),
    Push(Push),
    PushDir(PushDir),
    PutOn(PutOn),
    Receive(Receive),
    Remove(Remove),
    Search(Search),
    Set(Set),
    SetTo(SetTo),
    Show(Show),
    Sing(Sing),
    Sleep(Sleep),
    Smell(Smell),
    Swim(Swim),
    Swing(Swing),
    SwitchOff(SwitchOff),
    SwitchOn(SwitchOn),
    Take(Take),
    Taste(Taste),
    Tell(Tell),
    Think(Think),
    ThrowAt(ThrowAt),
    ThrownAt(ThrownAt),
    Tie(Tie),
    Touch(Touch),
    Turn(Turn),
    Unlock(Unlock),
    Wait(Wait),
    Wake(Wake),
    WakeOther(WakeOther),
    Wave(Wave),
    WaveHands(WaveHands),
    Wear(Wear),
}

#[macro_export]
macro_rules! new_action {
    ($class:ident) => {
        Action::$class($class{})
    };
}

pub trait ActionRoutine {
    fn execute(game: &mut Game, actor: EntityID, noun: Noun, second: Noun) -> bool;
}

macro_rules! stub_action {
    ($class:ident) => {
        impl ActionRoutine for $class {
            fn execute(_game: &mut Game, _actor: EntityID, _noun: Noun, _second: Noun) -> bool {
                return false;
            }
        }
    };
}

pub enum Noun {
    Entity(EntityID),
    Literal(String),
    Menu(MenuType),
    Nothing,
    Number(i64),
}

pub struct ActionRequest {
    pub actor: EntityID,
    pub action: Action,
    pub noun: Noun,
    pub second: Noun,
}

pub fn is_meta(action: Action) -> bool {
    return match action {
        Action::CloseMenu(_) => true,
        Action::NavigateMenu(_) => true,
        Action::NewGame(_) => true,
        Action::OpenMenu(_) => true,
        Action::Quit(_) => true,
        Action::Restart(_) => true,
        Action::Restore(_) => true,
        Action::Save(_) => true,
        _ => false
    };
}

pub enum RuleType {
    After,
    Before,
    Life,
    Order,
    ReactAfter,
    ReactBefore,
}

pub type Rule = fn(RuleType, Action) -> bool;

macro_rules! execute_action {
    ($class:ident) => {
        Action::$class($class) => $class::execute(game, actor, noun, second)
    };
}

pub fn execute_action(game: &mut Game, action_request: ActionRequest) {
    
    let actor:EntityID = action_request.actor;
    let action = action_request.action;
    let noun: Noun = action_request.noun;
    let second: Noun = action_request.second;

    if !is_meta(action) {
        //TODO(ches) if noun exists and before rule returns true, return
    }

    let during_result: bool = match action {
        Action::CloseMenu(CloseMenu) => CloseMenu::execute(game, actor, noun, second),
        Action::NavigateMenu(NavigateMenu) => NavigateMenu::execute(game, actor, noun, second),
        Action::NewGame(NewGame) => NewGame::execute(game, actor, noun, second),
        Action::OpenMenu(OpenMenu) => OpenMenu::execute(game, actor, noun, second),
        Action::Quit(Quit) => Quit::execute(game, actor, noun, second),
        Action::Restart(Restart) => Restart::execute(game, actor, noun, second),
        Action::Restore(Restore) => Restore::execute(game, actor, noun, second),
        Action::Save(Save) => Save::execute(game, actor, noun, second),

        Action::Answer(Answer) => Answer::execute(game, actor, noun, second),
        Action::Ask(Ask) => Ask::execute(game, actor, noun, second),
        Action::AskFor(AskFor) => AskFor::execute(game, actor, noun, second),
        Action::Attack(Attack) => Attack::execute(game, actor, noun, second),
        Action::Blow(Blow) => Blow::execute(game, actor, noun, second),
        Action::Burn(Burn) => Burn::execute(game, actor, noun, second),
        Action::Buy(Buy) => Buy::execute(game, actor, noun, second),
        Action::Clean(Clean) => Clean::execute(game, actor, noun, second),
        Action::Climb(Climb) => Climb::execute(game, actor, noun, second),
        Action::Close(Close) => Close::execute(game, actor, noun, second),
        Action::Consult(Consult) => Consult::execute(game, actor, noun, second),
        Action::Crush(Crush) => Crush::execute(game, actor, noun, second),
        Action::Cut(Cut) => Cut::execute(game, actor, noun, second),
        Action::Dig(Dig) => Dig::execute(game, actor, noun, second),
        Action::Disrobe(Disrobe) => Disrobe::execute(game, actor, noun, second),
        Action::Drink(Drink) => Drink::execute(game, actor, noun, second),
        Action::Drop(Drop) => Drop::execute(game, actor, noun, second),
        Action::Eat(Eat) => Eat::execute(game, actor, noun, second),
        Action::Empty(Empty) => Empty::execute(game, actor, noun, second),
        Action::Enter(Enter) => Enter::execute(game, actor, noun, second),
        Action::Examine(Examine) => Examine::execute(game, actor, noun, second),
        Action::Exit(Exit) => Exit::execute(game, actor, noun, second),
        Action::Fill(Fill) => Fill::execute(game, actor, noun, second),
        Action::GetOff(GetOff) => GetOff::execute(game, actor, noun, second),
        Action::Give(Give) => Give::execute(game, actor, noun, second),
        Action::Go(Go) => Go::execute(game, actor, noun, second),
        Action::Insert(Insert) => Insert::execute(game, actor, noun, second),
        Action::Inventory(Inventory) => Inventory::execute(game, actor, noun, second),
        Action::Jump(Jump) => Jump::execute(game, actor, noun, second),
        Action::JumpOver(JumpOver) => JumpOver::execute(game, actor, noun, second),
        Action::Kiss(Kiss) => Kiss::execute(game, actor, noun, second),
        Action::Listen(Listen) => Listen::execute(game, actor, noun, second),
        Action::LetGo(LetGo) => LetGo::execute(game, actor, noun, second),
        Action::Lock(Lock) => Lock::execute(game, actor, noun, second),
        Action::Look(Look) => Look::execute(game, actor, noun, second),
        Action::LookUnder(LookUnder) => LookUnder::execute(game, actor, noun, second),
        Action::NotUnderstood(NotUnderstood) => NotUnderstood::execute(game, actor, noun, second),
        Action::Open(Open) => Open::execute(game, actor, noun, second),
        Action::Order(Order) => Order::execute(game, actor, noun, second),
        Action::Pray(Pray) => Pray::execute(game, actor, noun, second),
        Action::Pull(Pull) => Pull::execute(game, actor, noun, second),
        Action::Push(Push) => Push::execute(game, actor, noun, second),
        Action::PushDir(PushDir) => PushDir::execute(game, actor, noun, second),
        Action::PutOn(PutOn) => PutOn::execute(game, actor, noun, second),
        Action::Receive(Receive) => Receive::execute(game, actor, noun, second),
        Action::Remove(Remove) => Remove::execute(game, actor, noun, second),
        Action::Search(Search) => Search::execute(game, actor, noun, second),
        Action::Set(Set) => Set::execute(game, actor, noun, second),
        Action::SetTo(SetTo) => SetTo::execute(game, actor, noun, second),
        Action::Show(Show) => Show::execute(game, actor, noun, second),
        Action::Sing(Sing) => Sing::execute(game, actor, noun, second),
        Action::Sleep(Sleep) => Sleep::execute(game, actor, noun, second),
        Action::Smell(Smell) => Smell::execute(game, actor, noun, second),
        Action::Swim(Swim) => Swim::execute(game, actor, noun, second),
        Action::Swing(Swing) => Swing::execute(game, actor, noun, second),
        Action::SwitchOff(SwitchOff) => SwitchOff::execute(game, actor, noun, second),
        Action::SwitchOn(SwitchOn) => SwitchOn::execute(game, actor, noun, second),
        Action::Take(Take) => Take::execute(game, actor, noun, second),
        Action::Taste(Taste) => Taste::execute(game, actor, noun, second),
        Action::Tell(Tell) => Tell::execute(game, actor, noun, second),
        Action::Think(Think) => Think::execute(game, actor, noun, second),
        Action::ThrowAt(ThrowAt) => ThrowAt::execute(game, actor, noun, second),
        Action::ThrownAt(ThrownAt) => ThrownAt::execute(game, actor, noun, second),
        Action::Tie(Tie) => Tie::execute(game, actor, noun, second),
        Action::Touch(Touch) => Touch::execute(game, actor, noun, second),
        Action::Turn(Turn) => Turn::execute(game, actor, noun, second),
        Action::Unlock(Unlock) => Unlock::execute(game, actor, noun, second),
        Action::Wait(Wait) => Wait::execute(game, actor, noun, second),
        Action::Wake(Wake) => Wake::execute(game, actor, noun, second),
        Action::WakeOther(WakeOther) => WakeOther::execute(game, actor, noun, second),
        Action::Wave(Wave) => Wave::execute(game, actor, noun, second),
        Action::WaveHands(WaveHands) => WaveHands::execute(game, actor, noun, second),
        Action::Wear(Wear) => Wear::execute(game, actor, noun, second),
    };

    //TODO(ches) react_before of things in vicinity
    //TODO(ches) react_before of room
    
    if during_result {
        return;
    }

    if !is_meta(action) {
        //TODO(ches) if noun exists and after rule returns true, return
    }

    //TODO(ches) react_after of things in vicinity
    //TODO(ches) react_after of room
}

impl ActionRoutine for CloseMenu {
    fn execute(game: &mut Game, _actor: EntityID, _noun: Noun, _second: Noun) -> bool {
        let menu = match game.state {
            GameState::Menu(menu) => Some(menu),
            _ => None,
        };

        if menu.is_none() {
            panic!("Unknown menu specified");
        }

        match menu.unwrap() {
            MenuType::TestMenu => game.state = GameState::Menu(MenuType::Main),
            _ => ()
        }

        return false;
    }
}
impl ActionRoutine for NavigateMenu {
    fn execute(game: &mut Game, _actor: EntityID, noun: Noun, _second: Noun) -> bool {
        let maybe_direction: Option<EntityID> = match noun {
            Noun::Entity(id) => Some(id),
            Noun::Literal(_) => None,
            Noun::Menu(_) => None,
            Noun::Nothing => None,
            Noun::Number(_) => None,
        };
        if maybe_direction.is_none() {
            //TODO(ches) report an error somehow
            return true;
        }

        let menu: Option<MenuType> = match game.state {
            GameState::Menu(menu_type) => Some(menu_type),
            _ => None
        };

        if menu.is_none() {
            //TODO(ches) report an error somehow
            return true;
        }

        let maybe_menu_data: Option<&mut dyn MenuNavigation> = match menu.unwrap() {
            MenuType::Character => Some(&mut game.menu_data.character_creation),
            MenuType::TestMenu => Some(&mut game.menu_data.test_menu),
            _ => None,
        };

        if maybe_menu_data.is_none() {
            return false;
        }

        let menu_data: &mut dyn MenuNavigation = maybe_menu_data.unwrap();
        let direction = maybe_direction.unwrap();
        if direction == game.special_entities.north {
            menu_data.navigate_menu_up();
        }
        else if direction == game.special_entities.south {
            menu_data.navigate_menu_down();
        }
        else if direction == game.special_entities.up {
            menu_data.navigate_menu_out();
        }
        else if direction == game.special_entities.down {
            menu_data.navigate_menu_in();
        }
        else if direction == game.special_entities.east {
            menu_data.navigate_menu_right();
        }
        else if direction == game.special_entities.west {
            menu_data.navigate_menu_left();
        }

        return false;
    }
}
impl ActionRoutine for NewGame {
    fn execute(game: &mut Game, _actor: EntityID, _noun: Noun, _second: Noun) -> bool {
        game.state = GameState::Running;
        return false;
    }
}
impl ActionRoutine for OpenMenu {
    fn execute(game: &mut Game, _actor: EntityID, noun: Noun, _second: Noun) -> bool {
        let menu = match noun {
            Noun::Menu(menu_type) => Some(menu_type),
            _ => None
        };

        if menu.is_none() {
            panic!("Unknown menu specified");
        }

        game.state = GameState::Menu(menu.unwrap());
        return false;
    }
}
impl ActionRoutine for Quit {
    fn execute(game: &mut Game, _actor: EntityID, _noun: Noun, _second: Noun) -> bool {
        game.state = GameState::QuitRequested;
        return false;
    }
}
stub_action!(Restart);
stub_action!(Restore);
stub_action!(Save);

stub_action!(Answer);
stub_action!(Ask);
stub_action!(AskFor);
stub_action!(Attack);
stub_action!(Blow);
stub_action!(Burn);
stub_action!(Buy);
stub_action!(Clean);
stub_action!(Climb);
stub_action!(Close);
stub_action!(Consult);
stub_action!(Crush);
stub_action!(Cut);
stub_action!(Dig);
stub_action!(Disrobe);
stub_action!(Drink);
stub_action!(Drop);
stub_action!(Eat);
stub_action!(Empty);
stub_action!(Enter);
stub_action!(Examine);
stub_action!(Exit);
stub_action!(Fill);
stub_action!(GetOff);
stub_action!(Give);
impl ActionRoutine for Go {
    fn execute(game: &mut Game, actor: EntityID, noun: Noun, _second: Noun) -> bool {
        
        let maybe_direction: Option<EntityID> = match noun {
            Noun::Entity(id) => Some(id),
            Noun::Literal(_) => None,
            Noun::Menu(_) => None,
            Noun::Nothing => None,
            Noun::Number(_) => None,
        };
        if maybe_direction.is_none() {
            //TODO(ches) report an error somehow
            return true;
        }

        let maybe_position = game.components.get_position_mut(actor);
        if maybe_position.is_none() {
            //TODO(ches) report an error somehow
            return true;
        }
        
        let direction = maybe_direction.unwrap();

        let mut offset_x: i16 = 0;
        let mut offset_y: i16 = 0;

        if direction == game.special_entities.north {
            offset_y = -1;
        }
        else if direction == game.special_entities.east {
            offset_x = 1;
        }
        else if direction == game.special_entities.south {
            offset_y = 1;
        }
        else if direction == game.special_entities.west {
            offset_x = -1;
        }
        else if direction == game.special_entities.north_east {
            offset_y = -1;
            offset_x = 1;
        }
        else if direction == game.special_entities.north_west {
            offset_y = -1;
            offset_x = -1;
        }
        else if direction == game.special_entities.south_east {
            offset_y = 1;
            offset_x = 1;
        }
        else if direction == game.special_entities.south_west {
            offset_y = 1;
            offset_x = -1;
        }
        else {
            //TODO(ches) report an error somehow
        }

        let position: &mut Position = maybe_position.unwrap();
        if position.x > 0 && offset_x < 0 || position.x < game.current_map.width - 1 && offset_x > 0 {
            position.x = (position.x as i16 + offset_x) as u16;
        }
        if position.y > 0 && offset_y < 0 || position.y < game.current_map.height - 1 && offset_y > 0 {
            position.y = (position.y as i16 + offset_y) as u16;
        }

        return false;
    }
}
stub_action!(Insert);
stub_action!(Inventory);
stub_action!(Jump);
stub_action!(JumpOver);
stub_action!(Kiss);
stub_action!(Listen);
stub_action!(LetGo);
stub_action!(Lock);
stub_action!(Look);
stub_action!(LookUnder);
stub_action!(NotUnderstood);
stub_action!(Open);
stub_action!(Order);
stub_action!(Pray);
stub_action!(Pull);
stub_action!(Push);
stub_action!(PushDir);
stub_action!(PutOn);
stub_action!(Receive);
stub_action!(Remove);
stub_action!(Search);
stub_action!(Set);
stub_action!(SetTo);
stub_action!(Show);
stub_action!(Sing);
stub_action!(Sleep);
stub_action!(Smell);
stub_action!(Swim);
stub_action!(Swing);
stub_action!(SwitchOff);
stub_action!(SwitchOn);
stub_action!(Take);
stub_action!(Taste);
stub_action!(Tell);
stub_action!(Think);
stub_action!(ThrowAt);
stub_action!(ThrownAt);
stub_action!(Tie);
stub_action!(Touch);
stub_action!(Turn);
stub_action!(Unlock);
stub_action!(Wait);
stub_action!(Wake);
stub_action!(WakeOther);
stub_action!(Wave);
stub_action!(WaveHands);
stub_action!(Wear);
