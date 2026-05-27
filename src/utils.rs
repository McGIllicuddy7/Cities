use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    hash::Hash,
    sync::{Arc, Mutex, MutexGuard},
};

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
