use std::f32::consts::PI;

use rand::random;
use raylib::math::Vector2;
use raylib::prelude::Color;
use raylib::texture::Image;

use crate::array2d::Array2d;
use crate::col::Collider2D;

#[derive(Debug, Clone)]
pub struct VectorField {
    values: Array2d<Vector2>,
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

    pub fn get_average_direction_at_point(&self, x: i32, y: i32) -> Vector2 {
        let mut avg = Vector2::zero();
        let mut count = 0.0;
        for i in -1..=1 {
            for j in -1..=1 {
                let dy = y + i;
                let dx = x + j;
                if dx >= 0 && dy >= 0 && dx < self.width() && dy < self.height() {
                    count += 1.;
                    avg += self.get(x, y);
                }
            }
        }
        avg / count
    }

    pub fn get_curl_at_point(&self, x: i32, y: i32) -> f32 {
        let dx1 = self.get(x + 1, y) - self.get(x, y);
        let dy1 = self.get(x, y + 1) - self.get(x, y);
        let dx2 = self.get(x, y) - self.get(x - 1, y);
        let dy2 = self.get(x, y) - self.get(x, y - 1);
        let dx = (dx1 + dx2).y / 2.0;
        let dy = (dy1 + dy2).x / 2.0;
        dx - dy
    }

    pub fn get_div_at_point(&self, x: i32, y: i32) -> f32 {
        let dx1 = self.get(x + 1, y) - self.get(x, y);
        let dy1 = self.get(x, y + 1) - self.get(x, y);
        let dx2 = self.get(x, y) - self.get(x - 1, y);
        let dy2 = self.get(x, y) - self.get(x, y - 1);
        let dx = (dx1 + dx2) / 2.0;
        let dy = (dy1 + dy2) / 2.0;
        dx.x + dy.y
    }

    pub fn width(&self) -> i32 {
        self.values.width() as i32
    }

    pub fn height(&self) -> i32 {
        self.values.height() as i32
    }

