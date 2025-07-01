use std::collections::HashMap;
use crate::game::entity::EntityId;
use crate::game::math::Vec2F;

pub type CharacterId  = u32;

#[derive(Debug, Clone)]
pub struct CharacterData {
    pub character_id: CharacterId,
    pub name: String,
    pub position: Vec2F,
    pub speed: f32,
}

pub struct CharactersDatabase {
    characters: HashMap<CharacterId, CharacterData>,
}

impl CharactersDatabase {
    pub fn new_test() -> Self {
        let mut characters = HashMap::new();

        let character_id = 1;
        debug_assert!(
            characters.insert(character_id, CharacterData {
                character_id,
                name: "Janusz".to_string(),
                position: Vec2F::new(0.0, 0.0),
                speed: 1.0,
            }).is_none()
        );

        let character_id = 2;
        debug_assert!(
            characters.insert(character_id, CharacterData {
                character_id,
                name: "Tuna".to_string(),
                position: Vec2F::new(2.0, 0.0),
                speed: 1.0,
            }).is_none()
        );

        let character_id = 3;
        debug_assert!(
            characters.insert(character_id, CharacterData {
                character_id,
                name: "Raspberry".to_string(),
                position: Vec2F::new(-3.0, 4.0),
                speed: 1.0,
            }).is_none()
        );

        Self {
            characters,
        }
    }

    pub fn get_character(&self, character_id: CharacterId) -> Option<&CharacterData> {
        self.characters.get(&character_id)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_creating_temporary_characters_database() {
        let _database = CharactersDatabase::new_test();
    }
}