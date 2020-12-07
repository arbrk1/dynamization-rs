//! Sorted `Vec`. Can be used as a priority queue or as an associative array. 
//! __Requires feature `sorted_vec`__.

use crate::*;

/// An opaque struct with an unspecified interface.
///
/// Obviously can't be used directly.
#[derive(Clone, Debug)]
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
                        unsafe { vec_ptr.add(i).write(x); }
                        maybe_x = a.next();
                        maybe_y = Some(y);
                    } else {
                        unsafe { vec_ptr.add(i).write(y); }
                        maybe_x = Some(x);
                        maybe_y = b.next();
                    }

                    i += 1;
                }

                (Some(x), None) => {
                    unsafe { vec_ptr.add(i).write(x); }
                    i += 1;
                    branch = Branch::A;
                    break;
                }
                
                (None, Some(y)) => {
                    unsafe { vec_ptr.add(i).write(y); }
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
                    unsafe { vec_ptr.add(i).write(x); }
                    i += 1;
                }
            }
            
            Branch::B => {
                for x in b {
                    unsafe { vec_ptr.add(i).write(x); }
                    i += 1;
                }
            }

            Branch::None => {}
        }

        assert!(i == vec.capacity());
        // Safety: the assertion above; also this assertion almost guarantees
        // that the memory accesses in the loops above (.add(i).write(...))
        // do not touch unallocated memory.
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


/// A priority queue based on a sorted vector.
///
/// Currently provides only basic operations.
#[derive(Clone, Debug)]
pub struct SVQueue<T, S = strategy::Binary> {
    dynamic: Dynamic<SortedVec<T>, S>,
    len: usize,
}


impl<T: Ord> SVQueue<T> {
    /// Uses the [`Binary`](strategy::Binary) strategy.
    pub fn new() -> Self {
        SVQueue {
            dynamic: Dynamic::new(),
            len: 0,
        }
    }

    /// Can be used as in 
    ///
    /// ```
    /// # use dynamization::sorted_vec::SVQueue;
    /// use dynamization::strategy;
    ///
    /// let pqueue = SVQueue::<i32>::with_strategy::<strategy::SkewBinary>();
    /// //                  ^^^^^^^ -- optional if the payload type can be inferred
    /// ```
    ///
    /// The [`SkewBinary`](strategy::SkewBinary) strategy has the fastest 
    /// [peeks](SVQueue::peek) and [deletions](SVQueue::pop).
    pub fn with_strategy<S: Strategy>() -> SVQueue<T, S> {
        SVQueue {
            dynamic: Dynamic::new(),
            len: 0,
        }
    }
}


impl<T: Ord, S: Strategy> SVQueue<T, S> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
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


/// An associative array based on a sorted vector.
///
/// Currently provides only basic operations.
#[derive(Clone, Debug)]
pub struct SVMap<K, V, S = strategy::Binary> {
    dynamic: Dynamic<SortedVec<SVPair<K, V>>, S>,
    len: usize,
}

#[derive(Clone, Debug)]
struct SVPair<K, V>(K, Option<V>);

impl<K: Ord, V> Ord for SVPair<K, V> {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<K: Ord, V> PartialOrd for SVPair<K, V> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<K: Ord, V> PartialEq for SVPair<K, V> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<K: Ord, V> Eq for SVPair<K, V> {}

impl<K: Ord, V> SVMap<K, V> {
    /// Uses the [`Binary`](strategy::Binary) strategy.
    pub fn new() -> Self {
        SVMap {
            dynamic: Dynamic::new(),
            len: 0,
        }
    }

    /// Can be used as in 
    ///
    /// ```
    /// # use dynamization::sorted_vec::SVMap;
    /// use dynamization::strategy;
    ///
    /// let svmap = SVMap::<String, i32>::with_strategy::<strategy::SkewBinary>();
    /// //               ^^^^^^^^^^^^^^^ -- optional if the payload type can be inferred
    /// ```
    pub fn with_strategy<S: Strategy>() -> SVMap<K, V, S> {
        SVMap {
            dynamic: Dynamic::new(),
            len: 0,
        }
    }
}


impl<K: Ord, V, S: Strategy> SVMap<K, V, S> {
    pub fn len(&self) -> usize {
        self.len
    }

    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    fn search<Q: Ord + ?Sized>(&self, key: &Q) -> Option<&SVPair<K, V>> where
        K: core::borrow::Borrow<Q> 
    {
        for unit in self.dynamic.units() {
            let vec = &unit.vec;

            if let Ok(index) = vec.binary_search_by(|entry| {
                entry.0.borrow().cmp(key)
            }) {
                // Safety: binary search
                return unsafe { Some(vec.get_unchecked(index)) }
            }
        }

        None
    }

    fn search_mut<Q: Ord + ?Sized>(&mut self, key: &Q) -> Option<&mut SVPair<K, V>> where
        K: core::borrow::Borrow<Q> 
    {
        for unit in self.dynamic.units_mut() {
            let vec = &mut unit.vec;

            if let Ok(index) = vec.binary_search_by(|entry| {
                entry.0.borrow().cmp(key)
            }) {
                // Safety: binary search
                return unsafe { Some(vec.get_unchecked_mut(index)) }
            }
        }

        None
    }

    pub fn get<Q: Ord + ?Sized>(&self, key: &Q) -> Option<&V> where
        K: core::borrow::Borrow<Q>
    {
        self.search(key).map(|entry| entry.1.as_ref()).flatten()
    }

    pub fn get_key_value<Q: Ord + ?Sized>(&self, key: &Q) -> Option<(&K, &V)> where
        K: core::borrow::Borrow<Q>
    {
        self.search(key).map(|entry| {
            match &entry.1 {
                Some(v) => { Some( (&entry.0, v) ) }
                None => { None }
            }
        }).flatten()
    }

    pub fn get_mut<Q: Ord + ?Sized>(&mut self, key: &Q) -> Option<&mut V> where
        K: core::borrow::Borrow<Q>
    {
        self.search_mut(key).map(|entry| entry.1.as_mut()).flatten()
    }

}