    pub fn get(&self, x: i32, y: i32) -> Vector2 {
        if x < 0 || y < 0 || x >= self.width() || y >= self.height() {
            return Vector2::zero();
        }
        self.values[y as usize][x as usize]
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> &mut Vector2 {
        &mut self.values[y as usize][x as usize]
    }

    pub fn set(&mut self, value: Vector2, x: i32, y: i32) {
        self.values[y as usize][x as usize] = value;
    }

    pub fn noise(&mut self, rot: f32) {
        let base = Image::gen_image_color(10, 10, Color::WHITE);
        let mut noise = base.gen_image_perlin_noise(
            self.width(),
            self.height(),
            random::<i32>().abs() % self.width(),
            random::<i32>().abs() % self.height(),
            32.,
        );
        for i in 0..self.height() {
            for j in 0..self.width() {
                let amnt = noise.get_color(j, i).r as f32 * 0.1
                    + (random::<i32>().abs() % 1000) as f32 / 1000.0 * 0.1;
                let p = self.get(j, i).rotated(amnt * rot);

                *self.get_mut(j, i) = p;
            }
        }
    }
}

pub fn random_vector() -> Vector2 {
    let theta_b: i32 = random();
    let theta = ((theta_b % 10000) as f32 / (10000.)) * std::f32::consts::TAU;
    Vector2 {
        x: theta.cos(),
        y: theta.sin(),
    }
}

pub fn render_vector_field(f: &VectorField, name: &str) {
    let mut out =
        raylib::prelude::Image::gen_image_color(f.width() * 10, f.height() * 10, Color::WHITE);
    for i in 0..f.height() {
        for j in 0..f.width() {
            let start = Vector2::new(j as f32, i as f32) * 10.;
            let v = f.get(j, i);
            if v.length_sqr() < 0.1 {
                out.draw_circle_v(start, 4, Color::RED);
            } else {
                let end = start + v * 8.;
                let delt1 = v.rotated(0.5);
                let delta1 = if delt1.dot(v) > 0. { -delt1 } else { delt1 };
                let delt2 = v.rotated(-0.5);
                let delta2 = if delt2.dot(v) > 0. { -delt2 } else { delt2 };
                out.draw_line_v(start, end, Color::RED);
                out.draw_line_v(end, end + delta1 * 4., Color::RED);
                out.draw_line_v(end, end + delta2 * 4., Color::RED);
            }
        }
    }
    out.export_image(name);
}

pub fn render_vector_field_div(f: &VectorField, name: &str) {
    let mut out =
        raylib::prelude::Image::gen_image_color(f.width() * 10, f.height() * 10, Color::WHITE);
    for i in 0..f.height() {
        for j in 0..f.width() {
            let start = Vector2::new(j as f32, i as f32) * 10.;
            let v = f.get(j, i);
            let div = f.get_div_at_point(j, i);
            let curl = f.get_curl_at_point(j, i);
            out.draw_rectangle(
                j * 10,
                i * 10,
                10,
                10,
                Color::new(
                    0,
                    ((div + 1.0) * 128.) as u8,
                    ((curl + 1.) * 128.) as u8,
                    255,
                ),
            );
            if v.length_sqr() < 0.1 {
                out.draw_circle_v(start, 4, Color::RED);
            } else {
                let end = start + v * 8.;
                let delt1 = v.rotated(0.5);
                let delta1 = if delt1.dot(v) > 0. { -delt1 } else { delt1 };
                let delt2 = v.rotated(-0.5);
                let delta2 = if delt2.dot(v) > 0. { -delt2 } else { delt2 };
                out.draw_line_v(start, end, Color::RED);
                out.draw_line_v(end, end + delta1 * 4., Color::RED);
                out.draw_line_v(end, end + delta2 * 4., Color::RED);
            }
        }
    }
    out.export_image(name);
}

pub fn render_field_div(f: &VectorField, name: &str) {
    let mut out = raylib::prelude::Image::gen_image_color(f.width(), f.height(), Color::WHITE);
    for i in 0..f.height() {
        for j in 0..f.width() {
            let div = f.get_div_at_point(j, i);
            if i > 0 && j > 0 && i < f.height() - 1 && j < f.width() - 1 {
                println!("x:{}, y:{}, div:{}", j, i, div);
            }
            let curl = f.get_curl_at_point(j, i);
            out.draw_pixel(
                j,
                i,
                Color::new(
                    0,
                    ((div + 1.0) * 128.) as u8,
                    ((curl + 1.) * 128.) as u8,
                    255,
                ),
            );
        }
    }
    for i in 0..f.height() {
        for j in 0..f.width() {
            if f.get(j, i).length() < 0.1 {
                out.draw_pixel(j, i, Color::RED);
            }
        }
    }
    out.export_image(name);
}

pub fn generate_voronoish(width: i32, height: i32, divisor: i32) -> VectorField {
    let mut f = VectorField::new_zero(width as usize, height as usize);
    let mut points = Vec::new();
    for i in 0..height / divisor {
        for j in 0..width / divisor {
            let delta_y = 5;
            let delta_x = 5;
            let sx: i32 = random();
            let sy: i32 = random();
            let scale_x = (sx % 10_000) as f32 / 20_000.;
            let scale_y = (sy % 10_000) as f32 / 20_000.;
            let x = j * delta_x + (scale_x as f32 * delta_x as f32) as i32;
            let y = i * delta_y + (scale_y as f32 * delta_y as f32) as i32;
            points.push(Vector2::new(x as f32, y as f32));
        }
    }
    for y in 0..f.height() {
        for x in 0..f.width() {
            let p = Vector2::new(x as f32, y as f32);
            let mut dir = Vector2::zero();
            for i in &points {
                let dist = p.distance_to(*i);
                if dist == 0.0 {
                    dir = Vector2::zero();
                    break;
                } else if dist > 20. {
                    continue;
                } else {
                    let delta = (*i - p) / (dist);
                    dir += delta;
                }
            }
            *f.get_mut(x, y) = if dir.length_sqr() > 0.1 {
                dir.normalized()
            } else {
                Vector2::zero()
            };
        }
    }
    f
}

pub fn generate_voronoish_count(width: i32, height: i32, count: i32) -> VectorField {
    let mut f = VectorField::new_zero(width as usize, height as usize);
    let mut points = Vec::new();
    for _ in 0..count {
        let x: i32 = random::<i32>().abs();
        let y: i32 = random::<i32>().abs();
        let v = Vector2::new((x % width) as f32, (y % height) as f32);
        points.push(v);
    }
    for y in 0..f.height() {
        for x in 0..f.width() {
            let p = Vector2::new(x as f32, y as f32);
            let mut dir = Vector2::zero();
            for i in &points {
                let mut dist = p.distance_to(*i);
                if dist == 0.0 {
                    dir = Vector2::zero();
                } else {
                    if dist > 100.0 {
                        continue;
                    }
                    if dist < 0.1 {
                        dist = 0.1;
                    }
                    let delta = (*i - p).normalized() / (dist / 2.);
                    dir += delta;
                }
            }
            *f.get_mut(x, y) = if dir.length_sqr() > 0. {
                dir.normalized()
            } else {
                Vector2::zero()
            };
        }
    }
    f.noise(0.05);
    f
}
#[derive(Debug, Clone)]
pub struct Building {
    pub col: Collider2D,
}

#[derive(Debug)]
pub struct City {
    pub buildings: Vec<Building>,
    pub occupied: Array2d<bool>,
    pub field: VectorField,
    pub width: i32,
    pub height: i32,
    pub roads: Vec<Road>,
}

impl City {
    pub fn new() -> Self {
        Self {
            buildings: Vec::new(),
            field: VectorField::new_zero(0, 0),
            width: 0,
            height: 0,
            roads: Vec::new(),
            occupied: Array2d::from_array([[false; 10]; 10]),
        }
    }

