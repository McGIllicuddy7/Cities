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
    let v  = dot(&s, &e)*delta;
    return v + start;
}
#[allow(unused)]
pub fn is_between_points(point:Vector2, start:Vector2, end:Vector2)->bool{
    let p = point-start;
    let e = end-start;
    let pn = p.normalize();
    let en = e.normalize();
    if pn != en{
        return false;
    }
    let dp = dot(&p, &pn);
    let de = dot(&e, &en);
    return dp<de;
}
#[allow(unused)]
pub fn dist_point_to_line(point:Vector2, start:Vector2, end:Vector2)->f64{
    let proj = project_vector2_line(point, start, end);
    let btwn = is_between_points(point, start, end);
    return if(btwn){
        distance(&point, &proj)
    } else{
        let d0 = distance(&point, &start);
        let d1 = distance(&point, &end);
        if d0<d1 {d0} else{ d1}
    }
}
