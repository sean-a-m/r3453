use components::objects::{
    Being, ImmaterialBeing, Vitals, ObjectType, Alignment, Physics, MatterState, GameObject
};

use components::objects;

pub enum LoaderType {
    Being(objects::Being),
    ImmaterialBeing(objects::ImmaterialBeing),
}

pub fn load_game_type(name: &str) -> Result<Box<GameObject>, &'static str> {
    match name {
        "player" => Ok(instantiate_player()),
        "wall" => Ok(instantiate_wall()),
        "petromancer" => Ok(instantiate_petromancer()),
        "ghostly_presence" => Ok(instantiate_ghostly_presence()),
        "floor" => Ok(instantiate_floor()),
        _ => Err("Something happened"),
    }
}

fn instantiate_player() -> Box<Being> {
    Box::new(
            Being {
            object_type: ObjectType::Player,
            vitals: Vitals {
                physical_integrity: 100,
                spiritual_integrity: 100,
            },
            alignment: Alignment::Neutral,
            physics: Physics{
                matter_state: MatterState::Solid,
                weight: 75.0,
                traversal_cost: 0,
                traversable: false,
                blocks_sight: false,
            }
        }
    )
}

fn instantiate_wall() -> Box<Being> {
    Box::new(Being {
        object_type: ObjectType::Wall,
        vitals: Vitals {
            physical_integrity: 1000,
            spiritual_integrity: 10000000,
        },
        alignment: Alignment::Neutral,
        physics: Physics{
            matter_state: MatterState::Solid,
            weight: 7500.0,
            traversal_cost: 0,
            traversable: false,
            blocks_sight: true,
        }
    })
}

fn instantiate_petromancer() -> Box<Being> {
    Box::new(Being {
        object_type: ObjectType::Petromancer,
        vitals: Vitals {
            physical_integrity: 15,
            spiritual_integrity: 200,
        },
        alignment: Alignment::Neutral,
        physics: Physics{
            matter_state: MatterState::Solid,
            weight: 75.0,
            traversal_cost: 0,
            traversable: false,
            blocks_sight: false,
        }
    })
}

fn instantiate_ghostly_presence() -> Box<ImmaterialBeing> {
    Box::new (ImmaterialBeing {
        object_type: ObjectType::GhostlyPresence,
        alignment: Alignment::Evil,
    })
}

fn instantiate_floor() -> Box<Being> {
    Box::new(Being {
        object_type: ObjectType::Floor,
        vitals: Vitals {
            physical_integrity: 1500,
            spiritual_integrity: 20000,
        },
        alignment: Alignment::Neutral,
        physics: Physics{
            matter_state: MatterState::Solid,
            weight: 7500.0,
            traversal_cost: 1,
            traversable: true,
            blocks_sight: false,
        }
    })
}
