extern crate uuid;

mod components;
mod resource;

use components::data::Point;
use components::objects::{GameObjectData, Localizable, GameObject};
use components::zone::{HashZone, Zone};
use loader::LoaderType;
use resource::loader;

fn main() {
    println!("Starting!");
    Point { x: 3, y: 4 };

    let mut game_objects = Vec::new();
    let mut game_map = HashZone::new();

    instantiate_being(&mut game_objects, &mut game_map, "player", &Point { x: 0, y: 0 });
    instantiate_being(&mut game_objects, &mut game_map, "wall", &Point { x: 2, y: 5 });
    instantiate_being(&mut game_objects, &mut game_map, "petromancer", &Point { x: 3, y: 4 });
    instantiate_being(&mut game_objects, &mut game_map, "ghostly_presence", &Point { x: 1, y: 7 });

    for (id, point) in &game_map.list_all() {
        println!("{}, {:?}", id, point);
    }

    for game_object in &game_objects {
        match game_object {
            LoaderType::Being(p) => println!("{}", p.game_object_data.id),
            LoaderType::ImmaterialBeing(p) => println!("{}", p.game_object_data.id),
        }
    }
    

    for game_object in &game_objects {
        match game_object {
            LoaderType::Being(p) => 
                println!("It's a Being struct.  It's a {:?}.  It has {:?} physical health and {:?} spiritual health. It's at point {:?}", 
                p.game_object_data.object_type, p.vitals.physical_integrity, p.vitals.spiritual_integrity, p.get_position(&game_map)),
            LoaderType::ImmaterialBeing(p) => println!("It's an ImmaterialBeing struct.  It's a {:?}.", p.game_object_data.object_type)
        }
    }
}

fn instantiate_being(
    game_object_vec: &mut Vec<LoaderType>,
    zone_vec: &mut HashZone,
    gtype: &str,
    point: &Point,
) -> () {
    let new_game_object = load_game_object(gtype);
    match new_game_object{
        LoaderType::Being(p) => {
            zone_vec.put_coords(&p.game_object_data.id, *point);
            game_object_vec.push(LoaderType::Being(p));
        }
        LoaderType::ImmaterialBeing(p) => {
            zone_vec.put_coords(&p.game_object_data.id, *point);
            game_object_vec.push(LoaderType::ImmaterialBeing(p));
        }
    }
}

fn load_game_object(gtype: &str) -> LoaderType {
    loader::load_game_type(gtype).unwrap()
}
