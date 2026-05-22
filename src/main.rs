use crate::buildings::generate_voronoi;

pub mod buildings;
pub fn main() {
    let v = buildings::setup_city_collection(1000, 1000);
    v.render();
    v.render_funny();
}
