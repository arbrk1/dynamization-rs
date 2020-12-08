//! Sorted `Vec`. Can be used as a priority queue or as an associative array. 
//! __Requires feature `sorted_vec`__.
//!
//! Defines an opaque [`SortedVec`] type and two containers:
//! * [`SVQueue`] analogous to [`BinaryHeap`](alloc::collections::BinaryHeap)
//! * [`SVMap`] analogous to [`BTreeMap`](alloc::collections::BTreeMap)


use crate::*;
use alloc::vec;

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

impl<T: Ord> core::iter::FromIterator<T> for SortedVec<T> {
    fn from_iter<I: IntoIterator<Item=T>>(iter: I) -> Self {
        let mut vec = iter.into_iter().collect::<Vec<_>>();
        
        vec.sort();

        SortedVec { vec }
    }
}


/// A max-priority queue based on a sorted vector.
///
/// Currently provides only basic operations.
///
/// Has slow insertions (4-8 times slower than those of 
/// [`BinaryHeap`](`alloc::collections::BinaryHeap`)) but fast deletions 
/// (2-3 times faster then [`BinaryHeap`](alloc::collections::BinaryHeap) ones).
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
    /// # #[cfg(feature="sorted_vec")] {
    /// # use dynamization::sorted_vec::SVQueue;
    /// use dynamization::strategy;
    ///
    /// let pqueue = SVQueue::<i32>::with_strategy::<strategy::SkewBinary>();
    /// //                  ^^^^^^^ -- optional if the payload type can be inferred
    /// # }
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
    /// Returns the number of elements currently stored.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `self.len() == 0`.
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }

    /// Inserts a new item into the container.
    pub fn push(&mut self, item: T) {
        self.dynamic.insert(item);
        self.len += 1;
    }

    /// Returns the current maximum.
    pub fn peek(&self) -> Option<&T> {
        self.dynamic.units()
            .filter_map(|unit| unit.vec.last())
            .max()
    }

    /// Exclusively returns the current maximum.
    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.dynamic.units_mut()
            .filter_map(|unit| unit.vec.last_mut())
            .max()
    }

    /// Removes the current maximum from the container.
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

#[test]
fn test_svqueue_len() {
    let some_numbers = vec![1,4,6,2,1,5,7,4,3,2,7,8];

    let mut pqueue = SVQueue::new();
    let mut counter = 0;

    for x in some_numbers { 
        pqueue.push(x);
        counter += 1;

        assert_eq!(pqueue.len(), counter);
        assert_eq!(pqueue.len(), pqueue.dynamic.len());
    }

    while !pqueue.is_empty() {
        pqueue.pop();
        counter -= 1;

        assert_eq!(pqueue.len(), counter);
        assert_eq!(pqueue.len(), pqueue.dynamic.len());
    }
}



/// An associative array based on a sorted vector.
///
/// Currently provides only basic operations.
///
/// Much slower than [`BTreeMap`](`alloc::collections::BTreeMap`) so useful 
/// only for nonpractical purposes (mainly as an example of implementing 
/// a dynamized container).
#[derive(Clone, Debug)]
pub struct SVMap<K, V, S = strategy::Binary> {
    dynamic: Dynamic<SortedVec<SVPair<K, V>>, S>,
    len: usize,
    free_count: usize,
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
            free_count: 0,
        }
    }

    /// Can be used as in 
    ///
    /// ```
    /// # #[cfg(feature="sorted_vec")] {
    /// # use dynamization::sorted_vec::SVMap;
    /// use dynamization::strategy;
    ///
    /// let svmap = SVMap::<String, i32>::with_strategy::<strategy::SkewBinary>();
    /// //               ^^^^^^^^^^^^^^^ -- optional if the payload type can be inferred
    /// # }
    /// ```
    pub fn with_strategy<S: Strategy>() -> SVMap<K, V, S> {
        SVMap {
            dynamic: Dynamic::new(),
            len: 0,
            free_count: 0,
        }
    }
}


impl<K: Ord, V, S: Strategy> SVMap<K, V, S> {
    /// Returns the number of elements currently stored.
    pub fn len(&self) -> usize {
        self.len
    }