    pub fn collides_with_building(&self, col: Collider2D, _rat: f32) -> bool {
        for i in &self.buildings {
            if i.col.pos.distance_to(col.pos) < (col.height + col.width) / 2. {
                return true;
            }
        }
        let rad = if col.height / 2. > col.width / 2. {
            col.height / 2.
        } else {
            col.width / 2.
        };
        {
            for i in &self.roads {
                if i.within_radius(col.pos, rad as i32) {
                    return true;
                }
            }
        }
        false
    }

    pub fn dist_to_road(&self, col: Collider2D) -> f32 {
        let mut out = 10000.;
        for i in &self.roads {
            let tmp = i.distance_to(col.pos);
            if tmp < out {
                out = tmp;
            }
        }
        out
    }
}

#[derive(Debug)]
pub struct Road {
    pub points: Vec<Vector2>,
    pub rad: i32,
}
impl Road {
    pub fn within_radius(&self, point: Vector2, rad: i32) -> bool {
        if self.points.len() < 2 {
            return false;
        }
        for i in 0..self.points.len() - 1 {
            let p1 = self.points[i];
            let p2 = self.points[i + 1];
            let p3 = (p1 + p2) / 2.;
            let p4 = (p1 + p3) / 2.;
            let p5 = (p2 + p3) / 2.;
            if p1.distance_to(point) < rad as f32 + 0.5 {
                return true;
            }
            if p2.distance_to(point) < rad as f32 + 0.5 {
                return true;
            }
            if p3.distance_to(point) < rad as f32 + 0.5 {
                return true;
            }
            if p4.distance_to(point) < rad as f32 + 0.5 {
                return true;
            }
            if p5.distance_to(point) < rad as f32 + 0.5 {
                return true;
            }
        }
        false
    }
    pub fn distance_to(&self, point: Vector2) -> f32 {
        let mut out = 1000.0;
        for i in 0..self.points.len() - 1 {
            let p1 = self.points[i];
            let p2 = self.points[i + 1];
            let p3 = (p1 + p2) / 2.;
            let p4 = (p1 + p3) / 2.;
            let p5 = (p2 + p3) / 2.;
            let t1 = p1.distance_to(point);
            if t1 < out {
                out = t1;
            }
            let t2 = p2.distance_to(point);
            if t2 < out {
                out = t2;
            }
            let t3 = p3.distance_to(point);
            if t3 < out {
                out = t3;
            }
            let t4 = p4.distance_to(point);
            if t4 < out {
                out = t4;
            }
            let t5 = p5.distance_to(point);
            if t5 < out {
                out = t5;
            }
        }
        out
    }
}

pub fn generate_roads(field: &VectorField, scale: f32) -> (Vec<Road>, Array2d<bool>) {
    let mut hits = Array2d::from_vec(
        (0..(field.height() * field.width()))
            .map(|_| false)
            .collect(),
        field.width() as usize,
        field.height() as usize,
    );
    let mut out = Vec::new();
    let mut starting_points = {
        let mut out = Vec::new();
        let delt = 8;
        for x in 0..field.width() / delt {
            for y in 0..field.height() / delt {
                out.push((
                    x * delt + random::<i32>().abs() % delt,
                    y * delt + random::<i32>().abs() % delt,
                ));
            }
        }
        for x in 0..0 {
            for i in out.clone() {
                out.push(i);
            }
        }
        out
    };
    println!("gen done");
    while !starting_points.is_empty() {
        let (mut x, mut y) = rand_select(&mut starting_points);
        let start = (x, y);
        let mut try_count = 0;
        let prev = hits.clone();
        loop {
            x = start.0;
            y = start.1;
            let par: bool = if random::<i32>().abs() % 100 < 50 {
                true
            } else {
                false
            };
            *hits.get_at_mut(x as usize, y as usize) = true;
            let mut c: Vec<Vector2> = Vec::new();
            let v0 = Vector2::new(x as f32, y as f32) * scale;
            c.push(v0);
            let mut idx = 0;
            let mut last_was_hit = false;
            let mut hit_count = 0;
            while idx < field.width() {
                if last_was_hit {
                    if *hits.get_at(x as usize, y as usize) {
                        if hit_count > 32 {
                            break;
                        } else {
                            hit_count += 1;
                        }
                    }
                } else {
                    if *hits.get_at(x as usize, y as usize) {
                        last_was_hit = true;
                        hit_count = 0;
                    } else {
                        last_was_hit = false;
                        hit_count = 0;
                    }
                }
                *hits.get_at_mut(x as usize, y as usize) = true;
                let v = field.get(x, y);
                let mut nearest = (-1, -1);
                let mut nearest_amnt = 0.0;
                let mut hit = false;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        if dy + y < 0
                            || dx + x < 0
                            || dx + x >= field.width()
                            || dy + y >= field.height()
                        {
                            continue;
                        }
                        let value = field.get(x + dx, y + dy);
                        if par {
                            if value.dot(v) > nearest_amnt
                                && value.dot(Vector2::new(dx as f32, dy as f32)) > 0.8
                            {
                                nearest = (dx, dy);
                                hit = true;
                                nearest_amnt = value.dot(v);
                            }
                        } else {
                            if value.rotated(PI / 2.).dot(v.rotated(PI / 2.)) > nearest_amnt
                                && value
                                    .rotated(PI / 2.)
                                    .dot(Vector2::new(dx as f32, dy as f32))
                                    > 0.8
                            {
                                nearest = (dx, dy);
                                hit = true;
                                nearest_amnt = value.rotated(PI / 2.).dot(v.rotated(PI / 2.));
                            }
                        }
                    }
                }
                if !hit {
                    break;
                }
                x += nearest.0;
                y += nearest.1;
                let v = Vector2::new(x as f32, y as f32) * scale;
                c.push(v);
                idx += 1;
                if c.len() > (field.width()) as usize {
                    break;
                }
            }
            if (c.len() > 8 && !par) || c.len() > 8 {
                out.push(Road {
                    points: c,
                    rad: random::<i32>().abs() % 2 + 2,
                });
                break;
            } else {
                try_count += 1;
                hits = prev.clone();
                if try_count < 8 {
                    //       println!("retrying:{}", try_count);
                    continue;
                } else {
                    break;
                }
            }
        }
        //  println!("{}", starting_points.len());
    }
    println!("{:#?}", out.len());
    (out, hits)
}

