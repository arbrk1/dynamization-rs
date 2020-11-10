//! # Making static containers dynamic
//!
//!

/// A trait that a static container must implement to become dynamizable.
pub trait Static where Self: Sized {
    /// Size of the container.
    ///
    /// Best measured with the number of single-element insertions needed 
    /// to make such a container.
    fn len(&self) -> usize;

    /// Merges two containers into one.
    ///
    /// One possible way to implement this is to collect both containers and 
    /// then make a new container from all the elements collected.
    fn merge_with(self, other: Self) -> Self;
}


pub trait Strategy where Self: Sized {
    fn new_capacity() -> (Self, usize);

    fn with_capacity(capacity: usize) -> Self;

    fn insert<Container: Static>(
        &mut self, units: &mut Vec<Option<Container>>, container: Container
    );
}

pub mod strategy;


/// A dynamic version of `Container`.
#[derive(Clone, Debug)]
pub struct Dynamic<Container, S: Strategy = strategy::Binary> {
    units: Vec<Option<Container>>,
    strategy: S,
}


impl<Container: Static, S: Strategy> Dynamic<Container, S> {
    pub fn new() -> Self {
        let (strategy, capacity) = S::new_capacity();

        Dynamic {
            units: Vec::with_capacity(capacity),
            strategy,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        let strategy = S::with_capacity(capacity);
        
        Dynamic {
            units: Vec::with_capacity(capacity),
            strategy,
        }
    }

    pub fn insert(&mut self, container: Container) {
        self.strategy.insert(&mut self.units, container);
    }

    /// Total size of the container.
    ///
    /// It is calculated as a sum of partial lengths.
    /// Usually can be replaced without much hassle by a variable 
    /// tracking insertions/deletion.
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
}


#[cfg(any(feature = "sorted_vec", doc))]
pub mod sorted_vec;


