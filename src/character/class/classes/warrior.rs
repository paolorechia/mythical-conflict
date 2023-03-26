use crate::actions::{AttackAction, BaseAttack, Effect, SpecialAbility};

pub fn get_special_abilities() -> Vec<SpecialAbility> {
    let mut v: Vec<SpecialAbility> = Vec::<SpecialAbility>::new();
    v.push(SpecialAbility {
        name: String::from("Powerful attack"),
        description: String::from("A powerful attack that lands more damage"),
        spirit_points: 1,
        required_level: 1,
        attack: AttackAction {
            range: 1,
            effect: Effect::PhysicalMultipler(1.2),
            area: vec![(0 as i32, 0 as i32)],
        },
    });
    v
}

pub fn get_base_attack() -> BaseAttack {
    BaseAttack {
        name: String::from("Sword"),
        attack: AttackAction {
            range: 1,
            effect: Effect::Physical(10),
            area: vec![(0 as i32, 0 as i32)],
        },
    }
}
