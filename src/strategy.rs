//! Different dynamization strategies.
//!
//! Currently only [`Binary`] is supported.

use crate::*;


/// A dynamization strategy.
///
/// Can be simply a ZST. Also can contain some internal bookkeeping machinery.
pub trait Strategy where Self: Sized {
    /// A default strategy with a default unit count.
    fn new_unit_count() -> (Self, usize);

    /// A strategy with a specified unit count.
    fn with_unit_count(unit_count: usize) -> Self;

    /// An algorithm for adding a new unit.
    ///
    /// Can modify an internal state of the strategy.
    fn add<Container: Static>(
        &mut self, units: &mut Vec<Option<Container>>, container: Container
    );
}


/// Binary dynamization.
///
/// A unit with the index `k` is of size between `2^{k-1}+1` and `2^k`.
///
/// When a new unit is added to an occupied index, it is merged with 
/// the occupying one and placed one place further.
#[derive(Clone, Debug)]
pub struct Binary;


impl Binary {
    fn unit_index(size: usize) -> usize {
        let mut unit_size = 1;
        let mut index = 0;

        while unit_size < size {
            index += 1;
            unit_size *= 2;
        }

        index
    }
}


impl Strategy for Binary {
    fn new_unit_count() -> (Self, usize) {
        (Binary, 8)
    }

    fn with_unit_count(_unit_count: usize) -> Self {
        Binary
    }

    fn add<Container: Static>(
        &mut self, 
        units: &mut Vec<Option<Container>>, 
        mut container: Container)
    {
        let index = Binary::unit_index(container.len());

        while units.len() <= index {
            units.push(None);
        }

        for unit in &mut units[index..] {
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
         
        units.push(Some(container));
    }
}

