use traits::create_action;

use crate::{entity::{Entity, EntityID}, game::{Game, GameState}};

create_action!(ExitMenu);
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
    ExitMenu(ExitMenu),
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

pub trait Actionable {
    fn before() -> bool;
    fn during(game: &mut Game, noun: &Option<impl Actor>, second: &Option<impl Actor>) -> bool;
    fn after() -> bool;
}

macro_rules! stub_before {
    () => {
        fn before() -> bool {
            return false;
        }
    };
}

macro_rules! stub_during {
    () => {
        fn during(_game: &mut Game, _noun: &Option<impl Actor>, _second: &Option<impl Actor>) -> bool {
            return false;
        }
    };
}

macro_rules! stub_after {
    () => {
        fn after() -> bool {
            return false;
        }
    };
}

pub struct ActionRequest {
    pub actor: EntityID,
    pub action: Action,
    pub noun: EntityID,
    pub second: EntityID,
}

pub fn is_meta(action: Action) -> bool {
    return match action {
        Action::ExitMenu(_) => true,
        Action::Quit(_) => true,
        Action::Restart(_) => true,
        Action::Restore(_) => true,
        Action::Save(_) => true,
        _ => false
    };
}

pub trait Actor {
    fn before(&self, action: Action) -> bool;
    fn after(&self, action: Action) -> bool;
    fn react_before(&self, action: Action) -> bool;
    fn react_after(&self, action: Action) -> bool;
}

#[macro_export]
macro_rules! stub_actor {
    () => {
        fn before(&self, _action: Action) -> bool {
            return false;
        }
        fn after(&self, _action: Action) -> bool {
            return false;
        }
        fn react_before(&self, _action: Action) -> bool {
            return false;
        }
        fn react_after(&self, _action: Action) -> bool {
            return false;
        }
    }
}


pub fn execute_action(game: &mut Game, action: Action, noun: Option<impl Actor>, second: Option<impl Actor>) {
    
    if !is_meta(action) {
        if noun.is_some() && noun.as_ref().unwrap().before(action) {
            return;
        }
    }

    let during_result: bool = match action {
        Action::Quit(Quit) => Quit::during(game, &noun, &second),
        Action::Examine(Examine) => Examine::during(game, &noun, &second),
        _ => false,
    };

    //TODO(ches) react_before of things in vicinity
    //TODO(ches) react_before of room
    
    if during_result {
        return;
    }

    if !is_meta(action) {
        if noun.is_some() && noun.as_ref().unwrap().after(action) {
            return;
        }
    }

    //TODO(ches) react_after of things in vicinity
    //TODO(ches) react_after of room
}

impl Actionable for Quit {
    stub_before!();
    fn during(game: &mut Game, _noun: &Option<impl Actor>, _second: &Option<impl Actor>) -> bool {
        game.state = GameState::QuitRequested;
        return false;
    }
    stub_after!();
}

impl Actionable for Examine {
    stub_before!();
    stub_during!();
    stub_after!();
}
