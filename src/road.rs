

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
            raylib::ffi::BeginMode3D(context.cam.clone());
            let mut point_buff = [vec2(0 as f32,0 as f32); 512]; 
            for i in 0..self.points.len(){
                point_buff[i].x = self.points[i].x as f32;
                point_buff[i].y = self.points[i].y as f32;
            }
           // let count:i32 = self.points.len() as i32;
           // let idx = raylib::ffi::GetShaderLocation(context.street_shader.clone(),"locations\0".as_ptr() as * const i8);
           // raylib::ffi::SetShaderValueV(context.street_shader.clone(),idx, point_buff.as_ptr() as *const c_void, 1 as i32, 512);
           // let count_idx = raylib::ffi::GetShaderLocation(context.street_shader.clone(),"count\0".as_ptr() as * const i8);
           // raylib::ffi::SetShaderValue(context.street_shader.clone(), count_idx,std::ptr::from_ref(&count) as *const c_void,4 as i32);
            raylib::ffi::DrawMesh(context.map_mesh.clone(), context.street_mat.clone(), context.street_location.clone());
            raylib::ffi::EndMode3D();
        }
    }
    #[allow(unused)]
    pub fn draw_debug(&self, context:&crate::context::Context){
        unsafe{
            for y in 0..context.height{
                for x in 0..context.width{
                    let loc:Vector2 = vec2(x as f64, y as f64);
                    if self.distance_to(loc) <10 as f64{
                        raylib::ffi::DrawPixel(x,y, raylib::color::Color::BLUEVIOLET.into());
                    }
                }
            }
        }
    }
}