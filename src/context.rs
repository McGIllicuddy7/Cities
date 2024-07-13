use crate::math::*;
#[derive(Clone)]
pub struct Context {
    pub height: i32,
    pub width: i32,
    pub block_scale: f64,
    pub building_scale: f64,
    pub whole_block_buildings_percent: f64,
    pub small_width: f64,
    pub medium_width: f64,
    pub large_width: f64,
}
impl Context {
    pub fn new(
        height: i32,
        width: i32,
        block_scale: f64,
        building_scale: f64,
        whole_block_buildings_percent: f64,
        small_width: f64,
        medium_width: f64,
        large_width: f64,
    ) -> Self {
        Self {
            height,
            width,
            block_scale,
            building_scale,
            whole_block_buildings_percent,
            small_width,
            medium_width,
            large_width,
        }
    }
    pub fn center(&self) -> Vector2 {
        vec2(self.width as f64 / 2.0, self.height as f64 / 2.0)
    }
    pub fn get_random_value(&self, minimum: i32, maximum: i32) -> i32 {
        (rand::random::<isize>() % (maximum as isize - minimum as isize) + minimum as isize) as i32
    }
    pub fn get_random_float(&self) -> f64 {
        self.get_random_value(0, 100_000) as f64 / 100_000 as f64
    }
    #[allow(unused)]
    pub fn get_random_vector(&self) -> Vector2 {
        let theta = self.get_random_value(0, 31415 * 2) as f64 / (10000) as f64;
        let r = self.get_random_float();
        vec2(theta.cos() * r, theta.sin() * r)
    }
}
impl Drop for Context {
    fn drop(&mut self) {}
}
