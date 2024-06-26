use crate::context::Context;
use crate::math::*;
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
        let blocks = building::generate_blocks(rings.as_slice());
        let buildings = {
            let mut tmp = vec![];
            for b in blocks {
                for c in b.buildings {
                    tmp.push(c);
                }
            }
            tmp
        };
        Self {
            roads,
            buildings,
        }
        .scale(context, 2.0)
    }
    pub fn draw(&self, context: &Context) {
        for r in &self.roads {
            r.draw(context);
        }
        for b in &self.buildings {
            b.draw(context);
        }
    }
    pub fn scale(&self, context: &Context, scaler: f64) -> Self {
        let mut out = Self {
            roads: vec![],
            buildings: vec![],
        };
        let center = vec2(
            context.width as f64 / 2_f64,
            context.height as f64 / 2_f64,
        );
        for r in &self.roads {
            let mut tmp = road::Road { points: vec![] };
            for v in &r.points {
                let dv = v - center;
                let nw = dv * scaler + center;
                tmp.points.push(nw);
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
