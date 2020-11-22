//! # Making static containers dynamic
//!
//! Sometimes it's much easier to construct some data structure from 
//! a predetermined set of data than to implement a way to update 
//! this data structure with new elements after construction.
//!
//! E.g. it's trivial to make a perfectly balanced search tree 
//! when the data is already known but not so trivial to keep its 
//! balance after adding/deleting some elements.
//!
//! This crate provides a cheap workaround for the case of a data 
//! structure not having any sensible method of insertion.
//!
//! ## Example
//!
//! Suppose you have a sorted vector:
//!
//! ```
//! # use core::iter::FromIterator;
//! struct SortedVec<T> {
//!     vec: Vec<T>
//! }
//!
//! impl<T: Ord> FromIterator<T> for SortedVec<T> {
//!     fn from_iter<I>(iter: I) -> Self where
//!         I: IntoIterator<Item=T>
//!     {
//!         let mut vec: Vec<_> = iter.into_iter().collect();
//!
//!         vec.sort();
//!
//!         SortedVec { vec }
//!     }
//! }
//! ```
//!
//! This is almost a perfect data structure for many use cases but every 
//! insertion is on the average linear in the length of the array.
//!
//! This crate provides a struct [`Dynamic`]:
//!
//! ```
//! # struct SortedVec<T> { t: T }
//! use dynamization::Dynamic;
//!
//! type DynamicSortedVec<T> = Dynamic<SortedVec<T>>;
//! ```
//!
//! which groups the stored data into independent 
//! [`units`](Dynamic::units) of different sizes.
//! The unit sizes are selected in such a way to make single-element 
//! insertions on the average logarithmic.
//!
//! The only thing needed to make [`Dynamic`] work is
//! to implement the [`Static`] trait:
//!
//! ```
//! # use core::iter::FromIterator;
//! # struct SortedVec<T> { vec: Vec<T> }
//! # impl<T: Ord> FromIterator<T> for SortedVec<T> {
//! #    fn from_iter<I>(iter: I) -> Self where
//! #        I: IntoIterator<Item=T> {
//! #        let mut vec: Vec<_> = iter.into_iter().collect();
//! #        vec.sort();
//! #        SortedVec { vec }}}
//! use dynamization::Static;
//!
//! impl<T: Ord> Static for SortedVec<T> {
//!     fn len(&self) -> usize { 
//!         self.vec.len() 
//!     }
//!
//!     fn merge_with(self, other: Self) -> Self {
//!         // Only for documentation purposes: two sorted arrays can be merged 
//!         // much more efficiently than sorting the concatenation result!
//!         self.vec.into_iter().chain(other.vec).collect()
//!     }
//! }
//! ```
//!
//! Now `DynamicSortedVec` has the [`add_unit`](Dynamic::add_unit) method.
//!
//! An optional trait [`Singleton`] can also be 
//! implemented to make the [`insert`](Dynamic::insert) method 
//! available:
//! ```
//! # struct SortedVec<T> { vec: Vec<T> }
//! use dynamization::Singleton;
//!
//! impl<T> Singleton for SortedVec<T> {
//!     type Item = T;
//!     
//!     fn singleton(item: Self::Item) -> Self {
//!         SortedVec { vec: vec![item] }
//!     }
//! }
//! ```
//!
//! Now you can use `DynamicSortedVec` as a rather efficient universal 
//! data structure:
//!
//! ```
//! # use dynamization::{ Static, Dynamic, Singleton };
//! # use core::iter::FromIterator;
//! # struct SortedVec<T> { vec: Vec<T> }
//! # impl<T: Ord> FromIterator<T> for SortedVec<T> {
//! #    fn from_iter<I>(iter: I) -> Self where
//! #        I: IntoIterator<Item=T> {
//! #        let mut vec: Vec<_> = iter.into_iter().collect();
//! #        vec.sort();
//! #        SortedVec { vec }}}
//! # impl<T: Ord> Static for SortedVec<T> {
//! #    fn len(&self) -> usize { self.vec.len() }
//! #    fn merge_with(self, other: Self) -> Self {
//! #        self.vec.into_iter().chain(other.vec).collect()
//! #    }
//! # }
//! # impl<T> Singleton for SortedVec<T> {
//! #     type Item = T;
//! #     fn singleton(item: Self::Item) -> Self {SortedVec {vec:vec![item]}}
//! # }
//! # type DynamicSortedVec<T> = Dynamic<SortedVec<T>>;
//! let mut foo = DynamicSortedVec::new();
//! for x in vec![(1, "one"), (5, "five"), (4, "four"), (3, "tree"), (6, "six")] {
//!     foo.insert(x);
//! }
//!
//! // Each query now must be implemented in terms of partial containers:
//! foo.units_mut().filter_map(|unit| {
//!     unit.vec
//!         .binary_search_by_key(&3, |pair| pair.0)
//!         .ok()
//!         .map(move |index| &mut unit.vec[index])
//! }).for_each(|three| {
//!     assert_eq!(three, &(3, "tree"));
//!     three.1 = "three";
//! });
//!
//! // A dynamic structure can be "freezed" with .try_collect():
//! assert_eq!(foo.try_collect().unwrap().vec, vec![
//!     (1, "one"),
//!     (3, "three"),
//!     (4, "four"),
//!     (5, "five"),
//!     (6, "six"),
//! ]);
//! ```

