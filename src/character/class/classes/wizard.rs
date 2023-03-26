use crate::actions::{AttackAction, BaseAttack, Effect, SpecialAbility};

pub fn get_special_abilities() -> Vec<SpecialAbility> {
    let mut v: Vec<SpecialAbility> = Vec::<SpecialAbility>::new();
    v.push(SpecialAbility {
        name: String::from("Magic Bolt"),
        description: String::from("A low level ranged magical attack"),
        spirit_points: 1,
        required_level: 1,
        attack: AttackAction {
            range: 8,
            effect: Effect::Magical(7),
            area: vec![(0 as i32, 0 as i32)],
        },
    });
    v
}

pub fn get_base_attack() -> BaseAttack {
    BaseAttack {
        name: String::from("Staff"),
        attack: AttackAction {
            range: 1,
            effect: Effect::Physical(2),
            area: vec![(0 as i32, 0 as i32)],
        },
    }
}
