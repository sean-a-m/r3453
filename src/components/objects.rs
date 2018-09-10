use super::data::Point;
use super::zone::Zone;
use uuid::Uuid;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ObjectType {
    Player,
    Petromancer,
    Wall,
    GhostlyPresence,
    Floor,
}

pub enum Alignment {
    Good,
    Neutral,
    Evil,
}

pub struct Being {
    pub object_type: ObjectType,
    pub vitals: Vitals,
    pub physics: Physics,
    pub alignment: Alignment,
   // pub movement: Option<Movement>,
   // pub interactions: Option<Vec<&Interaction>>,
}

pub struct Movement {
    pub move_speed: i32,
}

pub struct Interaction {
    //TODO!
    pub move_speed: i32,
}


pub struct ImmaterialBeing {
    pub object_type: ObjectType,
    pub alignment: Alignment,
}

pub struct Vitals {
    pub physical_integrity: i32,
    pub spiritual_integrity: i32,
}

pub struct Physics {
    pub matter_state: MatterState,
    pub weight: f32,
    pub traversal_cost: i32,
    pub traversable: bool,
    pub blocks_sight: bool,
}

pub enum MatterState {
    Solid,
    Liquid,
    Gas,
    Plasma,
}

pub trait GameObject {
    fn get_type(&self) -> ObjectType;
}

impl GameObject for Being {
    fn get_type(&self) -> ObjectType {
        self.object_type
    }
}

impl GameObject for ImmaterialBeing {
    fn get_type(&self) -> ObjectType {
        self.object_type
    }
}