use crate::math::*;
pub struct Context{
    pub height:i32,
    pub width:i32,
    pub block_scale:f64, 
    pub building_scale:f64, 
    pub whole_block_buildings_percent:i32,
}
impl Context{

    pub fn new(height:i32, width:i32, block_scale:f64, building_scale:f64, whole_block_buildings_percent:i32)->Self{
        unsafe{
            raylib::ffi::SetRandomSeed(std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs() as u32);
        }
        Self{height, width, block_scale, building_scale, whole_block_buildings_percent}
    }
    pub fn center(&self)->Vector2{
        vec2(self.width as f64 /2.0, self.height as f64/2.0)
    }
}
impl Drop for Context{
    fn drop(&mut self) {

    }
}