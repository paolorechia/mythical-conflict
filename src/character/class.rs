pub mod classes;
use self::classes::warrior;
use self::classes::wizard;
use crate::actions::{BaseAttack, SpecialAbility};

#[derive(Debug)]
pub enum Class {
    Warrior(),
    Wizard(),
    Priest(),
}

impl Class {
    pub fn get_special_abilities(self: &Self) -> Vec<SpecialAbility> {
        match self {
            Class::Warrior() => warrior::get_special_abilities(),
            Class::Wizard() => wizard::get_special_abilities(),
            _ => panic!("Not implemented"),
        }
    }
    pub fn get_base_attack(self: &Self) -> BaseAttack {
        match self {
            Class::Warrior() => warrior::get_base_attack(),
            Class::Wizard() => wizard::get_base_attack(),
            _ => panic!("Not implemented"),
        }
    }
}
