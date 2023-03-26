pub mod class;
pub mod attributes;
pub mod race;

use crate::actions::Action;
use self::{class::Class, attributes::Attributes};
use super::combatable;
use super::position::Position;
use crate::events::EventRequest;

#[derive(Debug)]
pub struct Character {
    pub name: String,
    pub race: race::Race,
    pub class: Class,
    pub level: i32, 
    pub bonus_attributes: attributes::Attributes,
    pub position: Position,
}

impl Character {
    fn attributes(&self) -> Attributes {
        Attributes {
            strength: self.race.base_attributes.strength + self.bonus_attributes.strength,
            speed: self.race.base_attributes.speed + self.bonus_attributes.speed,
            spirit: self.race.base_attributes.spirit + self.bonus_attributes.spirit,
            resilience: self.race.base_attributes.resilience + self.bonus_attributes.resilience,
            wisdom: self.race.base_attributes.wisdom + self.bonus_attributes.wisdom,
            intelligence: self.race.base_attributes.intelligence + self.bonus_attributes.intelligence,
            dexterity: self.race.base_attributes.dexterity + self.bonus_attributes.dexterity,
            agility: self.race.base_attributes.agility + self.bonus_attributes.agility,
        }
    }
}


impl combatable::Combatable for Character {
    fn move_to(&self, x: i32, y: i32) -> Result<EventRequest, combatable::CombatError>{
        let speed = self.attributes().speed;
        let x_distance  = (self.position.x - x).abs();
        let y_distance = (self.position.y - y).abs();
        if x_distance + y_distance > speed {
            Err(combatable::CombatError::MoveError("Move target position is beyond character reach"))
        } else {
            Ok(EventRequest::MoveEventRequest(x, y))
        }

    }
    fn act(&self, action: Action) -> Result<EventRequest, combatable::CombatError> {
        match action {
            Action::Attack(x, y) => {
                let x_distance  = (self.position.x - x).abs();
                let y_distance = (self.position.y - y).abs();
                if x_distance + y_distance > 1 {
                    Err(combatable::CombatError::ActionError("Attack beyond reach"))
                } else {
                    Ok(EventRequest::AttackEventRequest(x, y))
                }
            },
            Action::SpecialAbility(name) => {
                let special_abilities = self.class.get_special_abilities();
                for ability in special_abilities {
                    if name == ability.name {
                        return Ok(EventRequest::SpecialAbilityRequest(name))
                    }
                }
                Err(combatable::CombatError::ActionError("Unknown special ability"))
            },
            Action::UseItem(_) => {
                Err(combatable::CombatError::ActionError("Not implemented!"))
            }
            Action::Defend() => Ok(EventRequest::DefendRequest()),
            Action::Rest() => Ok(EventRequest::RestRequest()),
        }
    }
}