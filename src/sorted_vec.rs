//! Sorted `Vec`. Can be used as a priority queue or as an associative array. 
//! __Requires feature `sorted_vec`__.
//!
//! Currently only a priority-queue version is implemented.

use crate::*;

pub struct SortedVec<T> {
    vec: Vec<T>,
}

enum Branch {
    A, B, None
}

impl<T: Ord> Static for SortedVec<T> {
    fn len(&self) -> usize {
        self.vec.len()
    }

    fn merge_with(self, other: Self) -> Self {
        let a = self.vec;
        let b = other.vec;

        let mut vec: Vec<T> = Vec::with_capacity(a.len() + b.len());
        
        let vec_ptr = vec.as_mut_ptr();
        let mut i = 0;

        let mut a = a.into_iter();
        let mut b = b.into_iter();

        let mut maybe_x = a.next();
        let mut maybe_y = b.next();

        let branch;

        loop {
            match (maybe_x, maybe_y) {
                (Some(x), Some(y)) => {
                    if x < y {
                        unsafe { *vec_ptr.add(i) = x; }
                        maybe_x = a.next();
                        maybe_y = Some(y);
                    } else {
                        unsafe { *vec_ptr.add(i) = y; }
                        maybe_x = Some(x);
                        maybe_y = b.next();
                    }

                    i += 1;
                }

                (Some(x), None) => {
                    unsafe { *vec_ptr.add(i) = x; }
                    i += 1;
                    branch = Branch::A;
                    break;
                }
                
                (None, Some(y)) => {
                    unsafe { *vec_ptr.add(i) = y; }
                    i += 1;
                    branch = Branch::B;
                    break;
                }

                (None, None) => { 
                    branch = Branch::None;
                    break; 
                }
            }
        }

        match branch {
            Branch::A => {
                for x in a {
                    unsafe { *vec_ptr.add(i) = x; }
                    i += 1;
                }
            }
            
            Branch::B => {
                for x in b {
                    unsafe { *vec_ptr.add(i) = x; }
                    i += 1;
                }
            }

            Branch::None => {}
        }

        unsafe { vec.set_len(i); }

        SortedVec { vec }
    }
}

impl<T> Singleton for SortedVec<T> {
    type Item = T;
    
    fn singleton(item: Self::Item) -> Self {
        SortedVec { vec: vec![item] }
    }
}


pub struct SVQueue<T> {
    dynamic: Dynamic<SortedVec<T>>,
    len: usize,
}


impl<T: Ord> SVQueue<T> {
    pub fn new() -> Self {
        SVQueue {
            dynamic: Dynamic::new(),
            len: 0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn push(&mut self, item: T) {
        self.dynamic.insert(item);
        self.len += 1;
    }

    pub fn peek(&self) -> Option<&T> {
        self.dynamic.units()
            .filter_map(|unit| unit.vec.last())
            .max()
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.dynamic.units_mut()
            .filter_map(|unit| unit.vec.last_mut())
            .max()
    }

    pub fn pop(&mut self) -> Option<T> {
        let best_unit = self.dynamic.units_mut()
            .max_by(|u1, u2| {
                u1.vec.last().cmp(&u2.vec.last())
            });

        match best_unit {
            None => None,

            Some(unit) => {
                match unit.vec.pop() {
                    None => None,

                    Some(x) => {
                        self.len -= 1;
                        Some(x)
                    }
                }
            }
        }
    }
}



