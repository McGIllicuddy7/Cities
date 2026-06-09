use rayon::vec;

use crate::buildings::PointCollection;

pub struct Building {
    pub points: PointCollection,
    pub center_x: i32,
    pub center_y: i32,
    pub is_residence: bool,
}
