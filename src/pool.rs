use std::{
    cell::UnsafeCell,
    sync::{Mutex, MutexGuard},
};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Ptr<T: ?Sized> {
    guard: *const Mutex<bool>,
    value: *const UnsafeCell<T>,
}

impl<T> Ptr<T> {
    pub const fn null() -> Self {
        Self {
            guard: std::ptr::null_mut(),
            value: std::ptr::null_mut(),
        }
    }

    pub fn lock<'a>(&'a self) -> Result<LifeGuard<'a, T>, TryLockPtrError> {
        unsafe {
            if self.guard.is_null() || self.value.is_null() {
                return Err(TryLockPtrError::IsNull);
            }
            let v0 = (*self.guard).lock().unwrap();
            if !*v0 {
                return Err(TryLockPtrError::IsNotAllocated);
            }
            Ok(LifeGuard {
                _guard: v0,
                rf: &mut *(*self.value).get(),
            })
        }
    }

    pub fn try_lock<'a>(&'a self) -> Result<LifeGuard<'a, T>, TryLockPtrError> {
        unsafe {
            if self.guard.is_null() || self.value.is_null() {
                return Err(TryLockPtrError::IsNull);
            }
            let v0 = match (*self.guard).try_lock() {
                Ok(x) => x,
                Err(x) => match x {
                    std::sync::TryLockError::Poisoned(x) => x.into_inner(),
                    std::sync::TryLockError::WouldBlock => return Err(TryLockPtrError::WouldBlock),
                },
            };
            if !*v0 {
                return Err(TryLockPtrError::IsNotAllocated);
            }
            Ok(LifeGuard {
                _guard: v0,
                rf: &mut *(*self.value).get(),
            })
        }
    }

    pub fn _internal_get_ptrs(&self) -> (*const Mutex<bool>, *const UnsafeCell<T>) {
        (self.guard, self.value)
    }

    pub unsafe fn _internal_construct(guard: *mut Mutex<bool>, ptr: *mut UnsafeCell<T>) -> Self {
        Self { guard, value: ptr }
    }
}
impl<T: Pooled> Ptr<T> {
    pub fn new(x: T) -> Self {
        T::alloc_pooled(x)
    }
    pub fn delete(&self) {
        unsafe {
            let mut tmp = (*self.guard).lock().unwrap();
            if !*tmp {
                return;
            }
            (*self.value).get().drop_in_place();
            *tmp = false;
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum TryLockPtrError {
    WouldBlock,
    IsNull,
    IsNotAllocated,
}
impl std::fmt::Display for TryLockPtrError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}
impl std::error::Error for TryLockPtrError {}
pub struct LifeGuard<'a, T: ?Sized> {
    _guard: MutexGuard<'a, bool>,
    rf: &'a mut T,
}

impl<'a, T: ?Sized> LifeGuard<'a, T> {
    pub fn read(&self) -> &T {
        self.rf
    }

    pub fn write(&mut self) -> &mut T {
        self.rf
    }
}

#[macro_export]
macro_rules! ptr_cast {
    ($base:expr, $to_kind:ty) => {
        unsafe {
            let (guard, ptr) = $base._internal_get_ptrs();
            let ptr = ptr as *mut _ as *mut $to_kind as *mut std::cell::UnsafeCell<$to_kind>;
            crate::pool::Ptr::_internal_construct(guard, ptr)
        }
    };
}

pub const POOL_OBJECT_COUNT: usize = 4096;
#[macro_export]
macro_rules! make_pooled {
    ($type:ty) => {
        impl crate::pool::Pooled for $type {
            fn get_object_pool() -> (
                &'static std::cell::UnsafeCell<
                    [std::mem::MaybeUninit<crate::pool::PoolEntry<Self>>],
                >,
                &'static [std::sync::Mutex<bool>],
            ) {
                struct LT {
                    v: std::cell::UnsafeCell<
                        [std::mem::MaybeUninit<crate::pool::PoolEntry<$type>>;
                            crate::pool::POOL_OBJECT_COUNT],
                    >,
                }
                unsafe impl Send for LT {}
                unsafe impl Sync for LT {}
                static LIST: LT = LT {
                    v: std::cell::UnsafeCell::new([const { std::mem::MaybeUninit::zeroed() }; _]),
                };
                static GUARD_LIST: [Mutex<bool>; crate::pool::POOL_OBJECT_COUNT] =
                    [const { std::sync::Mutex::new(false) }; _];
                (&LIST.v, &GUARD_LIST)
            }
        }
    };
}

pub trait Pooled: Sized + Send + Sync + 'static {
    fn get_object_pool() -> (
        &'static UnsafeCell<[std::mem::MaybeUninit<PoolEntry<Self>>]>,
        &'static [Mutex<bool>],
    );
    fn alloc_pooled(v: Self) -> Ptr<Self> {
        unsafe {
            let (pool, locks) = Self::get_object_pool();
            let pool = &mut *pool.get();
            for i in 0..pool.len() {
                let guard = &locks[i];
                let mut g = match guard.try_lock() {
                    Ok(x) => x,
                    Err(e) => match e {
                        std::sync::TryLockError::Poisoned(x) => x.into_inner(),
                        std::sync::TryLockError::WouldBlock => {
                            continue;
                        }
                    },
                };
                if *g {
                    continue;
                }
                *g = true;
                let ptr = pool[i].as_mut_ptr();
                ptr.write(PoolEntry {
                    value: UnsafeCell::new(v),
                });
                return Ptr {
                    guard: guard,
                    value: &(*ptr).value,
                };
            }
            Ptr::null()
        }
    }
}

pub struct PoolEntry<T> {
    value: UnsafeCell<T>,
}

impl<T> PoolEntry<T> {
    pub const unsafe fn new() -> Self {
        Self {
            value: UnsafeCell::new(unsafe { std::mem::MaybeUninit::zeroed().assume_init() }),
        }
    }
}
unsafe impl<T: Send + Sync + 'static> Send for PoolEntry<T> {}
unsafe impl<T: Send + Sync + 'static> Sync for PoolEntry<T> {}
make_pooled!(u8);
make_pooled!(u16);
make_pooled!(u32);
make_pooled!(u64);
make_pooled!(i8);
make_pooled!(i16);
make_pooled!(i32);
make_pooled!(i64);
make_pooled!(String);

#[macro_export]
macro_rules! new {
    ($T:ty, $value:expr) => {
        $T::alloc_pooled($value)
    };
}

#[macro_export]
macro_rules! delete {
    ($value:expr) => {
        $value.delete();
    };
}
