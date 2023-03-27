use mythic_conflict::character::{self, race, attributes::Attributes, class::Class};
use mythic_conflict::position::Position;

#[test]
fn test_warrior() {
    let warrior = character::Character {
        name: String::from("Robert Johnnson"),
        race: race::Race::human(),
        class: Class::Warrior(),
        level: 1,
        bonus_attributes: Attributes::starting_attributes(),
        position: Position { x: 0, y: 0}
    };
    let abilities = warrior.class.get_special_abilities();
    assert!(abilities.len() > 0);
    warrior.class.get_base_attack();
}

#[test]
fn test_wizard() {
    let wizard: character::Character = character::Character {
        name: String::from("Joseph Gordon"),
        race: race::Race::elf(),
        class: Class::Wizard(),
        level: 1,
        bonus_attributes: Attributes::starting_attributes(),
        position: Position { x: 1, y: 0}
    };

    let abilities = wizard.class.get_special_abilities();
    assert!(abilities.len() > 0);
    wizard.class.get_base_attack();
}
