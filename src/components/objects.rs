use super::data::Point;
use super::zone::Zone;
use uuid::Uuid;

pub struct GameObjectData {
    pub id: Uuid,
    pub object_type: ObjectType,
}

impl GameObjectData{
    pub fn new(object_type: ObjectType) -> GameObjectData {
        GameObjectData {
            id: Uuid::new_v4(),
            object_type: object_type,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub enum ObjectType {
    Player,
    Petromancer,
    Wall,
    GhostlyPresence,
}

pub struct Being {
    pub game_object_data: GameObjectData,
    pub vitals: Vitals,
}

pub struct ImmaterialBeing {
    pub game_object_data: GameObjectData,
}

pub struct Vitals {
    pub physical_integrity: i32,
    pub spiritual_integrity: i32,
}

pub trait Localizable {
    fn get_position<'a, T: Zone>(&self, this_zone: &'a T) -> &'a Point;
}

impl Localizable for Being {
    fn get_position<'a, T: Zone>(&self, this_zone: &'a T) -> &'a Point {
        this_zone
            .get_coords(&self.game_object_data.id)
            .expect(format!("Couldn't locate {}", self.game_object_data.id).as_str())
    }
}

pub trait GameObject {
    fn object_type(&self) -> ObjectType;
    fn id(&self) -> &Uuid;
}

impl GameObject for Being {
    fn id(&self) -> &Uuid {
        &self.game_object_data.id
    }

    fn object_type(&self) -> ObjectType {
        self.game_object_data.object_type
    }
}

impl GameObject for ImmaterialBeing {
    fn id(&self) -> &Uuid {
        &self.game_object_data.id
    }

    fn object_type(&self) -> ObjectType {
        self.game_object_data.object_type
    }
}