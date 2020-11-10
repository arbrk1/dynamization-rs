//! # Making static containers dynamic
//!
//!

/// A trait that a static container must implement to become dynamizable.
pub trait Static where Self: Sized {
    /// Something akin to `(Key, Value)`.
    type Payload;

    /// Makes a single-element container from a single item.
    fn singleton(item: Self::Payload) -> Self;

    /// Merges two containers into one.
    ///
    /// One possible way to implement this is to collect both containers and 
    /// then make a new container from all the elements collected.
    fn merge_with(self, other: Self) -> Self;
}


/// A dynamic version of `Container`.
#[derive(Clone, Debug)]
pub struct Dynamic<Container/*, Strategy=Binary*/> {
    units: Vec<Option<Container>>,
    /*strategy: Strategy // essentially a marker, i.e. ZST*/
}


impl<Payload, Container: Static<Payload=Payload>> Dynamic<Container> {
    pub fn new() -> Self {
        Dynamic {
            units: Vec::with_capacity(8),
        }
    }

    // TODO: add strategies (binary, skew)
    pub fn from_sized(container: Container, size: usize) -> Self {
        let mut unit_size = 1;
        let mut index = 0;

        while unit_size < size {
            index += 1;
            unit_size *= 2;
        }

        let mut units = Vec::with_capacity(index+1);

        for _ in 0..index {
            units.push(None);
        }

        units.push(Some(container));

        Dynamic {
            units
        }
    }

    pub fn insert(&mut self, payload: Payload) {
        let mut container = Container::singleton(payload);
        
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


