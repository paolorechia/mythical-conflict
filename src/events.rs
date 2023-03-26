use crate::actions::SpecialAbility;

pub enum EventRequest {
    MoveEventRequest(i32, i32),
    AttackEventRequest(i32, i32),
    SpecialAbilityRequest(SpecialAbility),
    UseItemRequest(String),
    DefendRequest(),
    RestRequest(),
}