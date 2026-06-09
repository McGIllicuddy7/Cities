use std::{collections::BTreeMap, sync::Arc};

use crate::{
    buildings::generate_voronoi,
    logistics::{Constraint, ConstraintSet, State},
    utils::Timer,
};

pub mod buildings;
pub mod city;
pub mod logistics;
pub mod utils;
pub fn make_constraints_set() {
    let c = ConstraintSet::maximize(
        |s| s["x"] * s["x"] + s["y"] - s["z"] - s["w"],
        &[
            Constraint::ge(|s| s["x"], -10),
            Constraint::ge(|s| s["y"], 100),
            Constraint::ge(|s| s["w"], 100),
            Constraint::ge(|s| s["z"], 1000),
            Constraint::le(|s| s["x"] + s["y"] + s["z"] + s["w"], 2000),
        ],
    );
    let base = State::new(&[("x", 0), ("y", 0), ("z", 0), ("w", 0)]);
    let out = c.solve(base);
    println!("{:#?}", out);
}

pub fn main() {
    let _timer = Timer::new("city generation");
    // make_constraints_set();
    let width = 2000;
    let height = 2000;
    let city = buildings::setup_city_collection(width, height);
    let building_count = city.collections.len() as i32;
    let size_hectares = width * height / (180 * 180);
    let density = (building_count as f32) / (size_hectares as f32);
    let mut average_size = 0.;
    for (_, i) in &city.collections {
        average_size += i.points.len() as f32;
    }
    average_size /= building_count as f32;
    average_size /= (180 * 180) as f32;
    city.render_funny();
    println!(
        "building_count:{}, size:{} hectares, density:{} buildings per hectare, average building size:{} hectares",
        building_count, size_hectares, density, average_size
    );
}
