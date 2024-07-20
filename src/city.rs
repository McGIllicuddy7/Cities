use crate::building::{filter_buildings, purge_degenerates};
use crate::context::Context;
use crate::math::*;
use crate::prof_frame;
use crate::road::Road;
use crate::water::{climate_change, generate_water_ways, Direction, WaterGenerationRequest};
use crate::{building, road};
#[allow(unused)]
pub struct City {
    pub roads: Vec<Road>,
    pub buildings: Vec<building::Building>,
    pub water: Vec<Road>,
}

impl City {
    #[allow(unused)]
    pub fn new(scaler: f64, context: &Context) -> Self {
        prof_frame!("City::new()");
        let radius = 510.0 * scaler * 2.5_f64.sqrt();
        let scale = 1.0 / scaler;
        let rings = road::generate_ring_system(radius, context);
        let roads = road::collect_rings_to_roads(&rings);
        let blocks = building::generate_blocks(rings, context);
        let buildings = {
            let mut tmp = vec![];
            for b in blocks {
                for c in b.buildings {
                    tmp.push(c);
                }
            }
            tmp
        };
        let buildings = filter_buildings(buildings.as_slice(), scaler, context);
        let buildings = purge_degenerates(buildings.as_slice());
        let mut out = Self {
            roads,
            buildings,
            water: vec![],
        }
        .scale(context, scale * 1.0);
        let water = generate_water_ways(
            WaterGenerationRequest::Coast {
                dir: Direction::East,
            },
            context,
        );
        out.buildings = climate_change(&out.buildings, &water);
        out.water = water;
        out
    }
    pub unsafe fn draw(&self, context: &Context) {
        for r in &self.roads {
            r.draw(context);
        }
        for r in &self.water {
            r.draw_as_water(context);
        }
        for b in &self.buildings {
            b.draw(context);
        }
    }
    pub fn scale(&self, context: &Context, scaler: f64) -> Self {
        prof_frame!("City::scale()");
        let mut out = Self {
            roads: vec![],
            buildings: vec![],
            water: vec![],
        };
        let center = vec2(context.width as f64 / 2_f64, context.height as f64 / 2_f64);
        for r in &self.roads {
            let mut tmp = Road::new(&[], r.width * scaler);
            let mut i = 0;
            for v in &r.points {
                let dv = v - center;
                let nw = dv * scaler + center;
                tmp.points.push(nw);
                tmp.point_index_map
                    .insert((nw.x.round() as i64, nw.y.round() as i64), i);
                i += 1;
            }
            out.roads.push(tmp);
        }
        for b in &self.buildings {
            let d0 = b.p0 - center;
            let d1 = b.p1 - center;
            let d2 = b.p2 - center;
            let d3 = b.p3 - center;
            let nw = building::Building {
                p0: d0 * scaler + center,
                p1: d1 * scaler + center,
                p2: d2 * scaler + center,
                p3: d3 * scaler + center,
            };
            out.buildings.push(nw);
        }
        out
    }
}
