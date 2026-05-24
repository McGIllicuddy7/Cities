use std::{
    collections::{HashMap, HashSet},
    f32::consts::PI,
    sync::{Mutex, atomic::AtomicU32},
};

use rand::random;
use raylib::{color::Color, math::Vector2, texture::Image};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

pub struct Voronoi {
    pub next_idx: Mutex<u32>,
    pub height: i32,
    pub width: i32,
    pub data: Box<[AtomicU32]>,
    pub point_set: Vec<(i32, i32)>,
}

impl Voronoi {
    pub fn get(&self, x: i32, y: i32) -> u32 {
        if x >= self.width || y >= self.height || x < 0 || y < 0 {
            return 0;
        }
        self.data[(self.width * y + x) as usize].load(std::sync::atomic::Ordering::SeqCst)
    }

    pub fn set(&self, x: i32, y: i32, v: u32) {
        if x >= self.width || y >= self.height || x < 0 || y < 0 {
            return;
        }
        self.data[(self.width * y + x) as usize].store(v, std::sync::atomic::Ordering::SeqCst);
    }

    pub fn alloc_point(&self) -> u32 {
        let mut out = self.next_idx.lock().unwrap();
        let tmp = *out;
        *out += 1;
        tmp
    }
}
pub fn generate_voronoi(width: i32, height: i32) -> Voronoi {
    let mut data = Vec::new();
    data.reserve_exact((width * height) as usize);
    for _ in 0..height {
        for _ in 0..width {
            data.push(AtomicU32::new(0));
        }
    }
    let out = Voronoi {
        next_idx: Mutex::new(1),
        height: height,
        width: width,
        data: data.into_boxed_slice(),
        point_set: Vec::new(),
    };
    //let d = (width * height).isqrt();
    //let depth = (d / 1000 + 1);
    let depth = 2;
    subdivide_voronoi(&out, 0, depth, true);
    out
}

pub fn masked_subdivide(mask: u32, voronoi: &Voronoi, sv: i32) -> Vec<u32> {
    let mut min_x = voronoi.width;
    let mut min_y = voronoi.height;
    let mut max_x = 0;
    let mut max_y = 0;
    let mut list = Vec::new();
    for y in 0..voronoi.height {
        for x in 0..voronoi.width {
            if voronoi.get(x, y) == mask {
                if x < min_x {
                    min_x = x;
                }
                if y < min_y {
                    min_y = y;
                }
                if x > max_x {
                    max_x = x;
                }
                if y > max_y {
                    max_y = y;
                }
            }
        }
    }
    let mut points = Vec::new();
    let delta_x = (max_x - min_x) / sv;
    let delta_y = (max_y - min_y) / sv;
    if delta_x <= 2 || delta_y <= 2 {
        return list;
    }
    let angle = (random::<u64>() % 628) as f32 / 750. - (628 as f32) / 1500.;
    let vx = Vector2::new(1.0, 0.0).rotated(angle) * delta_x as f32;
    let vy = Vector2::new(0., 1.0).rotated(angle) * delta_y as f32;
    let base = Vector2::new(min_x as f32, min_y as f32);
    for i in -1..=sv {
        for j in -1..=sv {
            let x_mul = (j as f32 - (random::<u64>() % 100) as f32 / 200. + 0.75);
            let y_mul = (i as f32 - (random::<u64>() % 100) as f32 / 200. + 0.75);
            // println!("{}, {}", x_mul, y_mul);
            let point = vx * x_mul + vy * y_mul + base;
            let (x, y) = (point.x as i32, point.y as i32);
            let p = voronoi.alloc_point();
            let _v = ((x, y), p);
            // println!("{:#?}", _v);
            points.push(((x, y), p));
            list.push(p);
        }
    }
    (min_y..=max_y).into_par_iter().for_each(|y| {
        for x in min_x..=max_x {
            if voronoi.get(x, y) != mask {
                continue;
            }
            let mut closest_dist = 1000000;
            let mut best_idx = 0;
            let mut tied = false;
            for ((dx, dy), v) in &points {
                let delta = ((x - *dx) * (x - *dx) + (y - *dy) * (y - *dy))
                    .abs()
                    .isqrt();
                if delta <= closest_dist {
                    best_idx = *v;
                    closest_dist = delta;
                    tied = false;
                }
            }
            voronoi.set(x, y, if !tied { best_idx } else { 0 });
        }
    });
    list
}
impl Voronoi {
    pub fn render(&self) {
        let mut img = Image::gen_image_color(self.width as i32, self.height as i32, Color::WHITE);
        for y in 0..self.height {
            for x in 0..self.width {
                let g = self.get(x, y);
                let c = u32_to_color(g);
                img.draw_pixel(x, y, c);
            }
        }
        img.export_image("test.png");
    }
}

