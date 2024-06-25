use crate::context::Context;
use crate::{building, road};
#[allow(unused)]
pub struct City {
    roads: Vec<road::Road>,
    buildings: Vec<building::Building>,
}

impl City {
    pub fn new(radius: f64, context: &Context) -> Self {
        let rings = road::generate_ring_system(radius, context);
        let roads = road::collect_rings_to_roads(&rings);
        let buildings = building::generate_buildings(rings.as_slice());
        return Self { roads: roads ,buildings};
    }
    pub fn draw(&self, context: &Context) {
        for r in &self.roads {
          //r.draw(context);
        }
        for b in &self.buildings{
            b.draw(context);
        }
    }
}

