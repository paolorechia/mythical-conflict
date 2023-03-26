pub mod actions;
pub mod character;
pub mod combatable;
pub mod events;
pub mod grid;
pub mod position;

use crate::character::{race, attributes::Attributes, class::Class};
use crate::position::Position;

fn main() {
    let robert = character::Character {
        name: String::from("Robert Johnnson"),
        race: race::Race::human(),
        class: Class::Warrior(),
        level: 1,
        bonus_attributes: Attributes::starting_attributes(),
        position: Position { x: 0, y: 0}
    };

    let joseph: character::Character = character::Character {
        name: String::from("Joseph Gordon"),
        race: race::Race::elf(),
        class: Class::Wizard(),
        level: 1,
        bonus_attributes: Attributes::starting_attributes(),
        position: Position { x: 1, y: 0}
    };

    println!("Character 1:");
    println!("{robert:?}");

    let abilities = robert.class.get_special_abilities();

    println!("{abilities:?}");

    let base_attack = robert.class.get_base_attack();

    println!("{base_attack:?}");

    println!();
    println!("Character 2:");
    println!("{joseph:?}");
    let abilities = joseph.class.get_special_abilities();
    println!("{abilities:?}");
    let base_attack = joseph.class.get_base_attack();
    println!("{base_attack:?}");
}
