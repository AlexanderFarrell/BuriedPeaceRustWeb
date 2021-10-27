use std::marker::PhantomData;
use crate::which;
use js_sys::get_index;

pub struct FloatVec<T> {
    pub contents: Vec<(f32, T)>
}

impl<T> FloatVec<T> {
    pub fn new(contents: Vec<(f32, T)>) -> Self {
        Self {
            contents
        }
    }

    pub fn get(&self, index_float: f32) -> Option<&T> {
        match get_index(index_float) {
            Some(index) => {
                self.contents.get(index)
            }
            None => {None}
        }
    }

    pub fn get_mut(&mut self, index_float: f32) -> Option<&mut T> {
        match get_index(index_float) {
            Some(index) => {
                self.contents.get_mut(index)
            }
            None => {None}
        }
    }

    #[inline]
    fn get_index(&self, index_float: f32) -> Option<usize> {
        return if self.contents.is_empty() {
            None
        } else {
            i: usize = 0;
            len: usize = self.contents.len();

            while i < len {
                let item: f32 = self.contents.get(i).unwrap().0;
                if item < index_float {
                    i += 1;
                } else {
                    return Some(i)
                }
            }
            Some(i)
        }
    }
}