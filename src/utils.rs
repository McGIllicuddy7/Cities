use std::{
    alloc::Layout,
    sync::{Arc, Mutex, MutexGuard},
};

#[derive(Debug)]
pub struct ArenaData {
    pub start: *mut u8,
    pub count: usize,
    pub next_ptr: usize,
    pub next_arena: Option<Arena>,
    pub destructor_list: Vec<(*mut (), unsafe fn(*mut (), usize), usize)>,
}

#[derive(Clone, Debug)]
pub struct Arena {
    ptr: Arc<Mutex<ArenaData>>,
}
impl Arena {
    fn convenience_inner_lock<'a>(&'a self) -> MutexGuard<'a, ArenaData> {
        self.ptr.lock().unwrap()
    }

    pub fn alloc_bytes<'a>(&'a self, count: usize) -> *mut [u8] {
        let mut data = self.convenience_inner_lock();
        let mut count = count;
        if count % 16 != 0 {
            count += 16 - data.count % 16;
        }
        if data.next_ptr + count >= data.count {
            let x = if let Some(x) = data.next_arena.as_ref() {
                x
            } else {
                data.next_arena = Some(Arena::new_min_size(count));
                data.next_arena.as_ref().unwrap()
            };
            x.alloc_bytes(count)
        } else {
            let out_off = data.next_ptr;
            data.next_ptr += count;
            let out = unsafe { std::ptr::slice_from_raw_parts_mut(data.start.add(out_off), count) };
            out
        }
    }

    pub fn new_min_size(count: usize) -> Self {
        let mut to_alloc_size = 4096 * 4;
        while to_alloc_size < count {
            to_alloc_size *= 2;
        }
        let bytes =
            unsafe { std::alloc::alloc(Layout::from_size_align(to_alloc_size, 16).unwrap()) };
        let ptr = Arc::new(Mutex::new(ArenaData {
            start: bytes,
            count: to_alloc_size,
            next_ptr: 0,
            next_arena: None,
            destructor_list: Vec::new(),
        }));
        Self { ptr }
    }

    pub fn new() -> Self {
        let to_alloc_size = 4096 * 4;
        let bytes =
            unsafe { std::alloc::alloc(Layout::from_size_align(to_alloc_size, 16).unwrap()) };
        let ptr = Arc::new(Mutex::new(ArenaData {
            start: bytes,
            count: to_alloc_size,
            next_ptr: 0,
            next_arena: None,
            destructor_list: Vec::new(),
        }));
        Self { ptr }
    }

    pub fn alloc<'a, T: Send>(&'a self, val: T) -> &'a mut T {
        let bytes = self.alloc_bytes(size_of_val(&val));
        let ptr: *mut T = bytes.cast();
        unsafe {
            let f: (*mut (), unsafe fn(*mut (), usize), usize) = (
                ptr as *mut (),
                drop_ptr::<T> as unsafe fn(*mut (), usize),
                1,
            );
            self.convenience_inner_lock().destructor_list.push(f);
            self.convenience_inner_lock().destructor_list.push(f);
            ptr.write(val);
            ptr.as_mut().unwrap()
        }
    }

    pub fn alloc_slice<'a, T: Send + Clone>(&'a self, val: &[T]) -> &'a mut [T] {
        let bytes = self.alloc_bytes(size_of_val(&val));
        let ptr: *mut [T] = std::ptr::slice_from_raw_parts_mut(bytes.cast(), val.len());
        unsafe {
            let f: (*mut (), unsafe fn(*mut (), usize), usize) = (
                ptr as *mut (),
                drop_ptr::<T> as unsafe fn(*mut (), usize),
                val.len(),
            );
            self.convenience_inner_lock().destructor_list.push(f);
            for i in 0..val.len() {
                (ptr as *mut T).add(i).write(val[i].clone());
            }
            ptr.as_mut().unwrap()
        }
    }
    pub fn alloc_buffer<'a, T: Send + Default>(&'a self, count: usize) -> &'a mut [T] {
        unsafe {
            let bytes = self.alloc_bytes(size_of::<T>() * count);
            let ptr: *mut [T] = std::ptr::slice_from_raw_parts_mut(bytes.cast(), count);

            let f: (*mut (), unsafe fn(*mut (), usize), usize) = (
                ptr as *mut (),
                drop_ptr::<T> as unsafe fn(*mut (), usize),
                count,
            );
            self.convenience_inner_lock().destructor_list.push(f);
            for i in 0..count {
                (ptr as *mut T).add(i).write(T::default());
            }
            ptr.as_mut().unwrap()
        }
    }
    pub fn alloc_buffer_with_value<'a, T: Send + Clone>(
        &'a self,
        val: &T,
        count: usize,
    ) -> &'a mut [T] {
        unsafe {
            let bytes = self.alloc_bytes(size_of::<T>() * count);
            let ptr: *mut [T] = std::ptr::slice_from_raw_parts_mut(bytes.cast(), count);

            let f: (*mut (), unsafe fn(*mut (), usize), usize) = (
                ptr as *mut (),
                drop_ptr::<T> as unsafe fn(*mut (), usize),
                count,
            );
            self.convenience_inner_lock().destructor_list.push(f);
            for i in 0..count {
                (ptr as *mut T).add(i).write(val.clone());
            }
            ptr.as_mut().unwrap()
        }
    }
    pub fn concat_buffers<'a, T: Clone>(&'a self, start: &[T], remainder: &[T]) -> &'a mut [T] {
        unsafe {
            let count = start.len() + remainder.len();
            let bytes = self.alloc_bytes(size_of::<T>() * count);
            let ptr: *mut [T] = std::ptr::slice_from_raw_parts_mut(bytes.cast(), count);

            let f: (*mut (), unsafe fn(*mut (), usize), usize) = (
                ptr as *mut (),
                drop_ptr::<T> as unsafe fn(*mut (), usize),
                count,
            );
            self.convenience_inner_lock().destructor_list.push(f);
            for i in 0..start.len() {
                (ptr as *mut T).add(i).write(start[i].clone());
            }
            for i in 0..remainder.len() {
                (ptr as *mut T)
                    .add(start.len() + i)
                    .write(remainder[i].clone());
            }
            ptr.as_mut().unwrap()
        }
    }

    pub fn concat_strs<'a>(&'a self, a: &str, b: &str) -> &'a mut str {
        let bytes = self.concat_buffers(a.as_bytes(), b.as_bytes());
        std::str::from_utf8_mut(bytes).unwrap()
    }
}
unsafe fn drop_ptr<T>(v: *mut (), count: usize) {
    let x = v as *mut T;
    unsafe {
        for i in 0..count {
            std::ptr::drop_in_place(x.add(i));
        }
    }
}
impl Drop for ArenaData {
    fn drop(&mut self) {
        unsafe {
            for (ptr, func, count) in &mut self.destructor_list {
                (*func)(*ptr, *count);
            }
            std::alloc::dealloc(
                self.start,
                std::alloc::Layout::from_size_align(self.count, 16).unwrap(),
            );
        }
    }
}
