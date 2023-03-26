

pub enum EventRequest {
    MoveEventRequest(i32, i32),
    AttackEventRequest(i32, i32),
    SpecialAbilityRequest(String),
    UseItemRequest(String),
    DefendRequest(),
    RestRequest(),
}