use traits::create_action;

use crate::entity::Entity;

pub trait Action {}

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

pub trait Actionable {
    fn before() -> bool;
    fn during(argument: ActionArgument) -> bool;
    fn after() -> bool;
}

macro_rules! stub_before {
    () => {
        fn before() -> bool {
            return true;
        }
    };
}

macro_rules! stub_after {
    () => {
        fn after() -> bool {
            return true;
        }
    };
}

enum ActionArgument {
    Zero,
    One(Entity),
    Two(Entity, Entity),
    Count(u8)
}

enum ActionType {
    Meta,
    Gameplay
}

impl Actionable for Examine {
    stub_before!();
    fn during(argument: ActionArgument) -> bool {

        let target: Entity = match argument {
            ActionArgument::Zero => return false,
            ActionArgument::One(target) => target,
            ActionArgument::Two(_, _) => return false,
            ActionArgument::Count(_) => return false
        };
        

        return true;
    }
    stub_after!();
}
