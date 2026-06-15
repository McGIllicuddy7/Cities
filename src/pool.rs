use std::{
    any::TypeId,
    cell::UnsafeCell,
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

use serde::{Deserialize, Serialize, de::DeserializeOwned};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Ptr<T: ?Sized> {
    guard: *const Mutex<PoolObjectData>,
    value: *const UnsafeCell<T>,
    offset: u64,
    generation: u64,
    type_info: &'static TypeInfo,
}

impl<T> Ptr<T> {
    pub const fn null() -> Self {
        Self {
            guard: std::ptr::null_mut(),
            value: std::ptr::null_mut(),
            generation: 0,
            offset: 0,
            type_info: &INVALID_TYPE_INFO,
        }
    }
}
unsafe impl<T: Send + Sync + 'static> Send for Ptr<T> {}
unsafe impl<T: Send + Sync + 'static> Sync for Ptr<T> {}
impl<T: ?Sized> Ptr<T> {
    pub fn lock<'a>(&'a self) -> Result<LifeGuard<'a, T>, TryLockPtrError> {
        unsafe {
            if self.guard.is_null() || self.value.is_null() {
                return Err(TryLockPtrError::IsNull);
            }
            let v0 = (*self.guard).lock().unwrap();
            if !v0.is_init {
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
            if !v0.is_init || v0.generation != self.generation {
                return Err(TryLockPtrError::IsNotAllocated);
            }
            Ok(LifeGuard {
                _guard: v0,
                rf: &mut *(*self.value).get(),
            })
        }
    }

    pub fn _internal_get_ptrs(
        &self,
    ) -> (
        *const Mutex<PoolObjectData>,
        *const UnsafeCell<T>,
        u64,
        u64,
        &'static TypeInfo,
    ) {
        (
            self.guard,
            self.value,
            self.offset,
            self.generation,
            self.type_info,
        )
    }

    pub unsafe fn _internal_construct(
        guard: *const Mutex<PoolObjectData>,
        ptr: *const UnsafeCell<T>,
        offset: u64,
        generation: u64,
        type_info: &'static TypeInfo,
    ) -> Self {
        Self {
            guard,
            value: ptr,
            offset,
            generation,
            type_info,
        }
    }
}
impl<T: Pooled> Ptr<T> {
    pub fn new(x: T) -> Self {
        T::alloc_pooled(x)
    }
    pub fn delete(&self) {
        unsafe {
            let mut tmp = (*self.guard).lock().unwrap();
            if !tmp.is_init || tmp.generation != self.generation {
                return;
            }
            (*self.value).get().drop_in_place();
            tmp.is_init = false;
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

#[derive(Debug)]
pub struct LifeGuard<'a, T: ?Sized> {
    _guard: MutexGuard<'a, PoolObjectData>,
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
    ( $to_kind:ty,$base:expr) => {
        unsafe {
            let (guard, ptr, idx, generation, info) = $base._internal_get_ptrs();
            let ptr = (*ptr).get() as *const $to_kind as *const std::cell::UnsafeCell<$to_kind>;
            crate::pool::Ptr::_internal_construct(guard, ptr, idx, generation, info)
        }
    };
}

pub const POOL_OBJECT_COUNT: usize = 4096;
#[macro_export]
macro_rules! make_pooled {
    ($type:ty) => {
        impl crate::pool::Pooled for $type {
            fn get_type_info() -> &'static crate::pool::TypeInfo {
                static TYPE_INFO: crate::pool::TypeInfo = crate::pool::TypeInfo {
                    type_id: std::any::TypeId::of::<$type>(),
                    size: std::mem::size_of::<$type>() as u64,
                };
                return &TYPE_INFO;
            }
            fn get_object_pool() -> &'static crate::pool::MemoryPool<$type> {
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
                static GUARD_LIST: [Mutex<crate::pool::PoolObjectData>;
                    crate::pool::POOL_OBJECT_COUNT] = [const {
                    std::sync::Mutex::new(crate::pool::PoolObjectData {
                        generation: 0,
                        is_init: false,
                    })
                }; _];
                static POOL: crate::pool::MemoryPool<$type> = crate::pool::MemoryPool {
                    backing: &LIST.v,
                    data: &GUARD_LIST,
                };
                &POOL
            }
        }
    };
    ($type:ty, $object_count:expr) => {
        impl crate::pool::Pooled for $type {
            fn get_type_info() -> &'static crate::pool::TypeInfo {
                static TYPE_INFO: crate::pool::TypeInfo = crate::pool::TypeInfo {
                    type_id: std::any::TypeId::of::<$type>(),
                    size: std::mem::size_of::<$type>() as u64,
                };
                return &TYPE_INFO;
            }
            fn get_object_pool() -> &'static crate::pool::MemoryPool<$type> {
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
                static GUARD_LIST: [std::sync::Mutex<crate::pool::PoolObjectData>; $object_count] = [const {
                    std::sync::Mutex::new(crate::pool::PoolObjectData {
                        generation: 0,
                        is_init: false,
                    })
                };
                    _];
                static POOL: crate::pool::MemoryPool<$type> = crate::pool::MemoryPool {
                    backing: &LIST.v,
                    data: &GUARD_LIST,
                };
                &POOL
            }
        }
    };
}

