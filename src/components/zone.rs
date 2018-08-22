use super::data::Point;
use std::collections::HashMap;
use uuid::Uuid;

pub trait Zone {
    fn get_coords(&self, id: &Uuid) -> Option<&Point>;
    fn put_coords(&mut self, id: Uuid, coords: Point) -> ();
    fn list_all(&self) -> Vec<(&Uuid, &Point)>;
}

pub struct HashZone {
    pub beings: HashMap<Uuid, Point>,
}

impl HashZone {
    pub fn new() -> HashZone {
        HashZone {
            beings: HashMap::new(),
        }
    }
}

impl Zone for HashZone {
    fn get_coords(&self, id: &Uuid) -> Option<&Point> {
        self.beings.get(id)
    }

    fn put_coords(&mut self, id: Uuid, coords: Point) {
        self.beings.insert(id, coords);
    }

    fn list_all(&self) -> Vec<(&Uuid, &Point)> {
        let mut all_beings = Vec::new();
        for (k, v) in &self.beings {
            //Convert string to reference, lifetime is ok since my vec will exist for a while...I hope
            all_beings.push((k, v));
        }
        return all_beings;
    }
}