pub fn generate_city(width: i32, height: i32) -> City {
    let mut out = City::new();
    let div = 10;
    let vf = generate_voronoish_count(width / div, height / div, (height * width) / (div * div));
    let roads = generate_roads(&vf, div as f32);
    out.roads = roads.0;
    out.occupied = roads.1;
    let mut queue: Vec<(i32, i32)> = (0..(height / div) * (width / div) * 20)
        .map(|_| {
            (
                random::<i32>().abs() % (width),
                random::<i32>().abs() % (height),
            )
        })
        .collect();
    let base = Image::gen_image_color(10, 10, Color::WHITE);
    let mut noise = base.gen_image_perlin_noise(
        vf.width(),
        vf.height(),
        random::<i32>().abs() % vf.width(),
        random::<i32>().abs() % vf.height(),
        10.,
    );
    while !queue.is_empty() {
        let (x, y) = rand_select(&mut queue);
        let dv = vf.get_div_at_point(x / div, y / div);
        //    println!("x:{}, y:{}, div:{:#?}", x, y, dv * 10.);
        let dir = vf.get(x / div, y / div);
        if (dv.abs() < 0.9 && dir.length_sqr() > 0.0) || true {
            //  println!("dir:{:#?}", vf.get(x, y));
            let pos = Vector2::new(x as f32, y as f32);
            let center = Vector2::new(width as f32 / 2., height as f32 / 2.);
            let dir = dir.angle_to(Vector2::new(1.0, 0.0));
            let distance = pos.distance_to(center);
            if distance
                >= width as f32
                    * (1. - noise.get_color(x / div, y / div).r as f32 / 255.).clamp(0.3, 1.0)
                    / 2.
            {
                continue;
            }
            let mut rad = 15 + random::<i32>().abs() % 6;
            if random::<i32>().abs() % 20 > 18 {
                rad = 25;
            }
            while rad >= 10 {
                let (w, h) = if random() {
                    (rad, rad - random::<i32>().abs() % 5)
                } else {
                    (rad - random::<i32>().abs() % 5, rad)
                };
                let p = Collider2D {
                    pos: pos,
                    rotation: dir,
                    width: w as f32,
                    height: h as f32,
                    velocity: Vector2::zero(),
                    mass: 0.0,
                };
                let p2 = Collider2D {
                    pos: pos,
                    rotation: dir,
                    width: w as f32,
                    height: h as f32,
                    velocity: Vector2::zero(),
                    mass: 0.0,
                };
                if !out.collides_with_building(p2, div as f32) && out.dist_to_road(p2) < 30. {
                    let b = Building { col: p };
                    out.buildings.push(b);
                    //println!("rad:{}", rad);
                    break;
                } else {
                    rad -= 1;
                }
            }
        }
    }
    let mut outb = Vec::new();
    for i in 0..out.buildings.len() {
        let tmp = &out.buildings[i];
        for j in 0..out.buildings.len() {
            if i == j {
                continue;
            }
            if tmp.col.pos.distance_to(out.buildings[j].col.pos) < 30. {
                outb.push(tmp.clone());
                break;
            }
        }
    }
    println!("building count:{}", outb.len());
    out.buildings = outb;
    out.field = vf;
    out.height = height;
    out.width = width;
    out
}

