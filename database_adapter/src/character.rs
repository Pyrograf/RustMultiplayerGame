use std::borrow::Borrow;
use std::hash::{Hash, Hasher};

pub type CharacterId  = u32;

#[derive(Debug)]
pub struct NewCharacterData {
    pub name: String,
    pub position_x: f32,
    pub position_y: f32,
    pub speed: f32,
}

impl NewCharacterData {
    pub fn into_with_id(self, id: CharacterId) -> CharacterData {
        CharacterData {
            id,
            name: self.name,
            position_x: self.position_x,
            position_y: self.position_y,
            speed: self.speed,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CharacterData {
    pub id: CharacterId,
    pub name: String,
    pub position_x: f32,
    pub position_y: f32,
    pub speed: f32,
}

impl PartialEq for CharacterData {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for CharacterData {}

impl Hash for CharacterData {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl Borrow<CharacterId> for CharacterData {
    fn borrow(&self) -> &CharacterId {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;
    use crate::character::CharacterData;

    #[test]
    fn test_characters_with_the_same_id_are_equal() {
        let character_1 = CharacterData {
            id: 1,
            name: "Janusz".to_string(),
            speed: 2.0,
            position_x: 0.0,
            position_y: 0.0,
        };
        let character_1_other = CharacterData {
            id: 1,
            name: "Niejanusz".to_string(),
            speed: 1.0,
            position_x: 0.0,
            position_y: 0.0,
        };
        assert_eq!(character_1, character_1_other);

        let mut characters = HashSet::new();

        assert!(characters.insert(character_1));
        assert!(!characters.insert(character_1_other));
    }
}