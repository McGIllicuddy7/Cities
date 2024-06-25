pub use nalgebra_glm::*;
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
    return v + start;
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
    return first && second;
}
#[allow(unused)]
pub fn dist_point_to_line(point: Vector2, start: Vector2, end: Vector2) -> f64 {
    let proj = project_vector2_line(point, start, end);
    let btwn = is_between_points(proj, start, end);
    return if btwn {
        distance(&point, &proj)
    } else {
        let d0 = distance(&point, &start);
        let d1 = distance(&point, &end);
        if d0 < d1 {
            d0
        } else {
            d1
        }
    };
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
    return out;
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
    return out;
}
pub struct Rectangle{
    pub v0:Vector2,
    pub v1:Vector2,
    pub v2:Vector2, 
    pub v3:Vector2,
}
pub fn rect(v0:Vector2, v1:Vector2, v2:Vector2, v3:Vector2)->Rectangle{
    return Rectangle{v0, v1, v2, v3};
}
impl Rectangle{
    #[allow(unused)]
    pub fn scale(&self, scale:f64)->Rectangle{
        let center = (self.v0+self.v1+self.v2+self.v3)/ 4 as f64;
        let dv0 = self.v0-center;
        let dv1 = self.v1-center;
        let dv2 = self.v2-center;
        let dv3 = self.v3-center;
        return Self{v0:center+dv0*scale, v1:center+dv1*scale, v2:center+dv2*scale, v3:center+dv3*scale};
    }
    #[allow(unused)]
    pub fn scale_mut(&mut self, scale:f64){
        *self = self.scale(scale);
    }
}