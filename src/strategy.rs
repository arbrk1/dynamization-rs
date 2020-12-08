//! Different dynamization strategies.
//!
//! Currently only [`Binary`] is supported.

use crate::*;


/// A dynamization strategy.
///
/// Can be simply a ZST. Also can contain some internal bookkeeping machinery.
pub trait Strategy where Self: Sized {
    /// A default strategy with a default initial unit count.
    fn new_unit_count() -> (Self, usize);

    /// A strategy with a specified initial unit count.
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
/// A nonempty unit with the index `k>0` has size between `2^{k-1}+1` and `2^k`.
///
/// A new unit is added to the most appropriate index: if this index 
/// is occupied, the added unit is merged with
/// the occupying one and moved to the index corresponding to the resulting size.
///
/// Sometimes this strategy can provoke a long chain of merges (but not 
/// longer than a logarithm of the total size). For a more 
/// predictable performance see the [`SkewBinary`] strategy.
#[derive(Clone, Debug)]
pub struct Binary;


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
        let len = container.len();

        let mut unit_size = 1;
        let mut index = 0;

        while unit_size < len {
            index += 1;
            unit_size *= 2;
        }

        while units.len() <= index {
            units.push(None);
        }

        for unit in &mut units[index..] {
            let content = core::mem::replace(unit, None);
            
            match content {
                None => {
                    *unit = Some(container);
                    return;
                }

                Some(other) => {
                    container = container.merge_with(other);

                    if container.len() <= unit_size {
                        *unit = Some(container);
                        return;
                    }
                }
            }
        }
         
        units.push(Some(container));
    }
}


/// Simple binary dynamization.
///
/// Much like [`Binary`] but doesn't account for unit sizes: every new unit 
/// is added to the index 0.
///
/// Nevertheless has the same average asymptotics: each stored item `X` participates 
/// in at most `log N` merges where `N` is the count of items added after the 
/// item `X` was inserted.
#[derive(Clone, Debug)]
pub struct SimpleBinary;


impl Strategy for SimpleBinary {
    fn new_unit_count() -> (Self, usize) {
        (SimpleBinary, 8)
    }

    fn with_unit_count(_unit_count: usize) -> Self {
        SimpleBinary
    }

    fn add<Container: Static>(
        &mut self, 
        units: &mut Vec<Option<Container>>, 
        mut container: Container)
    {
        for unit in &mut *units {
            let content = core::mem::replace(unit, None);
            
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


/// Skew-binary dynamization.
///
/// Ensures that each unit addition makes not more than two merges.
///
/// Useful when there is a large overhead for each merge and predictable 
/// performance is needed.
///
/// But in other cases [`Binary`] and [`SimpleBinary`] are much more preferable.
#[derive(Clone, Debug)]
pub struct SkewBinary {
    last_merge: usize
}

impl Strategy for SkewBinary {
    fn new_unit_count() -> (Self, usize) {
        (SkewBinary { last_merge: 0 }, 8)
    }

    fn with_unit_count(unit_count: usize) -> Self {
        assert!(unit_count > 0);

        SkewBinary {
            last_merge: 0,
        }
    }

    fn add<Container: Static>(
        &mut self, 
        units: &mut Vec<Option<Container>>, 
        mut container: Container)
    {
        if units.len() == 0 {
            units.push(Some(container));
            return;
        }

        let unit = &mut units[self.last_merge];
        let content = core::mem::replace(unit, None);
            
        match content {
            None => {
                *unit = Some(container);
                return;
            }

            Some(other) => {
                container = container.merge_with(other);
            }
        }

        let new_index = self.last_merge + 1;
        
        if units.len() == new_index {
            units.push(Some(container));
            self.last_merge = 0;
            return;
        }

        let unit = &mut units[new_index];
        let content = core::mem::replace(unit, None);
            
        match content {
            None => {
                *unit = Some(container);
                self.last_merge = 0;
            }

            Some(other) => {
                *unit = Some(container.merge_with(other));
                self.last_merge = new_index;
            }
        }
    }
}

