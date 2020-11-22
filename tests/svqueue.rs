use dynamization::sorted_vec::SVQueue;
use std::collections::BinaryHeap;

#[test]
fn test_sorted() {
    use rand::{ Rng, SeedableRng };
    
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);

    for &size in &[0, 1, 2, 3, 4, 5, 10, 100, 1000] {
        let mut to_process = Vec::<i32>::new();

        for _ in 0..size {
            to_process.push(rng.gen());
        }

        let mut to_sort = to_process.clone();
        to_sort.sort();

        let mut result = Vec::new();
        let mut svqueue = SVQueue::new();

        for x in to_process {
            svqueue.push(x);
        }

        while svqueue.len() > 0 {
            result.push(svqueue.pop().unwrap());
        }

        result.reverse();
        assert_eq!(to_sort, result);
    }
}



#[test]
fn test_binheap() {
    use rand::{ Rng, SeedableRng };
    
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);

    for &size in &[0, 1, 2, 3, 4, 5, 10, 100, 1000] {
        let mut svqueue = SVQueue::<i32>::new();
        let mut bin_heap = BinaryHeap::<i32>::new();

        for i in 0..size {
            let x = rng.gen();

            svqueue.push(x);
            bin_heap.push(x);

            let mut a = svqueue.clone();
            let mut b = bin_heap.clone();

            for _ in 0..=i {
                assert_eq!(a.pop(), b.pop());
            }
        }
    }
}

