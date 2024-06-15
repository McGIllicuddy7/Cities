use crate::math::*;
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
            rust_raylib::ffi::BeginMode3D(context.cam.clone());
            let mut point_buff = [vec2(0 as f32,0 as f32); 512]; 
            for i in 0..self.points.len(){
                point_buff[i].x = self.points[i].x as f32;
                point_buff[i].y = self.points[i].y as f32;
            }
            let count:i32 = self.points.len() as i32;
            let idx = rust_raylib::ffi::GetShaderLocation(context.street_shader,"locations\0".as_ptr() as * const i8);
            rust_raylib::ffi::SetShaderValueV(context.street_shader.clone(),idx, point_buff.as_ptr() as *const c_void, rust_raylib::ffi::ShaderUniformDataType::Vec2, 512);
            let count_idx = rust_raylib::ffi::GetShaderLocation(context.street_shader.clone(),"count\0".as_ptr() as * const i8);
            rust_raylib::ffi::SetShaderValue(context.street_shader.clone(), count_idx, , );
            rust_raylib::ffi::EndMode3D();
        }
    }
}