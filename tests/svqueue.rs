use dynamization::sorted_vec::SVQueue;
use std::collections::BinaryHeap;
use dynamization::strategy;

#[test]
fn test_sorted() {
    test_sorted_strategy::<strategy::Binary>();
    test_sorted_strategy::<strategy::SimpleBinary>();
    test_sorted_strategy::<strategy::SkewBinary>();
}

fn test_sorted_strategy<S: strategy::Strategy>() {
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
        let mut svqueue = SVQueue::with_strategy::<S>();

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
    test_binheap_strategy::<strategy::Binary>();
    test_binheap_strategy::<strategy::SimpleBinary>();
    test_binheap_strategy::<strategy::SkewBinary>();
}

fn test_binheap_strategy<S: strategy::Strategy>() {
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

