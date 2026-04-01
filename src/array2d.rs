use std::{
    ops::{Index, IndexMut},
    sync::Arc,
};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Array2d<T> {
    values: Box<[T]>,
    width: usize,
    height: usize,
}

impl<T> Array2d<T> {
    pub fn from_array<const WIDTH: usize, const HEIGHT: usize>(
        values: [[T; WIDTH]; HEIGHT],
    ) -> Self {
        let flat: Vec<T> = values.into_iter().flatten().collect();
        Self {
            values: flat.into_boxed_slice(),
            width: WIDTH,
            height: HEIGHT,
        }
    }

    pub fn from_vec(v: Vec<T>, width: usize, height: usize) -> Self {
        assert!(v.len() == width * height);
        Self {
            values: v.into_boxed_slice(),
            width,
            height,
        }
    }

    pub fn new<V: Into<Box<[T]>>>(x: V, width: usize, height: usize) -> Self {
        let y = x.into();
        assert!(y.len() == width * height);
        Self {
            values: y,
            width,
            height,
        }
    }

    pub fn get(&self, idx: usize) -> &[T] {
        &self.values[self.width * idx..self.width * (idx + 1)]
    }

    pub fn get_at(&self, x: usize, y: usize) -> &T {
        &self.values[y * self.width + x]
    }

    pub fn get_mut(&mut self, idx: usize) -> &mut [T] {
        &mut self.values[self.width * idx..self.width * (idx + 1)]
    }

    pub fn get_at_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.values[y * self.width + x]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn to_shared(self) -> SharedArray2d<T> {
        SharedArray2d {
            values: self.values.into(),
            width: self.width,
            height: self.height,
        }
    }

    pub fn as_slice<'a>(&'a self) -> Slice2d<'a, T> {
        Slice2d {
            values: &self.values,
            width: self.width,
            height: self.height,
        }
    }

    pub fn as_slice_mut<'a>(&'a mut self) -> SliceMut2d<'a, T> {
        SliceMut2d {
            values: &mut self.values,
            width: self.width,
            height: self.height,
        }
    }

    //value, x, y
    pub fn kernel<U, FN: FnMut(&T, usize, usize) -> U>(&self, mut func: FN) -> Array2d<U> {
        let mut out = Vec::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                let tmp = func(&self[y][x], x, y);
                out.push(tmp);
            }
        }
        Array2d::new(out, self.width(), self.height())
    }

    //value, x, y
    pub fn kernel_mut<U, FN: FnMut(&mut T, usize, usize) -> U>(
        &mut self,
        mut func: FN,
    ) -> Array2d<U> {
        let mut out = Vec::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                let tmp = func(&mut self[y][x], x, y);
                out.push(tmp);
            }
        }
        Array2d::new(out, self.width(), self.height())
    }

    //value, x, y
    pub fn kernel_on<FN: FnMut(&mut T, usize, usize)>(&mut self, mut func: FN) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                func(&mut self[y][x], x, y);
            }
        }
    }

    pub fn data(&self) -> &[T] {
        &self.values
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.values
    }
}

impl<T> Index<usize> for Array2d<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

impl<T> IndexMut<usize> for Array2d<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Slice2d<'a, T> {
    values: &'a [T],
    width: usize,
    height: usize,
}

impl<'a, T> Slice2d<'a, T> {
    pub fn new(values: &'a [T], width: usize, height: usize) -> Self {
        assert!(values.len() == width * height);
        Self {
            values,
            width,
            height,
        }
    }
    pub fn get(&self, idx: usize) -> &[T] {
        &self.values[self.width * idx..self.width * (idx + 1)]
    }

    pub fn get_at(&self, x: usize, y: usize) -> &T {
        &self.values[y * self.width + x]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn as_slice(&self) -> Slice2d<'a, T> {
        Slice2d {
            values: &self.values,
            width: self.width,
            height: self.height,
        }
    }
    //value, x, y
    pub fn kernel<U, FN: FnMut(&T, usize, usize) -> U>(&self, mut func: FN) -> Array2d<U> {
        let mut out = Vec::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                let tmp = func(&self[y][x], x, y);
                out.push(tmp);
            }
        }
        Array2d::new(out, self.width(), self.height())
    }

    pub fn data(&self) -> &[T] {
        &self.values
    }
}

impl<'a, T> Index<usize> for Slice2d<'a, T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

#[derive(Debug)]
pub struct SliceMut2d<'a, T> {
    values: &'a mut [T],
    width: usize,
    height: usize,
}

