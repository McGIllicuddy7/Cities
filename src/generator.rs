use super::array2d::Array2d;
use rand::random;
use raylib::math::Vector2;
use std::f32::consts::TAU;
use std::ops::{Index, IndexMut};
#[derive(Debug, Clone)]
pub struct VectorField {
    values: Array2d<Vector2>,
}

impl Index<usize> for VectorField {
    type Output = [Vector2];
    fn index(&self, index: usize) -> &Self::Output {
        self.values.get(index)
    }
}

impl IndexMut<usize> for VectorField {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.values.get_mut(index)
    }
}

impl VectorField {
    pub fn new_random(width: usize, height: usize) -> Self {
        let mut out = Vec::new();
        out.reserve_exact(width * height);
        for _ in 0..height {
            for _ in 0..width {
                out.push(random_vector());
            }
        }
        Self {
            values: Array2d::from_vec(out, width, height),
        }
    }
    pub fn new_zero(width: usize, height: usize) -> Self {
        let mut out = Vec::new();
        out.reserve_exact(width * height);
        for _ in 0..height {
            for _ in 0..width {
                out.push(Vector2::zero());
            }
        }
        Self {
            values: Array2d::from_vec(out, width, height),
        }
    }
}

pub fn random_vector() -> Vector2 {
    let theta_b: i32 = random();
    let theta = ((theta_b % 10000) as f32 / (10000.)) * TAU;
    Vector2 {
        x: theta.cos(),
        y: theta.sin(),
    }
}
