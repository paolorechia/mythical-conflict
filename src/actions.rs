
#[derive(Debug)]
pub enum Effect {
    Physical(i32),
    Magical(i32),
    Heal(i32),
    PhysicalMultipler(f32),
}

#[derive(Debug)]
pub struct Resistances {
    pub physical: f32,
    pub magical: f32,
}

#[derive(Debug)]
pub struct AttackAction {
    pub range: i32,
    pub effect: Effect,
    pub area: Vec<(i32, i32)>,
}

#[derive(Debug)]
pub struct BaseAttack {
    pub name: String,
    pub attack: AttackAction,
}

#[derive(Debug)]
pub struct SpecialAbility {
    pub name: String,
    pub description: String,
    pub spirit_points: i32,
    pub required_level: i32,
    pub attack: AttackAction,
}

pub enum Action {
    Attack(i32, i32),
    SpecialAbility(String),
    UseItem(String),
    Defend(),
    Rest(),
}
