#![feature(non_ascii_idents)]
extern crate uuid;

mod components;
mod resource;

use components::data::Point;
use components::objects::{GameObject, ObjectType};
use components::zone::{HashZone, Zone};
use loader::LoaderType;
use resource::loader;
use std::collections::HashMap;
use uuid::Uuid;
use components::gamedata::{GameData};
use components::gameobject::{InstantiatedObject};

fn main() {
    let mut game_data = GameData::new();
    game_data.add_being("player");
    game_data.add_being("wall");
    game_data.add_being("petromancer");
    game_data.add_being("floor");

    let player_id = game_data.instantiate_localized_being(ObjectType::Player, &Point{x: 0, y: 0});
    game_data.instantiate_localized_being(ObjectType::Floor, &Point{x: 5, y: 5});
    game_data.instantiate_localized_being(ObjectType::Wall, &Point{x: 3, y: 7});
    game_data.instantiate_localized_being(ObjectType::Petromancer, &Point{x: 2, y: 1});

    game_data.move_being(&player_id, &Point{x: 10, y: 10});

    for (_, boxed_game_object) in game_data.get_beings() {
        let game_object = &**boxed_game_object;
        println!("Being entry: it's a {:?}!", game_object.get_type());
    }

    for (_, boxed_instantiated_being) in game_data.get_instantiated_beings() {
        let instantiated_being = &**boxed_instantiated_being;
        println!("Instantiated being entry: it's a {:?}!  It's id is {}.  It's located at: {:?}", instantiated_being.get_type(), instantiated_being.get_id(), game_data.get_being_location(instantiated_being.get_id()));
    }
}