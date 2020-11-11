use crate::*;


pub trait Strategy where Self: Sized {
    fn new_capacity() -> (Self, usize);

    fn with_capacity(capacity: usize) -> Self;

    fn add<Container: Static>(
        &mut self, units: &mut Vec<Option<Container>>, container: Container
    );
}


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
    fn new_capacity() -> (Self, usize) {
        (Binary, 8)
    }

    fn with_capacity(_capacity: usize) -> Self {
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

