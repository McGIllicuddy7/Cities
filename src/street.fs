#version 330

// Input vertex attributes
in vec3 vertexPosition;
in vec2 vertexTexCoord;
in vec3 vertexNormal;
in vec4 vertexColor;

// Input uniform values
uniform mat4 mvp;

// Output vertex attributes (to fragment shader)
out vec2 fragTexCoord;
out vec4 fragColor;

// NOTE: Add here your custom variables
in vec2 locations[500];
in int locations_count;
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
    let p = point-start;
    let e = end-start;
    let pn = p.normalize();
    let en = e.normalize();
    let dp = dot(&p, &pn);
    let de = dot(&e, &en);
    return dp<de;
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
void main()
{
    // Send vertex attributes to fragment shader
    fragTexCoord = vertexTexCoord;
    fragColor = vertexColor;

    // Calculate final vertex position
    gl_Position = mvp*vec4(vertexPosition, 1.0);
}