pub fn u32_to_color(v: u32) -> Color {
    let t2 = v * 52 + v;
    let c = t2.to_le_bytes();
    let out = Color {
        r: c[0],
        g: c[1],
        b: c[2],
        a: 255,
    };
    out
}

pub fn subdivide_voronoi(voronoi: &Voronoi, base: u32, depth: i32, first: bool) {
    let points = masked_subdivide(
        base,
        voronoi,
        if first {
            let tmp = ((voronoi.width * voronoi.height).isqrt() / 250).clamp(0, 300);
            println!("tmp:{tmp}");
            tmp
        } else {
            4
        },
    );
    if depth > 0 {
        for i in &points {
            expand_borders_for(voronoi, *i, 1);
        }
        for i in points {
            subdivide_voronoi(voronoi, i, depth - 1, false);
        }
    }
}

pub fn expand_borders_for(voronoi: &Voronoi, base: u32, road_size: i32) {
    let mut to_set = Vec::new();
    for y in 0..voronoi.height {
        for x in 0..voronoi.width {
            if voronoi.get(x, y) != base {
                continue;
            }
            let mut borders = false;
            for dy in -road_size..=road_size {
                for dx in -road_size..=road_size {
                    if dx == 0 && dy == 0 {
                        continue;
                    }
                    let v = voronoi.get(x + dx, y + dy);
                    if v != 0 && v != base {
                        borders = true;
                    }
                }
            }
            if borders {
                to_set.push((x, y));
            }
        }
    }
    for (x, y) in to_set {
        voronoi.set(x, y, 0);
    }
}
#[derive(Clone, Debug)]
pub struct Vertex {
    pub x: i32,
    pub y: i32,
    pub borders: Vec<u32>,
}
#[derive(Clone, Debug)]
pub struct PointCollection {
    pub points: Vec<(i32, i32)>,
    pub vertices: Vec<Vertex>,
}
pub struct CityCollection {
    pub vor: Voronoi,
    pub point_count: u32,
    pub vertices: Vec<Vertex>,
    pub collections: HashMap<u32, PointCollection>,
}
pub fn setup_city_collection(width: i32, height: i32) -> CityCollection {
    let vor = generate_voronoi(width, height);
    let point_count = *vor.next_idx.lock().unwrap();
    let (vertices, collections) = calculate_vertices(&vor);
    let mut out = CityCollection {
        vor,
        point_count,
        vertices,
        collections,
    };
    cleanup_city_collection(&mut out);
    out
}