impl<'a, T> SliceMut2d<'a, T> {
    pub fn new(values: &'a mut [T], width: usize, height: usize) -> Self {
        assert!(values.len() == width * height);
        Self {
            values,
            width,
            height,
        }
    }
    pub fn get(&self, idx: usize) -> &[T] {
        &self.values[self.width * idx..self.width * (idx + 1)]
    }

    pub fn get_at(&self, x: usize, y: usize) -> &T {
        &self.values[y * self.width + x]
    }

    pub fn get_mut(&mut self, idx: usize) -> &mut [T] {
        &mut self.values[self.width * idx..self.width * (idx + 1)]
    }

    pub fn get_at_mut(&mut self, x: usize, y: usize) -> &mut T {
        &mut self.values[y * self.width + x]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn as_slice<'b>(&'b self) -> Slice2d<'b, T> {
        Slice2d {
            values: self.values,
            width: self.width,
            height: self.height,
        }
    }

    pub fn as_slice_mut<'b>(&'b mut self) -> SliceMut2d<'b, T> {
        SliceMut2d {
            values: self.values,
            width: self.width,
            height: self.height,
        }
    }

    //value, x, y
    pub fn kernel<U, FN: FnMut(&T, usize, usize) -> U>(&self, mut func: FN) -> Array2d<U> {
        let mut out = Vec::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                let tmp = func(&self[y][x], x, y);
                out.push(tmp);
            }
        }
        Array2d::new(out, self.width(), self.height())
    }

    //value, x, y
    pub fn kernel_mut<U, FN: FnMut(&mut T, usize, usize) -> U>(
        &mut self,
        mut func: FN,
    ) -> Array2d<U> {
        let mut out = Vec::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                let tmp = func(&mut self[y][x], x, y);
                out.push(tmp);
            }
        }
        Array2d::new(out, self.width(), self.height())
    }

    //value, x, y
    pub fn kernel_on<FN: FnMut(&mut T, usize, usize)>(&mut self, mut func: FN) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                func(&mut self[y][x], x, y);
            }
        }
    }

    pub fn data(&self) -> &[T] {
        &self.values
    }

    pub fn data_mut(&mut self) -> &mut [T] {
        &mut self.values
    }
}

impl<'a, T> Index<usize> for SliceMut2d<'a, T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}

impl<'a, T> IndexMut<usize> for SliceMut2d<'a, T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SharedArray2d<T> {
    values: Arc<[T]>,
    width: usize,
    height: usize,
}

impl<T> Clone for SharedArray2d<T> {
    fn clone(&self) -> Self {
        Self {
            values: self.values.clone(),
            width: self.width,
            height: self.height,
        }
    }
}
impl<T> SharedArray2d<T> {
    pub fn from_array<const WIDTH: usize, const HEIGHT: usize>(
        values: [[T; WIDTH]; HEIGHT],
    ) -> Self {
        let flat: Vec<T> = values.into_iter().flatten().collect();
        Self {
            values: flat.into(),
            width: WIDTH,
            height: HEIGHT,
        }
    }

    pub fn from_vec(v: Vec<T>, width: usize, height: usize) -> Self {
        assert!(v.len() == width * height);
        Self {
            values: v.into(),
            width,
            height,
        }
    }

    pub fn new<V: Into<Box<[T]>>>(x: V, width: usize, height: usize) -> Self {
        let y = x.into();
        assert!(y.len() == width * height);
        Self {
            values: y.into(),
            width,
            height,
        }
    }

    pub fn get(&self, idx: usize) -> &[T] {
        &self.values[self.width * idx..self.width * (idx + 1)]
    }

    pub fn get_at(&self, x: usize, y: usize) -> &T {
        &self.values[y * self.width + x]
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn as_slice<'a>(&'a self) -> Slice2d<'a, T> {
        Slice2d {
            values: &self.values,
            width: self.width,
            height: self.height,
        }
    }

    //value, x, y
    pub fn kernel<U, FN: FnMut(&T, usize, usize) -> U>(&self, mut func: FN) -> Array2d<U> {
        let mut out = Vec::new();
        for y in 0..self.height() {
            for x in 0..self.width() {
                let tmp = func(&self[y][x], x, y);
                out.push(tmp);
            }
        }
        Array2d::new(out, self.width(), self.height())
    }
    pub fn data(&self) -> &[T] {
        &self.values
    }
}

impl<T> Index<usize> for SharedArray2d<T> {
    type Output = [T];
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index)
    }
}