pub trait Pooled: Sized + Send + Sync + 'static {
    fn get_type_info() -> &'static TypeInfo;
    fn get_object_pool() -> &'static MemoryPool<Self>;
    fn alloc_pooled(v: Self) -> Ptr<Self> {
        unsafe {
            let obj_pool = Self::get_object_pool();
            let (pool, locks) = (obj_pool.backing, obj_pool.data);
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
                if g.is_init {
                    continue;
                }
                g.is_init = true;
                g.generation = g.generation.wrapping_add(1);
                let ptr = pool[i].as_mut_ptr();
                ptr.write(PoolEntry {
                    value: UnsafeCell::new(v),
                });
                return Ptr {
                    guard: guard,
                    value: &(*ptr).value,
                    generation: g.generation,
                    offset: i as u64,
                    type_info: Self::get_type_info(),
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

#[macro_export]
macro_rules! new {
    ($T:ty, $value:expr) => {
        crate::pool::Ptr::<$T>::new($value)
    };
    ($value:expr) => {
        crate::pool::Ptr::new($value)
    };
}

#[macro_export]
macro_rules! delete {
    ($value:expr) => {
        $value.delete();
    };
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TypeInfo {
    pub type_id: TypeId,
    pub size: u64,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct PoolObjectData {
    pub is_init: bool,
    pub generation: u64,
}
pub static INVALID_TYPE_INFO: TypeInfo = TypeInfo {
    type_id: std::any::TypeId::of::<()>(),
    size: 0,
};

pub trait DynPool: Send + Sync + 'static {
    fn serialize(&self) -> Result<Vec<(Option<Vec<u8>>, u64)>, rmp_serde::encode::Error>;
    fn deserialize(
        &self,
        bytes: Vec<(Option<Vec<u8>>, u64)>,
    ) -> Result<(), rmp_serde::decode::Error>;
    fn reset(&self);
}
#[derive(Clone, Copy, Debug)]
pub struct MemoryPool<T: 'static + Send + Sync> {
    pub backing: &'static std::cell::UnsafeCell<[std::mem::MaybeUninit<crate::pool::PoolEntry<T>>]>,
    pub data: &'static [std::sync::Mutex<crate::pool::PoolObjectData>],
}

impl<T: 'static + Send + Sync + Serialize + DeserializeOwned + Pooled> DynPool for MemoryPool<T> {
    fn serialize(&self) -> Result<Vec<(Option<Vec<u8>>, u64)>, rmp_serde::encode::Error> {
        unsafe {
            let mut out = Vec::new();
            for i in 0..self.data.len() {
                let guard = self.data[i].lock().unwrap();
                if !guard.is_init {
                    out.push((None, guard.generation));
                } else {
                    let tmp1 = &*(*self.backing.get())[i].as_ptr();
                    let tmp2 = &*tmp1.value.get();
                    let bytes = rmp_serde::to_vec(tmp2)?;
                    out.push((Some(bytes), guard.generation));
                }
            }
            Ok(out)
        }
    }

    fn deserialize(
        &self,
        bytes: Vec<(Option<Vec<u8>>, u64)>,
    ) -> Result<(), rmp_serde::decode::Error> {
        unsafe {
            let mut list: Vec<(Option<T>, u64)> = Vec::new();
            for (i, generation) in bytes {
                let Some(b) = i else {
                    list.push((None, generation));
                    continue;
                };
                let v1: T = rmp_serde::from_slice(&b)?;
                list.push((Some(v1), generation));
            }

            for (idx, (value, generation)) in list.into_iter().enumerate() {
                let mut g0 = self.data[idx].lock().unwrap();
                if g0.is_init {
                    std::ptr::drop_in_place((*(*self.backing.get())[idx].as_ptr()).value.get());
                }
                g0.generation = generation;
                g0.is_init = value.is_some();
                if value.is_none() {
                    continue;
                }
                (*(*self.backing.get())[idx].as_ptr())
                    .value
                    .get()
                    .write(value.unwrap());
            }
            Ok(())
        }
    }
    fn reset(&self) {
        unsafe {
            for idx in 0..self.data.len() {
                let mut g0 = self.data[idx].lock().unwrap();
                if g0.is_init {
                    std::ptr::drop_in_place((*(*self.backing.get())[idx].as_ptr()).value.get());
                }
                g0.is_init = false;
                g0.generation = 0;
            }
        }
    }
}

#[derive(Clone, Copy)]
pub struct TypeRegistryEntry {
    pub type_info: &'static TypeInfo,
    pub data: &'static dyn DynPool,
}

pub struct TypeRegistry {
    pub data: Mutex<Option<HashMap<&'static str, TypeRegistryEntry>>>,
}

impl TypeRegistry {
    pub const fn new() -> Self {
        Self {
            data: Mutex::new(None),
        }
    }
    pub fn with<U>(
        &self,
        func: impl FnOnce(&mut HashMap<&'static str, TypeRegistryEntry>) -> U,
    ) -> U {
        let mut guard = self.data.lock().unwrap();
        if guard.is_none() {
            *guard = Some(HashMap::new());
        };
        func(&mut guard.as_mut().unwrap())
    }

    pub fn register_type<T: Pooled + Serialize + DeserializeOwned>(&self) {
        self.with(|map| {
            map.insert(
                std::any::type_name::<T>(),
                TypeRegistryEntry {
                    data: T::get_object_pool(),
                    type_info: T::get_type_info(),
                },
            );
        });
    }

    pub fn save(
        &self,
    ) -> Result<HashMap<Arc<str>, Vec<(Option<Vec<u8>>, u64)>>, rmp_serde::encode::Error> {
        self.with(|map| {
            let mut table: HashMap<Arc<str>, Vec<(Option<Vec<u8>>, u64)>> = HashMap::new();
            for (name, i) in map {
                table.insert((*name).into(), i.data.serialize()?);
            }
            Ok(table)
        })
    }

    pub fn load(
        &self,
        from: HashMap<Arc<str>, Vec<(Option<Vec<u8>>, u64)>>,
    ) -> Result<(), rmp_serde::decode::Error> {
        let old = self.save().unwrap();
        let tri: Result<(), rmp_serde::decode::Error> = self.with(|map| {
            for (name, i) in map {
                if let Some(data) = from.get(*name) {
                    i.data.deserialize(data.clone())?;
                } else {
                    i.data.reset();
                }
            }
            Ok(())
        });
        if tri.is_err() {
            self.load(old).unwrap();
        }
        tri
    }

    pub fn reset_all_pools(&self) {
        self.with(|i| {
            for (_, i) in i {
                i.data.reset();
            }
        })
    }
}
unsafe impl<T: Pooled> Send for MemoryPool<T> {}
unsafe impl<T: Pooled> Sync for MemoryPool<T> {}

#[macro_export]
macro_rules! register_types {
    ($($values:ty $(,)?)*) => {
       $(
            crate::pool::TYPE_REGISTRY.register_type::<$values>();
       )*
    };
}

pub static TYPE_REGISTRY: TypeRegistry = TypeRegistry::new();

pub fn save_pool_state()
-> Result<HashMap<Arc<str>, Vec<(Option<Vec<u8>>, u64)>>, rmp_serde::encode::Error> {
    TYPE_REGISTRY.save()
}

pub fn load_pool_state(
    state: HashMap<Arc<str>, Vec<(Option<Vec<u8>>, u64)>>,
) -> Result<(), rmp_serde::decode::Error> {
    TYPE_REGISTRY.load(state)
}
