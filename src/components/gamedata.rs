use loader::LoaderType;
use uuid::Uuid;
use std::collections::HashMap;
use super::objects::{GameObject, ObjectType};
use super::gameobject::{InstantiatedBeing};
use super::zone::{Zone, HashZone};
use resource::loader;
use super::data::{Point};

pub struct GameData {
    beings: HashMap<ObjectType, Box<GameObject>>,
    instantiated_beings: HashMap<Uuid, Box<InstantiatedBeing>>,
    game_map: HashZone,
}

impl<'a> GameData {
    pub fn new() -> GameData {
        GameData {
            beings: HashMap::new(),
            instantiated_beings: HashMap::new(),
            game_map: HashZone::new(),
        }
    }
    pub fn add_being(&mut self, game_object_name: &str) -> () {
        let game_object = self.load_game_object(game_object_name);
        self.beings.insert(game_object.get_type(), game_object);
    }

    pub fn move_being(&mut self, id: &Uuid, coords: &Point) -> () {
        self.game_map.move_id(id, coords)
    }

    pub fn instantiate_being(&mut self, object_type: ObjectType) -> () {
        let game_object_ref = self.beings.get(&object_type).unwrap();
        let new_being = InstantiatedBeing::new(object_type);
        self.instantiated_beings.insert(new_being.id, Box::new(new_being));
    }

    pub fn instantiate_localized_being(&'a mut self, object_type: ObjectType, point: &Point) -> Uuid {
        let game_object_ref = self.beings.get(&object_type).unwrap();
        let new_being = InstantiatedBeing::new(object_type);
        let new_id = new_being.id;
        self.game_map.put_coords(&new_being.id, point);
        self.instantiated_beings.insert(new_being.id, Box::new(new_being));
        new_id
    }

    pub fn get_instantiated_beings(&self) -> &HashMap<Uuid, Box<InstantiatedBeing>> {
        &self.instantiated_beings
    }

    pub fn get_beings(&self) -> &HashMap<ObjectType, Box<GameObject>> {
        &self.beings
    }

    pub fn get_being_location(&self, id: &Uuid) -> &Point {
        self.game_map.get_coords(id).unwrap()
    }

    fn load_game_object(&self, gtype: &str) -> Box<GameObject> {
        loader::load_game_type(gtype).unwrap()
    }


}