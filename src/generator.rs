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
            5.,
        );
        for i in 0..self.height() {
            for j in 0..self.width() {
                let amnt = noise.get_color(j, i).r as f32 / (255.0 * 1.5)
                    + (random::<i32>().abs() % 1000) as f32 / 1000.0 * 0.01;
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
            let div = (f.get_div_at_point(j, i));
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
                    let delta = (*i - p) / (dist * dist);
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
                    let delta = (*i - p).normalized() / (dist);
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
    f.noise(2.);
    f
}
#[derive(Debug)]
pub struct Building {
    pub col: Collider2D,
}

#[derive(Debug)]
pub struct City {
    pub buildings: Vec<Building>,
    pub field: VectorField,
    pub width: i32,
    pub height: i32,
}
impl City {
    pub fn new() -> Self {
        Self {
            buildings: Vec::new(),
            field: VectorField::new_zero(0, 0),
            width: 0,
            height: 0,
        }
    }

    pub fn collides_with_building(&self, col: Collider2D) -> bool {
        for i in &self.buildings {
            if i.col.pos.distance_to(col.pos) < (col.height + col.width) / 1.9 {
                return true;
            }
        }
        false
    }
}

pub fn generate_city(width: i32, height: i32) -> City {
    let mut out = City::new();
    let div = 5;
    let vf = generate_voronoish_count(width / div, height / div, (height * width) / (div * div));
    let mut queue: Vec<(i32, i32)> = (0..(height / div) * (width / div))
        .map(|_| (random::<i32>() % (width), random::<i32>() % (height)))
        .collect();
    while !queue.is_empty() {
        let (x, y) = rand_select(&mut queue);
        let dv = vf.get_div_at_point(x / div, y / div);
        //    println!("x:{}, y:{}, div:{:#?}", x, y, dv * 10.);
        let dir = vf.get(x / div, y / div);
        if dv.abs() < 0.1 && dir.length_sqr() > 0.0 {
            //  println!("dir:{:#?}", vf.get(x, y));
            let pos = Vector2::new(x as f32, y as f32);
            let center = Vector2::new(width as f32 / 2., height as f32 / 2.);
            let dir = dir.angle_to(Vector2::new(1.0, 0.0));
            let distance = pos.distance_to(center);
            if distance >= width as f32 / 1.9 {
                continue;
            }
            let mut rad = 16;
            while rad >= 10 {
                let p = Collider2D {
                    pos: pos,
                    rotation: dir,
                    width: rad as f32,
                    height: rad as f32,
                    velocity: Vector2::zero(),
                    mass: 0.0,
                };
                let p2 = Collider2D {
                    pos: pos,
                    rotation: dir,
                    width: rad as f32 + 2.,
                    height: rad as f32 + 2.,
                    velocity: Vector2::zero(),
                    mass: 0.0,
                };
                if !out.collides_with_building(p2) {
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
            let delta_x = Vector2::new(1., 0.0).rotated(i.col.rotation) * (i.col.width - 1.5) / 2.;
            let delta_y = Vector2::new(0., 1.0).rotated(i.col.rotation) * (i.col.height - 1.5) / 2.;
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
        out.export_image(name);
        render_vector_field_div(&self.field, &("div_".to_string() + name));
    }
}
