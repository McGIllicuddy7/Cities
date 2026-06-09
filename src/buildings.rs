use std::{
    collections::{HashMap, HashSet},
    f32::consts::PI,
    sync::{Mutex, atomic::AtomicU32},
};

use rand::random;
use raylib::{color::Color, math::Vector2, texture::Image};
use rayon::iter::{
    IntoParallelIterator, IntoParallelRefIterator, IntoParallelRefMutIterator, ParallelBridge,
    ParallelIterator,
};

use crate::utils::{ConcurrentHashMap, ConcurrentHashSet, ConcurrentList, noise_1d_layered};

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
    let tmul = if sv <= 2 { 1. } else { 1. };
    for i in -1..=sv {
        for j in -1..=sv {
            let x_mul = (j as f32 - ((random::<u64>() % 100) as f32 / 200. - 0.25) * tmul + 0.5);
            let y_mul = (i as f32 - ((random::<u64>() % 100) as f32 / 200. - 0.25) * tmul + 0.5);
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
    (min_y..=max_y).for_each(|y| {
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
            let tmp = ((voronoi.width * voronoi.height).isqrt() / 180).clamp(0, 300);
            println!("tmp:{tmp}");
            tmp
        } else if depth > 0 {
            4
        } else {
            2
        },
    );
    if depth > 0 {
        for i in &points {
            expand_borders_for(voronoi, *i, 1);
        }
        if first {
            points.par_iter().for_each(|i| {
                subdivide_voronoi(voronoi, *i, depth - 1, false);
            });
        } else {
            points.iter().for_each(|i| {
                subdivide_voronoi(voronoi, *i, depth - 1, false);
            });
        }
    }
}

pub fn expand_borders_for(voronoi: &Voronoi, base: u32, road_size: i32) {
    let to_set = ConcurrentList::new();
    (0..voronoi.height).for_each(|y| {
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
    });
    to_set.lock().par_iter().for_each(|(x, y)| {
        voronoi.set(*x, *y, 0);
    });
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
    println!("starting vertex calculation");
    let list = ConcurrentList::new();
    let point_set = ConcurrentHashMap::new();
    (0..vor.height).into_par_iter().for_each(|y| {
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
            //  point_set.get_mut(&v0).unwrap().points.push((x, y));
            point_set.with_mut(&v0, |_, points| points.unwrap().points.push((x, y)));
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
    });
    println!("set up done");
    let list2: ConcurrentList<Vertex> = ConcurrentList::new();
    list.lock().par_iter_mut().for_each(|i| {
        i.borders.sort();
    });
    println!("borders sorted");
    let hs = ConcurrentHashSet::new();
    let lck = list.lock();
    lck.par_iter().for_each(|i| {
        let mut center = (0, 0);
        let mut count = 0;
        let mut b2 = i.borders.clone();
        for j in list2.lock().iter() {
            if j.borders == i.borders {
                continue;
            }
        }
        for j in lck.iter() {
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
            return;
        }
        hs.insert(center);
        list2.push(Vertex {
            x: center.0,
            y: center.1,
            borders: b2,
        });
    });
    println!("shift completed");
    let list = list2;
    let list2 = ConcurrentList::new();
    let done_set = ConcurrentHashSet::new();
    let list = list.lock();
    println!("testing 1 2 3");
    (0..list.len()).into_par_iter().for_each(|i| {
        if done_set.contains(&i) {
            return;
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
    });
    let list = list2;
    println!("testing 4 5 6");
    for i in list.lock().iter() {
        for j in &i.borders {
            // point_set.get_mut(j).unwrap().vertices.push(i.clone());
            point_set.with_mut(j, |_, v| {
                v.unwrap().vertices.push(i.clone());
            })
        }
    }
    println!("testing 7 8 9");
    point_set.lock().par_iter_mut().for_each(|(_, col)| {
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
    });
    println!("returning");
    (list.lock().clone(), point_set.lock().clone())
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
            for j in 0..ps.vertices.len() {
                if i == j {
                    continue;
                }
                let b = &ps.vertices[j];
                let d = ((a.x - b.x) * (a.x - b.x) + (a.y - b.y) * (a.y - b.y)).isqrt();
                if d < 4 {
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
        let mut cx = 0.;
        let mut cy = 0.;
        for i in &vs2 {
            cx += i.x as f32;
            cy += i.y as f32;
        }
        if vs2.len() == 0 {
            ps.vertices = vs2;
            continue;
        }
        cx /= (vs2.len() as f32);
        cy /= (vs2.len() as f32);
        let c = Vector2::new(cx, cy);
        let mut prev_distance =
            Vector2::new(vs2[vs2.len() - 1].x as f32, vs2[vs2.len() - 1].y as f32).distance_to(c);
        /*   let mut updated = true;
        while updated {
            for i in 0..vs2.len() {
                let v0 = Vector2::new(vs2[i].x as f32, vs2[i].y as f32);
                let d1 = v0 - c;
                let d1c = d1.length();
                if d1c < prev_distance {
                    let d2 = d1.normalized();
                    let v2 = d2 * prev_distance;
                    vs2[i].x = v2.x as i32;
                    vs2[i].y = v2.y as i32;
                } else if prev_distance < d1c {
                    prev_distance = d1c;
                }
            }
            break;
        }*/
        let mut hit_set = Vec::new();
        for i in &vs2 {
            let mut hit = false;
            'b: for dy in -2..=2 {
                for dx in -2..=2 {
                    if cc.vor.get(i.x + dx, i.y + dy) == 0 {
                        hit = true;
                        break 'b;
                    }
                }
            }
            hit_set.push(hit);
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
                if d < 8 {
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
        for di in 0..vs2.len() {
            let v0 = &vs2[if di == 0 { vs2.len() - 1 } else { di - 1 }];
            let v1 = &vs2[di];
            let v2 = &vs2[(di + 1) % vs2.len()];
            let h0 = hit_set[if di == 0 { vs2.len() - 1 } else { di - 1 }];
            let h1 = hit_set[di];
            let h2 = hit_set[(di + 1) % hit_set.len()];
            let p0 = Vector2::new(v0.x as f32, v1.x as f32);
            let p1 = Vector2::new(v1.x as f32, v1.y as f32);
            let p2 = Vector2::new(v2.x as f32, v2.y as f32);
            let delta = c - p1;
            if delta.length() <= 0.1 {
                continue;
            }
            let l1 = delta.length() * 0.9;
            let dn = delta.normalized();
            {
                let d1 = -dn * l1 + c;
                vs2[di].x = d1.x as i32;
                vs2[di].y = d1.y as i32;
            }
            let n0 = (p1 - p0).rotated(PI / 2.);
            let n1 = (p2 - p1).rotated(PI / 2.);
            let n0p = if n0.dot(dn) < 0. { -n0 } else { n0 };
            let n1p = if n1.dot(dn) < 0. { -n1 } else { n1 };
            let mut voff = ((n0p + n1p) / 2.).normalized() * 0.2 + (dn * 0.8);
            if voff.dot(dn) < 0. {
                voff = -voff;
            }
            voff *= 1.2;
            let rw = 2.;
            if h1 {
                if h0 && !h2 {
                    let r1 = (p1 - p0).normalized().rotated(PI / 2.);
                    if r1.dot(dn) < 0.0 {
                        voff += -r1 * rw;
                    } else {
                        voff += r1 * rw;
                    }
                } else if h2 && !h0 {
                    let r1 = (p2 - p1).normalized().rotated(PI / 2.);
                    if r1.dot(dn) < 0.0 {
                        voff += -r1 * rw;
                    } else {
                        voff += r1 * rw;
                    }
                } else if h0 && h2 {
                    voff *= 4.;
                }
            }
            vs2[di].x += voff.x as i32;
            vs2[di].y += voff.y as i32;
        }
        vs2.sort_by(|a, b| {
            let x0 = a.x as f32;
            let y0 = a.y as f32;
            let x1 = b.x as f32;
            let y1 = b.y as f32;
            use raylib::prelude::Vector2;
            let ax = x0 - cx;
            let ay = y0 - cy;
            let bx = x1 - cx;
            let by = y1 - cy;
            let v1 = Vector2::new(ax as f32, ay as f32).normalized();
            let v2 = Vector2::new(bx as f32, by as f32).normalized();
            let t1 = v1.y.atan2(v1.x);
            let t2 = v2.y.atan2(v2.x);
            t1.partial_cmp(&t2).unwrap()
        });
        let mut tmp = ps.clone();
        tmp.vertices = vs2;
        let mut is_degen = false;
        for i in 0..tmp.vertices.len() {
            for j in 0..tmp.vertices.len() {
                if i == j {
                    continue;
                }
                if i == (j + 1) % tmp.vertices.len() || j == (i + 1) % tmp.vertices.len() {
                    continue;
                }
                let d = ((tmp.vertices[i].x - tmp.vertices[j].x)
                    * (tmp.vertices[i].x - tmp.vertices[j].x)
                    + (tmp.vertices[i].y - tmp.vertices[j].y)
                        * (tmp.vertices[i].y - tmp.vertices[j].y))
                    .isqrt();
                if d < 8 {
                    is_degen = true;
                }
            }
        }
        'lp: loop {
            if is_degen {
                break;
            }
            if tmp.vertices.len() < 4 {
                is_degen = true;
                break;
            }
            let mut dcx = 0;
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
                if p0.distance_to(p1) > 40. {
                    is_degen = true;
                    break 'lp;
                }
                if p1.distance_to(p2) > 40. {
                    is_degen = true;
                    break 'lp;
                }
                let d1 = (p2 - p0).normalized();
                let d2 = (p0 - p1).normalized();
                let ang = d1.dot(d2);
                if ang > 0.5 || ang < -0.1 {
                    dcx += 1;
                    if dcx > 1 {
                        //        is_degen = true;
                        //        break 'lp;
                    }
                }
            }
            if tmp.points.len() < 400 || tmp.points.len() > 4096 {
                is_degen = true;
                break 'lp;
            }
            break;
        }
        let w = cc.vor.width as f32 / 2.;
        if !is_degen
            && ((noise_1d_layered(c.x as i32, c.y as i32, 0.1, "noise", 4) + 0.4)
                > ((c.x - w) * (c.x - w) + (c.y - w) * (c.y - w)).sqrt() / (w))
        {
            col_new.insert(*point_idx, tmp);
        }
    }
    cc.collections = col_new;
}
