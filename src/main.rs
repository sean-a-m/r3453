extern crate uuid;

mod components;
mod resource;

use components::data::Point;
use components::objects::{GameObject, Localizable};
use components::zone::{HashZone, Zone};
use loader::LoaderType;
use resource::loader;

fn main() {
    println!("Starting!");
    Point { x: 3, y: 4 };

    let mut game_objects = Vec::new();
    let mut game_map = HashZone::new();

    instantiate_being(
        &mut game_objects,
        &mut game_map,
        "player",
        &Point { x: 0, y: 0 },
    );
    instantiate_being(
        &mut game_objects,
        &mut game_map,
        "wall",
        &Point { x: 2, y: 5 },
    );
    instantiate_being(
        &mut game_objects,
        &mut game_map,
        "petromancer",
        &Point { x: 3, y: 4 },
    );
    instantiate_being(
        &mut game_objects,
        &mut game_map,
        "ghostly_presence",
        &Point { x: 1, y: 7 },
    );

    for (id, point) in &game_map.list_all() {
        println!("{}, {:?}", id, point);
    }

    for game_object in &game_objects {
        match game_object {
            LoaderType::lVitalBeing(p) => println!("{}", p.being.id),
            LoaderType::lImmaterialBeing(p) => println!("{}", p.being.id),
        }
    }

    for game_object in &game_objects {
        match game_object {
            LoaderType::lVitalBeing(p) => 
                println!("It's a {:?} struct.  It's a {:?}.  It has {:?} physical health and {:?} spiritual health. It's at point {:?}", 
                p.game_type(), p.being.creature, p.vitals.physical_integrity, p.vitals.spiritual_integrity, p.being.get_position(&game_map)),
            LoaderType::lImmaterialBeing(p) => println!("It's a {:?} struct.  It's a {:?}.", p.game_type(), p.being.creature)
        }
    }
}

fn instantiate_being(
    game_object_vec: &mut Vec<LoaderType>,
    zone_vec: &mut HashZone,
    gtype: &str,
    point: &Point,
) -> () {
    let (new_game_object, id) = load_game_object(gtype);
    game_object_vec.push(new_game_object);
    zone_vec.put_coords(id, *point);
}

fn load_game_object(gtype: &str) -> (LoaderType, uuid::Uuid) {
    loader::load_game_type(gtype).unwrap()
}
