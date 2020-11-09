//! # Making static containers dynamic
//!
//!

/// A trait which a static container must implement to become dynamizable.
pub trait Static where Self: Sized {
    /// Something akin to `(Key, Value)`.
    type Payload;

    /// Makes a single-element container from a single datum.
    fn singleton(payload: Self::Payload) -> Self;

    /// Merges two containers into one.
    ///
    /// One possible way to implement this is to collect both containers and 
    /// then make a new container from all the elements collected.
    fn merge_with(self, other: Self) -> Self;
}


/// A dynamic version of `Container`.
#[derive(Clone, Debug)]
pub struct Dynamic<Container> {
    levels: Vec<Option<Container>>,
}



/// Iterator over all partial containers. Shared-reference version.
pub struct Levels<'a, Container> {
    index: usize,
    iter: core::slice::Iter<'a, Option<Container>>,
}

impl<'a, Container> Iterator for Levels<'a, Container> {
    type Item = (usize, &'a Container);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let index = self.index;
            self.index += 1;

            match self.iter.next() {
                None => { return None; }

                Some(x) => {
                    if let Some(container) = x.as_ref() {
                        return Some ( (index, container) );
                    }
                }
            }
        }
    }
}


/// Iterator over all partial containers. Unique-reference version.
pub struct LevelsMut<'a, Container> {
    index: usize,
    iter: core::slice::IterMut<'a, Option<Container>>,
}

impl<'a, Container> Iterator for LevelsMut<'a, Container> {
    type Item = (usize, &'a mut Container);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let index = self.index;
            self.index += 1;

            match self.iter.next() {
                None => { return None; }

                Some(x) => {
                    if let Some(container) = x.as_mut() {
                        return Some ( (index, container) );
                    }
                }
            }
        }
    }
}


impl<Payload, Container: Static<Payload=Payload>> Dynamic<Container> {
    pub fn new() -> Self {
        Dynamic {
            levels: Vec::with_capacity(8),
        }
    }

    pub fn insert(&mut self, payload: Payload) {
        let mut container = Container::singleton(payload);
        
        for level in &mut self.levels {
            let content = std::mem::replace(level, None);
            
            match content {
                None => {
                    *level = Some(container);
                    return;
                }

                Some(other) => {
                    container = container.merge_with(other);
                }
            }
        }

        self.levels.push(Some(container));
    }

    pub fn get(&self, index: usize) -> Option<&Container> {
        self.levels.get(index).map(|x| x.as_ref()).flatten()
    }
    
    pub unsafe fn get_unchecked(&self, index: usize) -> &Container {
        self.levels.get_unchecked(index).as_ref().unwrap()
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Container> {
        self.levels.get_mut(index).map(|x| x.as_mut()).flatten()
    }
    
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut Container {
        self.levels.get_unchecked_mut(index).as_mut().unwrap()
    }

    pub fn levels(&self) -> Levels<Container> {
        Levels {
            index: 0,
            iter: self.levels.iter(),
        }
    }

    pub fn levels_mut(&mut self) -> LevelsMut<Container> {
        LevelsMut {
            index: 0,
            iter: self.levels.iter_mut(),
        }
    }
}


pub mod sorted_vec;