pub fn rand_select<T>(v: &mut Vec<T>) -> T {
    let idx = (random::<u64>() as usize) % v.len();
    let t = v.remove(idx);
    t
}

impl City {
    pub fn render(&self, name: &str) {
        let mut out = Image::gen_image_color(self.width, self.height, Color::WHITE);
        for i in &self.buildings {
            let base = i.col.pos;
            let delta_x = Vector2::new(1., 0.0).rotated(i.col.rotation) * (i.col.width - 1.) / 2.;
            let delta_y = Vector2::new(0., 1.0).rotated(i.col.rotation) * (i.col.height - 1.) / 2.;
            let points = [
                base + delta_x + delta_y,
                base + delta_x - delta_y,
                base - delta_x + delta_y,
                base - delta_x - delta_y,
            ];
            for i in 0..4 {
                let p1 = points[i];
                let p2 = points[(i + 1) % 4];
                let p3 = points[(i + 2) % 4];
                out.draw_triangle(p1, p2, p3, Color::BLACK);
            }
        }
        for i in &self.roads {
            for j in 0..i.points.len() - 1 {
                //    out.draw_line_ex(i.points[j], i.points[j + 1], 2, Color::LIGHTGRAY);
            }
        }
        out.export_image(name);
        render_vector_field_div(&self.field, &("div_".to_string() + name));
    }
}
