use super::data::Point;
use std::collections::{HashMap, HashSet, hash_map::Iter};
use uuid::Uuid;

pub trait Zone {
    fn get_coords(&self, id: &Uuid) -> Option<&Point>;
    fn put_coords(&mut self, id: &Uuid, coords: &Point) -> ();
    fn get_at_coords(&self, coords: &Point) -> Option<&HashSet<Uuid>>;
    fn move_id(&mut self, id: &Uuid, coords: &Point) -> ();
}

pub struct HashZone {
    pub beings: HashMap<Uuid, Point>,
    pub locations: HashMap<Point, HashSet<Uuid>>,
}

impl HashZone {
    pub fn new() -> HashZone {
        HashZone {
            beings: HashMap::new(),
            locations: HashMap::new(),
        }
    }

    fn list_all_things(&self) -> Iter<Uuid, Point> {
        self.beings.iter()
    }
}

impl Zone for HashZone {
    fn get_coords(&self, id: &Uuid) -> Option<&Point> {
        self.beings.get(id)
    }

    fn get_at_coords(&self, coords: &Point) -> Option<&HashSet<Uuid>> {
        self.locations.get(coords)
    }

    fn put_coords(&mut self, id: &Uuid, coords: &Point) {
        self.beings.insert(*id, *coords);
        self.locations
            .entry(*coords)
            .or_insert_with(|| HashSet::new())
            .insert(*id);
    }

    fn move_id(&mut self, id: &Uuid, coords: &Point) -> () {
        match self.beings.get(id) {
            Some(point) => {
                self.locations.get_mut(point)
                    .unwrap()
                    .remove(id);
            },
            None => {},
        }
        self.put_coords(id, coords);
    }
}
