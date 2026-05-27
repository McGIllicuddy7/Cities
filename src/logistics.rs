use std::{
    collections::{BTreeMap, HashMap, HashSet},
    ops::Index,
    sync::{Arc, Mutex, atomic::AtomicI64},
};

use rand::{rng, seq::SliceRandom};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

use crate::utils::{ConcurrentHashSet, ConcurrentList};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct State {
    pub list: BTreeMap<Arc<str>, i64>,
}

#[derive(Clone)]
pub struct Constraint {
    pub value: Arc<dyn Fn(&State) -> i64 + Send + Sync>,
    pub ge: bool,
    pub threshold: i64,
}
#[derive(Clone)]
pub struct ConstraintSet {
    pub maximize: Arc<dyn Fn(&State) -> i64 + Send + Sync>,
    pub should_maximize: bool,
    pub constraints: Vec<Constraint>,
}
impl ConstraintSet {
    pub fn is_valid(&self, s: &State) -> bool {
        for i in &self.constraints {
            let v = (i.value)(s);
            if i.ge {
                if v < i.threshold {
                    return false;
                }
            } else {
                if v > i.threshold {
                    return false;
                }
            }
        }
        true
    }
    pub fn find_usable_state(&self, seed: State) -> State {
        let stck = ConcurrentList::new();
        stck.push(seed.clone());
        let tried_set = ConcurrentHashSet::new();
        let out = Mutex::new(None);
        let mut avg = 0;
        for i in &self.constraints {
            if i.threshold.abs() > avg {
                avg = i.threshold.abs();
            }
        }
        avg /= (self.constraints.len() as i64);
        let v0 = AtomicI64::new(avg);
        (0..8).into_par_iter().for_each(|_| {
            let mut idx = 0;
            while let Some(x) = stck.pop() {
                if self.is_valid(&x) {
                    println!("{:#?}", x);
                    *out.lock().unwrap() = Some(x);
                    break;
                }
                for (name, _) in &x.list {
                    let v1 = v0.load(std::sync::atomic::Ordering::SeqCst);
                    let mut s1 = x.clone();
                    *s1.list.get_mut(name).unwrap() += v1;
                    let mut s2 = x.clone();
                    *s2.list.get_mut(name).unwrap() -= v1;
                    if !tried_set.contains(&s1) {
                        tried_set.insert(s1.clone());
                        stck.push(s1);
                    }
                    if !tried_set.contains(&s2) {
                        tried_set.insert(s2.clone());
                        stck.push(s2);
                    }
                }
                idx += 1;
                if idx > 5 {
                    let v1 = v0.load(std::sync::atomic::Ordering::SeqCst);
                    let v2 = v1 / 2;
                    if v2 <= 0 {
                        v0.store(avg, std::sync::atomic::Ordering::SeqCst);
                    } else {
                        v0.store(avg, std::sync::atomic::Ordering::SeqCst);
                    }
                    idx = 0;
                }
                stck.lock().shuffle(&mut rng());
                if out.lock().unwrap().is_some() {
                    break;
                }
            }
        });

        out.lock().unwrap().take().unwrap()
    }

    pub fn solve(&self, seed: State) -> State {
        let mut state = self.find_usable_state(seed);
        let mut max_eval = (self.maximize)(&state);
        let mut try_stack: Vec<State> = Vec::new();
        let mut tried_states: HashSet<State> = HashSet::new();
        tried_states.insert(state.clone());
        let mut failed_count = 0;
        loop {
            let mut hit = false;
            for (name, _v) in &state.list {
                let mut s1 = state.clone();
                let mut s2 = state.clone();
                *s1.list.get_mut(name).unwrap() += 1;
                *s2.list.get_mut(name).unwrap() -= 1;
                for (name1, _) in &state.list {
                    if name == name1 {
                        continue;
                    }
                    let mut s3 = s1.clone();
                    let mut s4 = s1.clone();
                    *s3.list.get_mut(name1).unwrap() += 1;
                    *s4.list.get_mut(name1).unwrap() -= 1;
                    if self.is_valid(&s3) {
                        try_stack.push(s3);
                    }
                    if self.is_valid(&s4) {
                        try_stack.push(s4);
                    }
                }
                for (name1, _) in &state.list {
                    if name == name1 {
                        continue;
                    }
                    let mut s3 = s2.clone();
                    let mut s4 = s2.clone();
                    *s3.list.get_mut(name1).unwrap() += 1;
                    *s4.list.get_mut(name1).unwrap() -= 1;
                    if self.is_valid(&s3) && !tried_states.contains(&s3) {
                        try_stack.push(s3.clone());
                    }
                    tried_states.insert(s3);
                    if self.is_valid(&s4) && !tried_states.contains(&s4) {
                        try_stack.push(s4.clone());
                    }
                    tried_states.insert(s4.clone());
                }
                try_stack.push(s1.clone());
                try_stack.push(s2.clone());
            }
            let mut increased = false;
            while let Some(x) = try_stack.pop() {
                if self.is_valid(&x) {
                    let v = (self.maximize)(&x);
                    if self.should_maximize {
                        if v >= max_eval {
                            if v > max_eval {
                                increased = true;
                            }
                            max_eval = v;
                            state = x;
                            hit = true;
                        }
                    } else {
                        if v <= max_eval {
                            if v < max_eval {
                                increased = true;
                            }
                            max_eval = v;
                            state = x;
                            hit = true;
                        }
                    }
                }
                if !increased {
                    failed_count += 1;
                    if failed_count > 100 {
                        hit = false;
                        break;
                    }
                } else {
                    failed_count = 0;
                }
            }
            if !hit {
                break;
            }
        }
        state
    }
    pub fn maximize(
        to_maximize: impl (Fn(&State) -> i64) + 'static + Send + Sync,
        constraints: &[Constraint],
    ) -> Self {
        Self {
            maximize: Arc::new(to_maximize),
            should_maximize: true,
            constraints: constraints.to_vec(),
        }
    }
    pub fn minimize(
        to_minimize: impl (Fn(&State) -> i64) + 'static + Send + Sync,
        constraints: &[Constraint],
    ) -> Self {
        Self {
            maximize: Arc::new(to_minimize),
            should_maximize: false,
            constraints: constraints.to_vec(),
        }
    }
}

impl Constraint {
    pub fn ge(func: impl (Fn(&State) -> i64) + 'static + Send + Sync, threshold: i64) -> Self {
        Self {
            value: Arc::new(func),
            ge: true,
            threshold,
        }
    }
    pub fn le(func: impl (Fn(&State) -> i64) + 'static + Send + Sync, threshold: i64) -> Self {
        Self {
            value: Arc::new(func),
            ge: false,
            threshold,
        }
    }
}

impl State {
    pub fn new(values: &[(&str, i64)]) -> Self {
        let mut out = BTreeMap::new();
        for (k, v) in values {
            out.insert((*k).into(), *v);
        }
        Self { list: out }
    }
}
impl Index<&str> for State {
    type Output = i64;

    fn index(&self, index: &str) -> &Self::Output {
        &self.list[index]
    }
}
