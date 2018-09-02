use components::objects::{
    Being, ImmaterialBeing, Vitals, GameObjectData, ObjectType, GameObject
};

use components::objects;
use uuid::Uuid;

pub enum LoaderType {
    Being(objects::Being),
    ImmaterialBeing(objects::ImmaterialBeing),
}

pub fn load_game_type(name: &str) -> Result<LoaderType, &'static str> {
    match name {
        "player" => Ok(instantiate_player()),
        "wall" => Ok(instantiate_wall()),
        "petromancer" => Ok(instantiate_petromancer()),
        "ghostly_presence" => Ok(instantiate_ghostly_presence()),
        _ => Err("Something happened"),
    }
}

fn instantiate_player() -> LoaderType {
    let return_obj = Being {
        game_object_data: GameObjectData::new(ObjectType::Player),
        vitals: Vitals {
            physical_integrity: 100,
            spiritual_integrity: 100,
        },
    };

    LoaderType::Being(return_obj)
}

fn instantiate_wall() -> LoaderType {
    let return_obj = Being {
        game_object_data: GameObjectData::new(ObjectType::Wall),
        vitals: Vitals {
            physical_integrity: 1000,
            spiritual_integrity: 10000000,
        },
    };
    LoaderType::Being(return_obj)
}

fn instantiate_petromancer() -> LoaderType {
    let return_obj = Being {
        game_object_data: GameObjectData::new(ObjectType::Petromancer),
        vitals: Vitals {
            physical_integrity: 15,
            spiritual_integrity: 200,
        },
    };
    LoaderType::Being(return_obj)
}

fn instantiate_ghostly_presence() -> LoaderType {
    let return_obj = ImmaterialBeing {
        game_object_data: GameObjectData::new(ObjectType::GhostlyPresence),
    };
    LoaderType::ImmaterialBeing(return_obj)
}
