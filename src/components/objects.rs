use super::data::Point;
use super::zone::Zone;
use uuid::Uuid;

pub struct Being {
    pub id: Uuid,
    pub creature: Creature,
}

pub struct Vitals {
    pub physical_integrity: i32,
    pub spiritual_integrity: i32,
}

#[derive(Debug, Copy, Clone)]
pub enum GameType {
    VitalBeing,
    ImmaterialBeing,
}

#[derive(Debug, Copy, Clone)]
pub enum Creature {
    Player,
    Petromancer,
    Wall,
    GhostlyPresence,
}

pub trait Localizable {
    fn get_position<'a, T: Zone>(&self, this_zone: &'a T) -> &'a Point;
}

impl Localizable for Being {
    fn get_position<'a, T: Zone>(&self, this_zone: &'a T) -> &'a Point {
        this_zone
            .get_coords(&self.id)
            .expect(format!("Couldn't locate {}", self.id).as_str())
    }
}

pub struct VitalBeing {
    pub game_type: GameType,
    pub being: Being,
    pub vitals: Vitals,
}

pub struct ImmaterialBeing {
    pub game_type: GameType,
    pub being: Being,
}

pub trait GameObject {
    fn game_type(&self) -> GameType;
    fn id(&self) -> &Uuid;
}

impl GameObject for VitalBeing {
    fn game_type(&self) -> GameType {
        self.game_type
    }

    fn id(&self) -> &Uuid {
        &self.being.id
    }
}

impl GameObject for ImmaterialBeing {
    fn game_type(&self) -> GameType {
        self.game_type
    }

    fn id(&self) -> &Uuid {
        &self.being.id
    }
}
