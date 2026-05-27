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
        |state| state["x"] * state["x"] + state["y"] - state["z"] - state["w"],
        &[
            Constraint::ge(|state| state["x"], -10),
            Constraint::ge(|state| state["y"], 100),
            Constraint::ge(|state| state["w"], 100),
            Constraint::ge(|state| state["z"], 1000),
            Constraint::le(|s| s["x"] + s["y"] + s["z"] + s["w"], 2000),
        ],
    );
    let base = State::new(&[("x", 0), ("y", 0), ("z", 0), ("w", 0)]);
    let out = c.solve(base);
    println!("{:#?}", out);
}

pub fn main() {
    let _timer = Timer::new("city generation");
    make_constraints_set();
}