pub fn calculate_vertices(vor: &Voronoi) -> (Vec<Vertex>, HashMap<u32, PointCollection>) {
    let mut list = Vec::new();
    let mut point_set = HashMap::new();
    for y in 0..vor.height {
        for x in 0..vor.width {
            let v0 = vor.get(x, y);
            if !point_set.contains_key(&v0) {
                point_set.insert(
                    v0,
                    PointCollection {
                        points: Vec::new(),
                        vertices: Vec::new(),
                    },
                );
            }
            point_set.get_mut(&v0).unwrap().points.push((x, y));
            let mut kinds = Vec::new();
            for dy in -2..=2 {
                for dx in -2..=2 {
                    if dx != 0 && dy != 0 {
                        continue;
                    }
                    let v = vor.get(x + dx, y + dy);
                    if !kinds.contains(&v) && v != 0 {
                        kinds.push(v);
                    }
                }
            }
            if kinds.len() > 2 {
                list.push(Vertex {
                    x,
                    y,
                    borders: kinds,
                });
            }
        }
    }
    let mut list2: Vec<Vertex> = Vec::new();
    for i in &mut list {
        i.borders.sort();
    }
    let mut hs = HashSet::new();
    for i in &list {
        let mut center = (0, 0);
        let mut count = 0;
        let mut b2 = i.borders.clone();
        for j in &list2 {
            if j.borders == i.borders {
                continue;
            }
        }
        for j in &list {
            if j.borders == i.borders {
                center.0 += j.x;
                center.1 += j.y;
                count += 1;
            }
        }
        b2.sort();
        center.0 /= count;
        center.1 /= count;
        if hs.contains(&center) {
            continue;
        }
        hs.insert(center);
        list2.push(Vertex {
            x: center.0,
            y: center.1,
            borders: b2,
        });
    }
    list = list2;
    let mut list2 = Vec::new();
    let mut done_set = HashSet::new();

    for i in 0..list.len() {
        if done_set.contains(&i) {
            continue;
        }
        done_set.insert(i);
        let ax = list[i].x;
        let ay = list[i].y;
        let mut center = (list[i].x, list[i].y);
        let mut b2 = list[i].borders.clone();
        let mut count = 1;
        for j in i + 1..list.len() {
            let bx = list[j].x;
            let by = list[j].y;
            let d = ((ax - bx) * (ax - bx) + (ay - by) * (ay - by)).isqrt();
            if d < 4 {
                center.0 += bx;
                center.1 += by;
                count += 1;
                for k in &list[j].borders {
                    if !b2.contains(k) {
                        b2.push(*k);
                    }
                }
                done_set.insert(j);
            }
        }
        b2.sort();
        center.0 /= count;
        center.1 /= count;
        list2.push(Vertex {
            x: center.0,
            y: center.1,
            borders: b2,
        });
    }
    list = list2;
    for i in &list {
        for j in &i.borders {
            point_set.get_mut(j).unwrap().vertices.push(i.clone());
        }
    }
    for (_, col) in &mut point_set {
        let mut cx = 0;
        let mut cy = 0;
        for i in &col.points {
            cx += i.0;
            cy += i.1;
        }
        cx /= (col.points.len() as i32);
        cy /= (col.points.len() as i32);
        col.vertices.sort_by(|a, b| {
            let x0 = &a.x;
            let y0 = &a.y;
            let x1 = &b.x;
            let y1 = &b.y;
            use raylib::prelude::Vector2;
            let ax = *x0 - cx;
            let ay = *y0 - cy;
            let bx = *x1 - cx;
            let by = *y1 - cy;
            let v1 = Vector2::new(ax as f32, ay as f32).normalized();
            let v2 = Vector2::new(bx as f32, by as f32).normalized();
            let t1 = v1.y.atan2(v1.x);
            let t2 = v2.y.atan2(v2.x);
            t1.partial_cmp(&t2).unwrap()
        });
    }
    (list, point_set)
}
impl CityCollection {
    pub fn render(&self) {
        let mut img =
            Image::gen_image_color(self.vor.width as i32, self.vor.height as i32, Color::WHITE);
        for y in 0..self.vor.height {
            for x in 0..self.vor.width {
                let g = self.vor.get(x, y);
                let c = u32_to_color(g);
                img.draw_pixel(x, y, c);
            }
        }
        for j in &self.vertices {
            img.draw_pixel(j.x, j.y, Color::WHITE);
        }
        img.export_image("test.png");
    }

    pub fn render_funny(&self) {
        let mut img =
            Image::gen_image_color(self.vor.width as i32, self.vor.height as i32, Color::WHITE);
        for (_, p) in &self.collections {
            for i in 0..p.vertices.len() {
                let j = (i + 1) % p.vertices.len();
                let base = &p.vertices[i];
                let second = &p.vertices[j];
                img.draw_line(base.x, base.y, second.x, second.y, Color::BLACK);
            }
        }
        img.export_image("test_funny.png");
    }
}

