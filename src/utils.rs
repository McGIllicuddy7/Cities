use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    hash::Hash,
    sync::{Arc, Mutex, MutexGuard},
};

use raylib::{color::Color, math::Vector2};

pub struct ConcurrentHashMap<T: Hash + Eq, U> {
    inner: Arc<Mutex<HashMap<T, U>>>,
}
impl<T: Hash + Eq, U> ConcurrentHashMap<T, U> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
        }
    }
    pub fn insert(&self, key: T, value: U) -> Option<U> {
        self.inner.lock().unwrap().insert(key, value)
    }
    pub fn remove<V: ?Sized + Hash + Eq>(&self, key: &V) -> Option<U>
    where
        T: Borrow<V>,
    {
        self.inner.lock().unwrap().remove(key)
    }

    pub fn contains_key<V: ?Sized + Hash + Eq>(&self, key: &V) -> bool
    where
        T: Borrow<V>,
    {
        self.inner.lock().unwrap().contains_key(key)
    }

    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().len()
    }
    pub fn with<V: ?Sized + Hash + Eq, W>(
        &self,
        key: &V,
        to_run: impl FnOnce(&V, Option<&U>) -> W,
    ) -> W
    where
        T: Borrow<V>,
    {
        let lck = self.inner.lock().unwrap();
        let v0 = lck.get(key);
        (to_run)(key, v0)
    }
    pub fn with_mut<V: ?Sized + Hash + Eq, W>(
        &self,
        key: &V,
        to_run: impl FnOnce(&V, Option<&mut U>) -> W,
    ) -> W
    where
        T: Borrow<V>,
    {
        let mut lck = self.inner.lock().unwrap();
        let v0 = lck.get_mut(key);
        (to_run)(key, v0)
    }
    pub fn lock<'a>(&'a self) -> MutexGuard<'a, HashMap<T, U>> {
        self.inner.lock().unwrap()
    }
}
impl<T: Hash + Eq, U: Clone> ConcurrentHashMap<T, U> {
    pub fn get<V: ?Sized + Hash + Eq>(&self, v: &V) -> Option<U>
    where
        T: Borrow<V>,
    {
        self.inner.lock().unwrap().get(v).map(|i| i.clone())
    }
}

pub struct ConcurrentHashSet<T: Hash + Eq> {
    inner: Arc<Mutex<HashSet<T>>>,
}
impl<T: Hash + Eq> ConcurrentHashSet<T> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashSet::new())),
        }
    }
    pub fn insert(&self, key: T) -> bool {
        self.inner.lock().unwrap().insert(key)
    }
    pub fn remove<V: ?Sized + Hash + Eq>(&self, key: &V) -> bool
    where
        T: Borrow<V>,
    {
        self.inner.lock().unwrap().remove(key)
    }

    pub fn contains<V: ?Sized + Hash + Eq>(&self, key: &V) -> bool
    where
        T: Borrow<V>,
    {
        self.inner.lock().unwrap().contains(key)
    }

    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().len()
    }
}
impl<T: Hash + Eq + Clone> ConcurrentHashSet<T> {
    pub fn get<V: ?Sized + Hash + Eq>(&self, v: &V) -> Option<T>
    where
        T: Borrow<V>,
    {
        self.inner.lock().unwrap().get(v).map(|i| i.clone())
    }
}
#[derive(Clone, Debug)]
pub struct ConcurrentList<T> {
    inner: Arc<Mutex<Vec<T>>>,
}
impl<T> ConcurrentList<T> {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Vec::new())),
        }
    }
    pub fn len(&self) -> usize {
        self.inner.lock().unwrap().len()
    }
    pub fn push(&self, v: T) {
        self.inner.lock().unwrap().push(v);
    }
    pub fn pop(&self) -> Option<T> {
        self.inner.lock().unwrap().pop()
    }
    pub fn insert(&self, at: usize, v: T) -> Result<(), T> {
        let mut guard = self.inner.lock().unwrap();
        if guard.len() > at {
            return Err(v);
        } else {
            guard.insert(at, v);
            Ok(())
        }
    }

    pub fn remove(&self, at: usize) -> Option<T> {
        let mut guard = self.inner.lock().unwrap();
        if guard.len() >= at {
            None
        } else {
            Some(guard.remove(at))
        }
    }

    pub fn lock<'a>(&'a self) -> MutexGuard<'a, Vec<T>> {
        self.inner.lock().unwrap()
    }

    pub fn with<V>(&self, at: usize, func: impl FnOnce(usize, Option<&T>) -> V) -> V {
        let lck = self.inner.lock().unwrap();
        func(at, lck.get(at))
    }

    pub fn with_mut<V>(&self, at: usize, func: impl FnOnce(usize, Option<&mut T>) -> V) -> V {
        let mut lck = self.inner.lock().unwrap();
        func(at, lck.get_mut(at))
    }
}
impl<T: Clone> ConcurrentList<T> {
    pub fn get(&self, at: usize) -> Option<T> {
        self.inner.lock().unwrap().get(at).map(|i| i.clone())
    }
}

pub struct Timer<'a> {
    message: &'a str,
    start: std::time::Instant,
}
impl<'a> Timer<'a> {
    pub fn new(msg: &'a str) -> Self {
        Self {
            message: msg,
            start: std::time::Instant::now(),
        }
    }
}

impl<'a> Drop for Timer<'a> {
    fn drop(&mut self) {
        let dur = self.start.elapsed();
        println!("{} took: {:#?}", self.message, dur)
    }
}

