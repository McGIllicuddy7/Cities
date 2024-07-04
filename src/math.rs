

pub use nalgebra_glm::*;

use crate::context::*;
pub type Vector2 = TVec2<f64>;
#[allow(unused)]
pub fn project_vector2_line(point: Vector2, start: Vector2, end: Vector2) -> Vector2 {
    if end == start {
        return end;
    }
    let s = point - start;
    let e = end - start;
    let delta = e.normalize();
    let v = dot(&s, &delta) * delta;
    v + start
}
#[allow(unused)]
pub fn is_between_points(point: Vector2, start: Vector2, end: Vector2) -> bool {
    let first = {
        let p = point - start;
        let e = end - start;
        let pn = p.normalize();
        let en = e.normalize();
        let dp = dot(&p, &pn);
        let de = dot(&e, &en);
        dp < de
    };
    let second = {
        let p = point - end;
        let e = start - end;
        let pn = p.normalize();
        let en = e.normalize();
        let dp = dot(&p, &pn);
        let de = dot(&e, &en);
        de < dp
    };
     first && second
}
#[allow(unused)]
pub fn dist_point_to_line(point: Vector2, start: Vector2, end: Vector2) -> f64 {
    let proj = project_vector2_line(point, start, end);
    let btwn = is_between_points(proj, start, end);
    if btwn {
        distance(&point, &proj)
    } else {
        let d0 = distance(&point, &start);
        let d1 = distance(&point, &end);
        if d0 < d1 {
            d0
        } else {
            d1
        }
    }
}
#[allow(unused)]
pub fn to_raylib_vec(v: Vector2) -> raylib::ffi::Vector2 {
    raylib::ffi::Vector2 {
        x: v.x as f32,
        y: v.y as f32,
    }
}
#[allow(unused)]
pub fn gradient(v:&[Vector2], point:Vector2)->Vector2{
    let mut out = vec2(0.0, 0.0);
    for p in v{
        let delta = p-point;
        let grad = delta/length2(&delta);
        out += grad;
    }
    out
}
#[allow(unused)]
pub fn gradient_clamped(v:&[Vector2], point:Vector2, max_radius:f64)->Vector2{
    let mut out = vec2(0.0, 0.0);
    for p in v{
        if distance(p, &point)<max_radius{
            let delta = p-point;
            let grad = delta/(length2(&delta));
            out += grad;
        }
    }
    out
}
#[allow(unused)]
pub fn int_power<T>(base:T, other:usize)->T
where T: std::ops::MulAssign+Copy+From<i32>
{
    let mut out = T::from(1);
    for i in 0..other{
        out *= base;
    }
    out
}
pub struct Rectangle{
    pub v0:Vector2,
    pub v1:Vector2,
    pub v2:Vector2, 
    pub v3:Vector2,
}
pub fn rect(v0:Vector2, v1:Vector2, v2:Vector2, v3:Vector2)->Rectangle{
    Rectangle{v0, v1, v2, v3}
}
impl Rectangle{
    #[allow(unused)]
    pub fn scale(&self, scale:f64)->Rectangle{
        let center = (self.v0+self.v1+self.v2+self.v3)/ 4_f64;
        let dv0 = self.v0-center;
        let dv1 = self.v1-center;
        let dv2 = self.v2-center;
        let dv3 = self.v3-center;
        Self{v0:center+dv0*scale, v1:center+dv1*scale, v2:center+dv2*scale, v3:center+dv3*scale}
    }
    #[allow(unused)]
    pub fn scale_mut(&mut self, scale:f64){
        *self = self.scale(scale);
    }
    #[allow(unused)]
    pub fn from(v:[Vector2;4])->Self{
         Self { v0: v[0], v1: v[1],v2:v[2] ,v3:v[3] }
    }
    #[allow(unused)]
    pub fn into(&self)->[Vector2;4]{
        [self.v0,self.v1,self.v2,self.v3]
    }
}
#[allow(unused)]
struct NoiseOctave1d{
    pub points:Vec<f64>,
    pub point_dist:f64
}
#[allow(unused)]
pub struct NoiseGenerator1d{
    octaves:Vec<NoiseOctave1d>,
    norm:f64
}
impl NoiseOctave1d{
    pub fn new(length:f64, point_dist:f64,context:&Context)->Self{
        let p0 = context.get_random_float();
        let mut points = vec![];
        let count = (length/point_dist).floor() as usize;
        for _ in 1..count{
            points.push(context.get_random_float());
        }
        points.push(p0);
        Self{points, point_dist}
    }
    pub fn get_value(&self,location:f64)->f64{
        let a = (location/self.point_dist).floor();
        let b = (location/self.point_dist).ceil();
        let l_val = location/self.point_dist-a;
        let a_ind = a as usize % self.points.len();
        let b_ind = b as usize % self.points.len();
        self.points[a_ind]*(1.0-l_val)+self.points[b_ind]*l_val
    }
}
impl NoiseGenerator1d{
    #[allow(unused)]
    pub fn new(length:f64, point_dist:f64, octaves_count:usize, context:&Context)->Self{
        let octaves = (0..octaves_count).
        map(|i| NoiseOctave1d::new(length, point_dist/(int_power(2, i) as f64), context)).
        collect();
        let norm = (0..octaves_count).map(|i| int_power(0.5, i)).sum();
        return Self{octaves, norm};
    }
    #[allow(unused)]
    pub fn get_value(&self,location:f64)->f64{
        ( 0..self.octaves.len()).map(|i| self.octaves[i].get_value(location)*int_power(0.5, i)/self.norm).sum()
    }
}
pub fn interpolate(a0:f64, a1:f64, w:f64)->f64 {
    (a1 - a0) * w + a0
}
#[allow(unused)]
struct NoiseOctave2d{
    points:Vec<Vec<Vector2>>,
    scale_divisor:f64,
}
//based on https://en.wikipedia.org/wiki/Perlin_noise
impl NoiseOctave2d{
    pub fn new(context:&Context, scale_divisor:f64)->Self{
        let mut points = vec![];
        for _ in 0..256{
            let mut tmp = vec![];
            for _ in 0..256{
                tmp.push(context.get_random_vector());
            }
            points.push(tmp);
        }
        Self{points, scale_divisor}
    }
    fn random_gradient(&self, x:i32, y:i32)->Vector2{
        return self.points[y as usize % self.points.len()][x as usize% self.points.len()];
    }
    fn dot_grid_gradient(&self,ix:i32, iy:i32, x:f64, y:f64)->f64 {
        let gradient = self.random_gradient(ix, iy);
        let dx = x - ix as f64;
        let dy = y - iy as f64;
        dx*gradient.x + dy*gradient.y
    }
    
