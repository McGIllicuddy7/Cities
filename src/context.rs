use crate::math::*;
pub struct Context {
    pub height: i32,
    pub width: i32,
    pub block_scale: f64,
    pub building_scale: f64,
    pub whole_block_buildings_percent: i32,
    rand_lock:std::sync::Mutex<()>,
}
impl Context {
    pub fn new(
        height: i32,
        width: i32,
        block_scale: f64,
        building_scale: f64,
        whole_block_buildings_percent: i32,
    ) -> Self {
        unsafe {
            raylib::ffi::SetRandomSeed(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs() as u32,
            );
        }
        let rand_lock = std::sync::Mutex::new(());
        Self {
            height,
            width,
            block_scale,
            building_scale,
            whole_block_buildings_percent,
            rand_lock,
        }
    }
    pub fn center(&self) -> Vector2 {
        vec2(self.width as f64 / 2.0, self.height as f64 / 2.0)
    }
    pub fn get_random_value(&self, minimum: i32, maximum: i32) -> i32 {
        let b = self.rand_lock.lock().unwrap();
        let v =  unsafe { raylib::ffi::GetRandomValue(minimum, maximum) };
        drop(b);
        v
    }
    pub fn get_random_float(&self)->f64{
        self.get_random_value(0, 100_000_000) as f64 /100_000_000 as f64
    }
    #[allow(unused)]
    pub fn get_random_vector(&self)->Vector2{
        let theta = self.get_random_value( 0,31415*2) as f64 / (10000) as f64;
        let r = self.get_random_float();
        vec2(theta.cos()*r, theta.sin()*r)
    }
}
impl Drop for Context {
    fn drop(&mut self) {}
}

