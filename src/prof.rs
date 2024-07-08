use lazy_static::lazy_static;
use std::collections::HashMap;
use std::time;
#[allow(unused)]
////
///ProfileFrames should not ever be cloneable or copyable outside of prof, ideally they
///wouldn't be movable either but such is life
pub struct ProfileFrame {
    instant: time::Instant,
    func_name: &'static str,
}
fn duration_get(t: u128) -> f64 {
    t as f64 / 1_000_000.
}
impl Drop for ProfileFrame {
    fn drop(&mut self) {
        frame_pop(&self);
    }
}
pub fn private_profile_start(func_name: &'static str) -> ProfileFrame {
    let frame = ProfileFrame {
        instant: time::Instant::now(),
        func_name,
    };
    frame_push(&frame);
    frame
}
#[macro_export]
macro_rules! prof_frame {
    ($name:expr) => {
        let _frame = crate::prof::private_profile_start($name);
    };
}
struct TimeManager {
    times: Option<*mut HashMap<&'static str, u128>>,
    stack: Vec<&'static str>,
}
impl TimeManager {
    pub fn frame_push(&mut self, frame: &ProfileFrame) {
        if self.times.is_none() {
            let m = Box::new(HashMap::new() as HashMap<&'static str, u128>);
            let t = Box::leak(m);
            self.times = Some(t);
        }
        let times = unsafe { &mut *self.times.expect("should be initialized") };
        if !(times.contains_key(frame.func_name)) {
            times.insert(frame.func_name, 0);
        }
        self.stack.push(frame.func_name);
    }
    pub fn frame_pop(&mut self, frame: &ProfileFrame) {
        let times = unsafe { &mut *self.times.expect("should be initialized") };
        let t = times
            .get(frame.func_name)
            .expect("should contain by construction")
            + frame.instant.elapsed().as_micros();
        times.insert(frame.func_name, t);
        self.stack.pop();
        if self.stack.len() == 0 {
            let mut total = 0 as u128;
            for (_, value) in &*times {
                total += *value;
            }
            for (key, value) in &*times {
                println!(
                    "{} took {}, {}% of the run time",
                    key,
                    duration_get(*value),
                    duration_get(*value) / (total as f64 / 1_000_000.) * 100.0
                );
            }
        }
    }
}

static mut TIME_MAN: TimeManager = TimeManager {
    times: None,
    stack: vec![],
};

fn frame_push(frame: &ProfileFrame) {
    unsafe {
        TIME_MAN.frame_push(frame);
    }
}
fn frame_pop(frame: &ProfileFrame) {
    unsafe {
        TIME_MAN.frame_pop(frame);
    }
}