/// A trait that a static container must implement to become dynamizable.
pub trait Static where Self: Sized {
    /// Size of the container.
    ///
    /// Best measured with the number of single-element insertions needed 
    /// to make such a container.
    ///
    /// If the size can't be determined (i.e. the container doesn't have any 
    /// way of defining single-element insertion), return `1` and use some
    /// strategy which doesn't rely on knowing correct sizes: e.g. 
    /// [`SimpleBinary`](strategy::SimpleBinary) or 
    /// [`SkewBinary`](strategy::SkewBinary).
    fn len(&self) -> usize;

    /// Merges two containers into one.
    ///
    /// One possible way to implement this is to collect both containers and 
    /// then make a new container from all the elements collected.
    fn merge_with(self, other: Self) -> Self;
}

/// A trait which can be implemented to provide a dynamized structure
/// with a convenient [`insert`](Dynamic::insert) method.
pub trait Singleton where Self: Sized {
    /// A type of the container payload.
    type Item;

    /// A container made from a single item.
    fn singleton(item: Self::Item) -> Self;
}



pub mod strategy;
use strategy::Strategy;

/// A dynamic version of `Container`.
#[derive(Clone, Debug)]
pub struct Dynamic<Container, S = strategy::Binary> {
    units: Vec<Option<Container>>,
    strategy: S,
}


impl<Container: Static, S: Strategy> Dynamic<Container, S> {
    /// A new container with a default initial unit count.
    pub fn new() -> Self {
        let (strategy, unit_count) = S::new_unit_count();

        Dynamic {
            units: Vec::with_capacity(unit_count),
            strategy,
        }
    }

    /// A new container with a specified initial unit count.
    pub fn with_unit_count(unit_count: usize) -> Self {
        let strategy = S::with_unit_count(unit_count);
        
        Dynamic {
            units: Vec::with_capacity(unit_count),
            strategy,
        }
    }

    /// Adds a new unit (partial container).
    pub fn add_unit(&mut self, container: Container) {
        self.strategy.add(&mut self.units, container);
    }

    /// Total size of the container.
    ///
    /// It is calculated as a sum of partial lengths.
    /// Usually can be replaced without much hassle by a variable 
    /// tracking insertions/deletions.
    pub fn len(&self) -> usize {
        self.units().map(|x| x.len()).sum()
    }

    /// Iterator over all partial containers. Shared-reference version.
    pub fn units(&self) -> impl Iterator<Item=&Container> {
        self.units.iter().filter_map(|x| x.as_ref())
    }

    /// Iterator over all partial containers. Unique-reference version.
    pub fn units_mut(&mut self) -> impl Iterator<Item=&mut Container> {
        self.units.iter_mut().filter_map(|x| x.as_mut())
    }

    /// Collects all partial containers into a single one.
    ///
    /// Returns `None` if there are no units.
    pub fn try_collect(self) -> Option<Container> {
        let mut iter = self.units.into_iter().filter_map(|x| x);

        match iter.next() {
            None => None,

            Some(first) => {
                Some( iter.fold(first, |acc, x| acc.merge_with(x)) )
            }
        }
    }
}

impl<Container: Static+Singleton, S: Strategy> Dynamic<Container, S> {
    /// Inserts a single item.
    ///
    /// Requires [`Singleton`] to be implemented for the container type.
    pub fn insert(&mut self, item: Container::Item) {
        self.add_unit(Container::singleton(item));
    }
}


#[cfg(any(feature = "sorted_vec", doc))]
pub mod sorted_vec;


