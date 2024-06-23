use crate::context::Context;
use crate::road;
pub struct City {
    roads: Vec<road::Road>,
}
impl City {
    pub fn new(radius: f64, context: &Context) -> Self {
        let rings = road::generate_ring_system(radius, context);
        let roads = road::collect_rings_to_roads(&rings);
        return Self { roads: roads };
    }
    pub fn draw(&self, context: &Context) {
        for r in &self.roads {
            r.draw(context);
        }
    }
}