pub fn lerp(p0: f32, p1: f32, amount: f32) -> f32 {
    p0 * (1. - amount) + p1 * amount
}
pub fn noise_1d(x: i32, y: i32, scale: f32, rsyn: &str) -> f32 {
    let scale = scale / 10.0;
    fn pos_vector(x: i32, y: i32, scale: f32, rsyn: &str) -> Vector2 {
        static MAP: Mutex<Option<HashMap<(i32, i32, i32, Arc<str>), f32>>> = Mutex::new(None);
        let mut g = MAP.lock().unwrap();
        if g.is_none() {
            *g = Some(HashMap::new());
        };
        let map = g.as_mut().unwrap();
        let s: Arc<str> = rsyn.into();
        let ist = (x, y, scale as i32, s);
        if let Some(theta) = map.get(&ist) {
            Vector2::new(theta.cos(), theta.sin())
        } else {
            let theta_0 = rand::random::<u32>() % 62_831;
            let theta = theta_0 as f32 / 10_000.;
            map.insert(ist, theta);
            Vector2::new(theta.cos(), theta.sin())
        }
    }
    let sx = x as f32 * scale;
    let sy = y as f32 * scale;
    let bx = sx.floor() as i32;
    let by = sy.floor() as i32;
    let dx = sx - bx as f32;
    let dy = sy - by as f32;
    let point = Vector2::new(dx, dy);
    let x0_y0 = (point - Vector2::new(0.0, 0.0)).dot(pos_vector(bx, by, scale, rsyn));
    let x1_y0 = (point - Vector2::new(1.0, 0.0)).dot(pos_vector(bx + 1, by, scale, rsyn));
    let x0_y1 = (point - Vector2::new(0.0, 1.0)).dot(pos_vector(bx, by + 1, scale, rsyn));
    let x1_y1 = (point - Vector2::new(1.0, 1.0)).dot(pos_vector(bx + 1, by + 1, scale, rsyn));
    let y_0s = lerp(x0_y0, x1_y0, dx);
    let y_1s = lerp(x0_y1, x1_y1, dx);
    let output = lerp(y_0s, y_1s, dy);
    (output + 1.) / 2.0
}

pub fn noise_3d(x: i32, y: i32, scale: f32) -> Color {
    let r = noise_1d(x, y, scale, "r");
    let g = noise_1d(x, y, scale, "g");
    let b = noise_1d(x, y, scale, "b");
    Color {
        r: (r * 255.0) as u8,
        g: (g * 255.0) as u8,
        b: (b * 255.0) as u8,
        a: 255,
    }
}

pub fn noise_1d_layered(x: i32, y: i32, scale: f32, rsyn: &str, layers: i32) -> f32 {
    let mut out = 0.0;
    let mut div = 1.0;
    let mut total = 0.0;
    for _ in 0..layers {
        out += noise_1d(x, y, scale * div, rsyn) / div;
        total += 1. / div;
        div *= 2.;
    }
    out / total
}

pub fn noise_3d_layered(x: i32, y: i32, scale: f32, layers: i32) -> Color {
    let r = noise_1d_layered(x, y, scale, "r", layers) * 255.;
    let g = noise_1d_layered(x, y, scale, "g", layers) * 255.;
    let b = noise_1d_layered(x, y, scale, "b", layers) * 255.;
    Color {
        r: r as u8,
        g: g as u8,
        b: b as u8,
        a: 255,
    }
}

pub fn blend(color0: Color, color1: Color, amount: f32) -> Color {
    let r = color0.r as f32 * (1. - amount) + color1.r as f32 * amount;
    let g = color0.g as f32 * (1. - amount) + color1.g as f32 * amount;
    let b = color0.b as f32 * (1. - amount) + color1.b as f32 * amount;
    let a = color0.a as f32 * (1. - amount) + color1.a as f32 * amount;
    Color {
        r: r as u8,
        g: g as u8,
        b: b as u8,
        a: a as u8,
    }
}

pub fn blend_hsv(color0: Color, color1: Color, amount: f32) -> Color {
    let c0_hsv = color0.color_to_hsv();
    let c1_hsv = color1.color_to_hsv();
    let l = c0_hsv * (1. - amount) + c1_hsv * amount;

    Color::color_from_hsv(l.x, l.y, l.z)
}

pub fn blend_3_way(color0: Color, color1: Color, color2: Color, amount: f32) -> Color {
    if amount < 0.5 {
        let v = amount * 2.;
        blend(color0, color1, v)
    } else {
        let v = amount * 2. - 1.;
        blend(color1, color2, v)
    }
}

pub fn blend_3_hsv(color0: Color, color1: Color, color2: Color, amount: f32) -> Color {
    if amount < 0.5 {
        let v = amount * 2.;
        blend_hsv(color0, color1, v)
    } else {
        let v = amount * 2. - 1.;
        blend_hsv(color1, color2, v)
    }
}

pub fn from_grayscale(v: f32) -> Color {
    Color {
        r: (v * 255.) as u8,
        g: (v * 255.) as u8,
        b: (v * 255.) as u8,
        a: 255,
    }
}
pub fn from_rgb(r: f32, g: f32, b: f32) -> Color {
    Color {
        r: (r * 255.) as u8,
        g: (g * 255.) as u8,
        b: (b * 255.) as u8,
        a: 255,
    }
}

pub fn burn(v: f32, thresh: f32) -> f32 {
    if (v - thresh).abs() < 0.1 {
        0.5
    } else if v < thresh {
        0.0
    } else {
        1.0
    }
}
