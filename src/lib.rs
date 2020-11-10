//! # Making static containers dynamic
//!
//!

/// A trait that a static container must implement to become dynamizable.
pub trait Static where Self: Sized {
    /// Merges two containers into one.
    ///
    /// One possible way to implement this is to collect both containers and 
    /// then make a new container from all the elements collected.
    fn merge_with(self, other: Self) -> Self;
}


pub trait Strategy {
    fn new_with_capacity() -> (Self, usize);

    fn unit_index(size: usize) -> usize;

    fn insert<Container: Static>(
        &mut self, units: &mut Vec<Option<Container>>, container: Container
    );
}

pub mod strategy;


/// A dynamic version of `Container`.
#[derive(Clone, Debug)]
pub struct Dynamic<Container, S: Strategy> {
    units: Vec<Option<Container>>,
    strategy: S,
}


impl<Container: Static, S: Strategy> Dynamic<Container, S> {
    pub fn new() -> Self {
        let (strategy, capacity) = S::new_with_capacity();

        Dynamic {
            units: Vec::with_capacity(capacity),
            strategy,
        }
    }

    pub fn from_sized(container: Container, size: usize) -> Self {
        /*
        let mut unit_size = 1;
        let mut index = 0;

        while unit_size < size {
            index += 1;
            unit_size *= 2;
        }
        */

        let index = S::unit_index();

        let mut units = Vec::with_capacity(index+1);

        for _ in 0..index {
            units.push(None);
        }

        units.push(Some(container));

        Dynamic {
            units
        }
    }

    pub fn insert(&mut self, item: Payload) {
        let mut container = Container::singleton(item);
        
        for unit in &mut self.units {
            let content = std::mem::replace(unit, None);
            
            match content {
                None => {
                    *unit = Some(container);
                    return;
                }

                Some(other) => {
                    container = container.merge_with(other);
                }
            }
        }

        self.units.push(Some(container));
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


