use crate::actions;
use crate::events::EventRequest;

pub enum CombatError {
    MoveError(&'static str),
    ActionError(&'static str)
}

pub trait Combatable {
    fn move_to(&self, x: i32, y: i32) -> Result<EventRequest, CombatError>;
    fn act(&self, action: actions::Action) -> Result<EventRequest, CombatError>;
}
