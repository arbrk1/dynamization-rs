//! Sorted `Vec`. Can be used as a priority queue or as an associative array.

use crate::*;

pub struct SortedVec<T> {
    vec: Vec<T>,
}

enum Branch {
    A, B, None
}

impl<T: Ord> Static for SortedVec<T> {
    type Payload = T;

    fn singleton(payload: T) -> Self {
        SortedVec {
            vec: vec![payload],
        }
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


impl<T: Ord> Dynamic<SortedVec<T>> {
    pub fn push(&mut self, payload: T) {
        self.insert(payload);
    }
}