    #[allow(unused)]
    pub fn perlin(&self,x0:f64, y0:f64)->f64 {
        let x = x0/16.0;
        let y = y0/16.0;
        let x0 = x.floor() as i32;
        let x1 = x0 + 1;
        let y0 = y.floor() as i32;
        let y1 = y0 + 1;
        let sx = x as f64 - x0 as f64;
        let  sy = y as f64 - y0 as f64;
        let n0 = self.dot_grid_gradient(x0, y0, x, y);
        let n1 = self.dot_grid_gradient(x1, y0, x, y);
        let ix0 = interpolate(n0, n1, sx);
        let n0 = self.dot_grid_gradient(x0, y1, x, y);
        let n1 = self.dot_grid_gradient(x1, y1, x, y);
        let ix1 = interpolate(n0, n1, sx);
        let value = interpolate(ix0, ix1, sy);
        value
    }
}
#[allow(unused)]
pub struct NoiseGenerator2d{
    octaves:Vec<NoiseOctave2d>
}
impl NoiseGenerator2d{
    pub fn new(depth:usize, scale_divisor:f64,context:&Context)->Self{
        let mut octaves = vec![];
        for _ in 0..depth{
            octaves.push(NoiseOctave2d::new(context, scale_divisor));
        }
        Self { octaves }
    }
    pub fn perlin(&self, v:Vector2)->f64{
        let mut mlt = 1 as f64;
        let mut out = 0.0;
        let mut div = 1.0;
        let mut scaler = 1.0;
        for i in 0..self.octaves.len(){
            div += mlt;
            out += self.octaves[i].perlin(v.x/scaler, v.y*scaler)*mlt;
            mlt *= 3.0/4.0;
            scaler *= 2.0;
        }
        out /div
    }
}