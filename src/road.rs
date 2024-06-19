

use crate::context::Context;
use raylib::ffi::GetRandomValue;

use crate::math::*;
#[allow(unused)]
use std::ffi::c_void;
#[allow(unused)]
pub struct Road{
    pub points:Vec<Vector2>
}

impl Road{
    #[allow(unused)]
    pub fn new(points:&[Vector2])->Self{
        let mut v = vec![];
        for p in points{
            v.push(*p);
        }
        return Road{points:v};
    }
    #[allow(unused)]
    pub fn distance_to(&self, point:Vector2)->f64{
        let mut min = distance(&self.points[0], &point);
        for i in 0..self.points.len()-1{
            let dist = dist_point_to_line(point, self.points[i], self.points[i+1]);
            if(dist<min){
                min = dist;
            }
        }
        return min;
    }

    #[allow(unused)]
    pub fn draw(&self, context:&crate::context::Context){
        unsafe{
            for i in 0..self.points.len(){
                let s= self.points[i];
                let e = self.points[(i+1)%self.points.len()];
                raylib::ffi::DrawLineEx(to_raylib_vec(s), to_raylib_vec(e), 4 as f32, raylib::color::Color::BLACK.into())
            }
        }
    }
}
#[allow(unused)]
pub fn generate_road(start:Vector2, context:&Context)->Road{
    let mut points:Vec<Vector2> = vec![start];
    let mut most_recent = start;
    let max = 0;
    let mut arc_length = 0.0 ;
    loop {
        
    }
    return Road { points: points };
}