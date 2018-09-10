use super::data::Point;
use super::zone::Zone;
use super::objects::{Being, GameObject, ObjectType};
use uuid::Uuid;

pub struct InstantiatedBeing {
    pub id: Uuid,
    being: ObjectType,
  //  inventory: Option<Inventory>,
  //  mutated_actions: Vec<&Action>
    vital_state: VitalState,
}

struct VitalState {
    physical_damage: i32,
    spiritual_damage: i32,
}

impl VitalState {
    pub fn new() -> VitalState {
        VitalState {
            physical_damage: 0,
            spiritual_damage: 0,
        }
    }
}

/*
pub struct Inventory {
    inventory: Vec<&InstantiatedBeing>,
}*/

impl<'a> InstantiatedBeing {
    pub fn new(being: ObjectType) -> InstantiatedBeing {
        InstantiatedBeing {
            id: Uuid::new_v4(),
            being: being, 
            vital_state: VitalState::new(),
        }
    }
}

pub trait Localizable {
    fn get_position<'a, T: Zone>(&self, this_zone: &'a T) -> &'a Point;
}

impl<'a> Localizable for InstantiatedBeing {
    fn get_position<'b, T: Zone>(&self, this_zone: &'b T) -> &'b Point {
        this_zone
            .get_coords(&self.id)
            .expect(format!("Couldn't locate {}", self.id).as_str())
    }
}

pub trait InstantiatedObject {
    fn get_id(&self) -> &Uuid;
    fn get_type(&self) -> ObjectType;
}

impl InstantiatedObject for InstantiatedBeing{
    fn get_id(&self) -> &Uuid {
        &self.id
    }

    fn get_type(&self) -> ObjectType {
        self.being
    }
}

