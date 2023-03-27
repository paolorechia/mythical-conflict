#[derive(Debug)]
pub struct Attributes {
    pub strength: i32,
    pub resilience: i32,
    pub dexterity: i32,
    pub spirit: i32,
    pub intelligence: i32,
    pub wisdom: i32,
    pub speed: i32,
    pub agility: i32,
}

impl Attributes {
    pub fn starting_attributes() -> Attributes {
        Attributes {
            strength: 0,
            resilience: 0,
            dexterity: 0,
            spirit: 0,
            intelligence: 0,
            wisdom: 0,
            speed: 0,
            agility: 0
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_starting_attributes() {
        /* This test is not too useful, but shows how to setup an unit test. */
        let attributes = Attributes::starting_attributes();
        assert_eq!(attributes.strength, 0);
        assert_eq!(attributes.resilience, 0);
        assert_eq!(attributes.dexterity, 0);
        assert_eq!(attributes.spirit, 0);
        assert_eq!(attributes.intelligence, 0);
        assert_eq!(attributes.wisdom, 0);
        assert_eq!(attributes.speed, 0);
        assert_eq!(attributes.agility, 0);
    }
}