
use crate::context::Context;
use crate::road::*;
use crate::math::*;
use crate::building::Building;
type Water =Road;
const WATER_WIDTH:f64 = 10.0;
#[allow(unused)]
#[derive(Clone,Copy,PartialEq)]
pub enum Direction{
    North, 
    South, 
    East,
    West,
}
impl Direction{
    #[allow(unused)]
    pub fn as_vec(&self)->Vector2{
        match *self{
            Self::North=>vec2(0.0, -1.0),
            Self::South=>vec2(0.0, 1.0),
            Self::East=>vec2(1.0, 0.0),
            Self::West=>vec2(-1.0,0.0)
        }
    }
}

#[allow(unused)]
pub enum WaterGenerationRequest{
    Coast{dir:Direction},
    River{dir:Direction},
    RiverToCoast{dir:Direction}
}

#[allow(unused)]
fn copy_and_move_water(base:&Water, mv:Vector2)->Water{
    let mut out = base.clone();
    for i in 0..out.points.len(){
        out.points[i]+=mv;
    }
    out
}
#[allow(unused)]
fn connect_points_into_water(start:Vector2, end:Vector2, context:&Context)->Water{
    let noise = NoiseGenerator1d::new(distance(&end, &start),250.0,4, context);
    let mut points = vec![start];
    let count = (distance(&start, &end)/10.0).ceil() as usize;
    let delta = (end-start)/(count as f64);
    let delta_x = rotate_vec2(&normalize(&delta), 90.0);
    let dl = length(&delta);
    let mut current = start;
    for i in 0..count+8{
        current = start+delta_x*noise.get_value(dl*i as f64)*100.0+delta*(i as f64);
        points.push(current);
    }
    Road::new(&points, WATER_WIDTH)

}
#[allow(unused)]
fn generate_river(dir:Direction, context:&Context)->Vec<Water>{
    let start = match dir{
        Direction::North=>vec2(context.width as f64/2.0, context.height as f64),
        Direction::South=>vec2(context.width as f64/2.0,0.0),
        Direction::East=>vec2(0.0, context.height as f64/2.0),
        Direction::West=>vec2(context.width as f64, context.height as f64/2.0),
    };
    let end = match dir{
        Direction::North=>vec2(context.width as f64/2.0, 0.0),
        Direction::South=>vec2(context.width as f64/2.0,context.height as f64),
        Direction::East=>vec2(context.width as f64, context.height as f64/2.0),
        Direction::West=>vec2(0.0, context.height as f64/2.0),
    };
    let mv = (match dir{
        Direction::North=>vec2(1.0, 0.0),
        Direction::South=>vec2(-1.0, 0.0),
        Direction::East=>vec2(0.0, 1.0),
        Direction::West=>vec2(0.0,-1.0),
    })*WATER_WIDTH;
    let s = connect_points_into_water(start, end, context);
    vec![copy_and_move_water(&s, -2.0*mv), copy_and_move_water(&s, -mv), s.clone(), copy_and_move_water(&s, mv), copy_and_move_water(&s, 2.0*mv)]
}

#[allow(unused)]
fn generate_coast(dir:Direction, context:&Context)->Vec<Water>{
    let delta = 0.25;
    let start = match dir{
        Direction::North=>vec2(-WATER_WIDTH, context.height as f64*(1.0-delta)),
        Direction::South=>vec2(-WATER_WIDTH ,context.height as f64 *delta),
        Direction::East=>vec2(context.width as f64*(1.0-delta), WATER_WIDTH),
        Direction::West=>vec2(context.width as f64*delta,WATER_WIDTH),
    };
    let end = match dir{
        Direction::North=>vec2(context.width as f64, context.height as f64*(1.0-delta)),
        Direction::South=>vec2(context.width as f64 ,context.height as f64 *delta),
        Direction::East=>vec2(context.width as f64*(1.0-delta), context.height as f64),
        Direction::West=>vec2(context.width as f64*delta, context.height as f64),
    };
    let mv = (match dir{
        Direction::North=>vec2(0.0, 1.0),
        Direction::South=>vec2(0.0, -1.0),
        Direction::East=>vec2(1.0, 0.0),
        Direction::West=>vec2(-1.0,0.0),
    })*WATER_WIDTH;
    let mut out = vec![];
    let count = {
        if dir == Direction::North || dir ==Direction::South{
            ((context.height as f64*delta)/WATER_WIDTH).ceil() as usize
        } else{
            ((context.width as f64*delta)/WATER_WIDTH).ceil() as usize
        }
    };
    let s = connect_points_into_water(start, end, context);
    for i in 0..count*2{
        out.push(copy_and_move_water(&s, mv*i as f64))
    }
    out
}

#[allow(unused)]
pub fn generate_water_ways(request:WaterGenerationRequest, context:&Context)->Vec<Water>{
    match request{
        WaterGenerationRequest::Coast{dir}=>generate_coast(dir, context),
        WaterGenerationRequest::River { dir }=>generate_river(dir, context),
        WaterGenerationRequest::RiverToCoast { dir }=>vec![generate_coast(dir,context), generate_river(dir,context )].into_iter().flatten().collect(),
    }
}

#[allow(unused)]
pub fn climate_change(buildings:&[Building],water:&[Water])->Vec<Building>{
    let mut out = vec![];
    for b in buildings{
        let mut hit = false;
        for w in water{
            for p in &(b.to_rect().as_array()){
                if w.distance_to(*p)<w.width{
                    hit = true;
                    break;
                }
            }
            if hit{
                break;
            }
        }
        if !hit{
            out.push(*b);
        }
    }
    out
}
