use components::objects::{
    Being, Creature, GameObject, GameType, ImmaterialBeing, VitalBeing, Vitals,
};
use uuid::Uuid;

pub enum LoaderType {
    lVitalBeing(VitalBeing),
    lImmaterialBeing(ImmaterialBeing),
}

pub fn load_game_type(name: &str) -> Result<(LoaderType, Uuid), &'static str> {
    match name {
        "player" => Ok(instantiate_player()),
        "wall" => Ok(instantiate_wall()),
        "petromancer" => Ok(instantiate_petromancer()),
        "ghostly_presence" => Ok(instantiate_ghostly_presence()),
        _ => Err("Something happened"),
    }
}

fn instantiate_player() -> (LoaderType, Uuid) {
    let id: Uuid = Uuid::new_v4();
    let return_obj = VitalBeing {
        game_type: GameType::VitalBeing,
        being: Being {
            creature: Creature::Player,
            id: id,
        },
        vitals: Vitals {
            physical_integrity: 100,
            spiritual_integrity: 100,
        },
    };

    (LoaderType::lVitalBeing(return_obj), id)
}

fn instantiate_wall() -> (LoaderType, Uuid) {
    let id: Uuid = Uuid::new_v4();
    let return_obj = VitalBeing {
        game_type: GameType::VitalBeing,
        being: Being {
            creature: Creature::Wall,
            id: id,
        },
        vitals: Vitals {
            physical_integrity: 1000,
            spiritual_integrity: 10000000,
        },
    };
    (LoaderType::lVitalBeing(return_obj), id)
}

fn instantiate_petromancer() -> (LoaderType, Uuid) {
    let id: Uuid = Uuid::new_v4();
    let return_obj = VitalBeing {
        game_type: GameType::VitalBeing,
        being: Being {
            creature: Creature::Petromancer,
            id: id,
        },
        vitals: Vitals {
            physical_integrity: 15,
            spiritual_integrity: 200,
        },
    };
    (LoaderType::lVitalBeing(return_obj), id)
}

fn instantiate_ghostly_presence() -> (LoaderType, Uuid) {
    let myid: Uuid = Uuid::new_v4();
    let return_obj = ImmaterialBeing {
        game_type: GameType::ImmaterialBeing,
        being: Being {
            creature: Creature::GhostlyPresence,
            id: myid,
        },
    };
    (LoaderType::lImmaterialBeing(return_obj), myid)
}