    /// Returns `self.len() == 0`.
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
                // Safety: binary search returned Ok
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
                // Safety: binary search returned Ok
                return unsafe { Some(vec.get_unchecked_mut(index)) }
            }
        }

        None
    }
    
    /// Searches for an item with a specified key. 
    ///
    /// Returns `true` if the item is found.
    pub fn contains_key<Q: Ord + ?Sized>(&self, key: &Q) -> bool where
        K: core::borrow::Borrow<Q>
    {
        self.search(key).is_some()
    }

    /// Searches for an item with a specified key. 
    ///
    /// Returns a shared reference to the item found or `None`.
    pub fn get<Q: Ord + ?Sized>(&self, key: &Q) -> Option<&V> where
        K: core::borrow::Borrow<Q>
    {
        self.search(key).map(|entry| entry.1.as_ref()).flatten()
    }

    /// Searches for an item with a specified key. 
    ///
    /// Returns shared references to the key and the value stored or `None` if 
    /// the item has not been found.
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

    /// Searches for an item with a specified key. 
    ///
    /// Returns an exclusive reference to the item found or `None`.
    pub fn get_mut<Q: Ord + ?Sized>(&mut self, key: &Q) -> Option<&mut V> where
        K: core::borrow::Borrow<Q>
    {
        self.search_mut(key).map(|entry| entry.1.as_mut()).flatten()
    }

    /// Inserts a new key-value pair into the map.
    ///
    /// Returns the old value if it has been present. __Does not__ update 
    /// the key in such a case!
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        if let Some(entry) = self.search_mut(&key) {
            let result = core::mem::replace(&mut entry.1, Some(value));

            if let None = result {
                self.len += 1;
                self.free_count -= 1;
            }

            result
        } else {
            self.dynamic.insert(SVPair(key, Some(value)));
            self.len += 1;
            None
        }
    }
   

    const REBUILD_THRESHOLD: usize = 16;
   
    /// Removes an item from the container.
    ///
    /// Returns the item removed or `None` if the item has not been found.
    pub fn remove<Q: Ord + ?Sized>(&mut self, key: &Q) -> Option<V> where
        K: core::borrow::Borrow<Q>
    {
        if let Some(entry) = self.search_mut(&key) {
            let result = core::mem::replace(&mut entry.1, None);

            if let Some(_) = result {
                self.len -= 1;
                self.free_count += 1;
            }

            if self.free_count > self.len && self.len > Self::REBUILD_THRESHOLD {
                let mut tmp = SVMap::<K, V>::with_strategy::<S>();

                core::mem::swap(self, &mut tmp);

                *self = tmp.into_iter().collect();
            }

            result
        } else { None }
    }
    
   
    /// Removes all elements from the map.
    pub fn clear(&mut self) {
        self.dynamic.clear();
        self.len = 0;
        self.free_count = 0;
    }
}

impl<K: Ord, V, S: Strategy> core::iter::FromIterator<(K, V)> for SVMap<K, V, S> {
    fn from_iter<I: IntoIterator<Item=(K, V)>>(iter: I) -> Self {
        let sv = iter
            .into_iter()
            .map(|(k,v)| SVPair(k, Some(v)))
            .collect::<SortedVec<_>>();

        let mut dynamic = Dynamic::new();
        let len = sv.len();
        dynamic.add_unit(sv);

        Self {
            dynamic,
            len,
            free_count: 0,
        }
    }
}


/// Iterator over key-value pairs for [`SVMap`].
///
/// __Does not__ sort the values by key.
// FIXME: sort entries by key! (or better add a corresponding method)
pub struct SVMapKV<K, V> {
    unit_iter: DynamicIntoIter<SortedVec<SVPair<K, V>>>,
    opt_kv_iter: Option<alloc::vec::IntoIter<SVPair<K, V>>>,
}

impl<K, V> Iterator for SVMapKV<K, V> {
    type Item = (K, V);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(kv_iter) = &mut self.opt_kv_iter {
                while let Some(sv_pair) = kv_iter.next() {
                    if let Some(value) = sv_pair.1 {
                        return Some( (sv_pair.0, value) );
                    }
                }
            } // kv_iter is empty or absent
            
            self.opt_kv_iter = 
                self.unit_iter.next().map(|sv| sv.vec.into_iter());
            
            if let None = self.opt_kv_iter { return None; }
        }
    }
}


impl<K: Ord, V, S> core::iter::IntoIterator for SVMap<K, V, S> {
    type Item = (K, V);
    type IntoIter = SVMapKV<K, V>;

    fn into_iter(self) -> Self::IntoIter {
        SVMapKV {
            unit_iter: self.dynamic.into_iter(),
            opt_kv_iter: None,
        }
    }
}
