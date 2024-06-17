pub use nalgebra_glm::*;
pub type Vector2 = TVec2<f64>;
#[allow(unused)]
pub fn project_vector2_line(point:Vector2, start:Vector2, end:Vector2)->Vector2{
    if end == start{
        return end;
    }
    let s = point-start;
    let e = end-start;
    let delta = e.normalize();
    let v  = dot(&s, &delta)*delta;
    return v + start;
}
#[allow(unused)]
pub fn is_between_points(point:Vector2, start:Vector2, end:Vector2)->bool{
    let first = {
        let p = point-start;
        let e = end-start;
        let pn = p.normalize();
        let en = e.normalize();
        let dp = dot(&p, &pn);
        let de = dot(&e, &en);
        dp<de
    };
    let second = {
        let p = point-end ;
        let e = start-end;
        let pn = p.normalize();
        let en = e.normalize();
        let dp = dot(&p, &pn);
        let de = dot(&e, &en);
        dp<de
    };
    return first && second;
}
#[allow(unused)]
pub fn dist_point_to_line(point:Vector2, start:Vector2, end:Vector2)->f64{
    let proj = project_vector2_line(point, start, end);
    let btwn = is_between_points(proj, start, end);
    return if btwn{
        distance(&point, &proj)
    } else{
        let d0 = distance(&point, &start);
        let d1 = distance(&point, &end);
        if d0<d1 {d0} else{ d1}
    }
}
#[allow(unused)]
pub fn to_raylib_vec(v:Vector2)->raylib::ffi::Vector2{
    raylib::ffi::Vector2{x: v.x as f32, y: v.y as f32} 
}