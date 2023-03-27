use super::attributes::Attributes;

#[derive(Debug)]
pub struct Race<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub base_attributes: Attributes,
}

impl<'a> Race<'a> {
    pub fn human() -> Self {
        Race {
            name: "Human",
            description: "Humans are a versatile race, with no specific strengths or weaknesses. They have average scores in all attributes.",
            base_attributes: Attributes {
                strength: 50,
                resilience: 50,
                dexterity: 50,
                spirit: 50,
                intelligence: 50,
                wisdom: 50,
                speed: 6,
                agility: 50,
            }
        }
    }
    pub fn elf() -> Self {
        Race {
            name: "Elf",
            description: "Elves are a nimble and intelligent race, with a natural affinity for magic and ranged combat. They tend to be physically weaker than other races, but are very agile and fast.",
            base_attributes: Attributes {
                strength: 40,
                resilience: 40,
                dexterity: 55,
                spirit: 50,
                intelligence: 55,
                wisdom: 55,
                speed: 6,
                agility: 55,
            }
        }
    }
}