pub fn cleanup_city_collection(cc: &mut CityCollection) {
    let mut col_new = HashMap::new();
    for (point_idx, ps) in &mut cc.collections {
        let mut hs = HashSet::new();
        let mut vs2 = Vec::new();
        for i in 0..ps.vertices.len() {
            let a = &ps.vertices[i];
            let mut cx = a.x;
            let mut cy = a.y;
            if hs.contains(&(a.x, a.y)) {
                continue;
            }
            let mut count = 1;
            let mut borders = a.borders.clone();
            for j in i + 1..ps.vertices.len() {
                let b = &ps.vertices[j];
                let d = ((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)).isqrt();
                if d < 6 {
                    hs.insert((b.x, b.y));
                    cx += b.x;
                    cy += b.y;
                    count += 1;
                    for k in &b.borders {
                        if !borders.contains(k) {
                            borders.push(*k);
                        }
                    }
                }
            }
            cx = cx / count;
            cy = cy / count;
            borders.sort();
            vs2.push(Vertex {
                x: cx,
                y: cy,
                borders,
            })
        }
        let mut cx = 0;
        let mut cy = 0;
        for i in &vs2 {
            cx += i.x;
            cy += i.y;
        }
        if vs2.len() == 0 {
            ps.vertices = vs2;
            continue;
        }
        cx /= (vs2.len() as i32);
        cy /= (vs2.len() as i32);
        for i in &mut vs2 {
            use raylib::math::Vector2;
            let dx = (i.x - cx);
            let dy = (i.y - cy);
            let v2 = Vector2::new(dx as f32, dy as f32);
            let l = v2.length();
            let x = i.x;
            let y = i.y;
            let disp = {
                let mut tmp = 2.;
                for dy in -2..=2 {
                    for dx in -2..=2 {
                        if cc.vor.get(x + dx, y + dy) == 0 {
                            if dx.abs() < 1 && dy.abs() < 1 {
                                tmp = 6.
                            } else if tmp < 8. {
                                tmp = 4.;
                            }
                        }
                    }
                }
                tmp
            };
            let l = if l < disp { 0. } else { l - disp };
            let vn = v2.normalized();
            let vnew = vn * l + Vector2::new(cx as f32, cy as f32);
            i.x = vnew.x as i32;
            i.y = vnew.y as i32;
        }

        let mut vs: Vec<Vertex> = Vec::new();
        let mut done_set = HashSet::new();
        for i in 0..vs2.len() {
            if done_set.contains(&i) {
                continue;
            }
            done_set.insert(i);
            let base = &vs2[i];
            let mut cx = base.x;
            let mut cy = base.y;
            let mut count = 1;
            let mut borders = base.borders.clone();
            for j in i + 1..vs2.len() {
                let second = &vs2[j];
                let d = ((base.x - second.x) * (base.x - second.x)
                    + (base.y - second.y) * (base.y - second.y))
                    .isqrt();
                if d < 4 {
                    cx += second.x;
                    cy += second.y;
                    count += 1;
                    done_set.insert(j);
                    for k in &second.borders {
                        if !borders.contains(k) {
                            borders.push(*k);
                        }
                    }
                }
            }
            borders.sort();
            cx /= count;
            cy /= count;
            vs.push(Vertex {
                x: cx,
                y: cy,
                borders,
            });
        }
        vs2 = vs;
        vs2.sort_by(|a, b| {
            let x0 = &a.x;
            let y0 = &a.y;
            let x1 = &b.x;
            let y1 = &b.y;
            use raylib::prelude::Vector2;
            let ax = *x0 - cx;
            let ay = *y0 - cy;
            let bx = *x1 - cx;
            let by = *y1 - cy;
            let v1 = Vector2::new(ax as f32, ay as f32).normalized();
            let v2 = Vector2::new(bx as f32, by as f32).normalized();
            let t1 = v1.y.atan2(v1.x);
            let t2 = v2.y.atan2(v2.x);
            t1.partial_cmp(&t2).unwrap()
        });
        let mut tmp = ps.clone();
        tmp.vertices = vs2;
        let mut is_degen = false;
        'lp: loop {
            if tmp.vertices.len() < 4 {
                is_degen = true;
                break;
            }
            for i in 0..tmp.vertices.len() {
                use raylib::math::Vector2;
                let j = (i + 1) % tmp.vertices.len();
                let k = if i == 0 {
                    tmp.vertices.len() - 1
                } else {
                    i - 1
                };
                let p0 = Vector2::new(tmp.vertices[k].x as f32, tmp.vertices[k].y as f32);
                let p1 = Vector2::new(tmp.vertices[i].x as f32, tmp.vertices[i].y as f32);
                let p2 = Vector2::new(tmp.vertices[j].x as f32, tmp.vertices[j].y as f32);
                if p0.distance_to(p1) < 1. {
                    is_degen = true;
                    break 'lp;
                }
                if p1.distance_to(p2) < 1. {
                    is_degen = true;
                    break 'lp;
                }
                let d1 = (p2 - p1).normalized();
                let d2 = (p1 - p0).normalized();
                let ang = d1.angle_to(d2);
                if ang < 0.1 {
                    is_degen = true;
                    break 'lp;
                }
            }
            break;
        }
        if !is_degen {
            col_new.insert(*point_idx, tmp);
        }
    }
    cc.collections = col_new;
}
