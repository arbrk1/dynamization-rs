use dynamization::sorted_vec::SVMap;
use dynamization::strategy;
use std::collections::BTreeMap;


#[test]
fn test_assoc() {
    test_assoc_strategy::<strategy::Binary>();
    test_assoc_strategy::<strategy::SimpleBinary>();
    test_assoc_strategy::<strategy::SkewBinary>();
}

fn test_assoc_strategy<S: strategy::Strategy>() {
    use rand::{ Rng, SeedableRng };
    
    let mut rng = rand::rngs::StdRng::seed_from_u64(42);

    for &size in &[0, 1, 2, 3, 4, 5, 10, 100, 1000, 10000] {
        let mut svmap = SVMap::<i32,i32>::new();
        let mut btree = BTreeMap::<i32,i32>::new();

        for _ in 0..size {
            let k = rng.gen_range(0, 100);
            let v = rng.gen();
            let action = rng.gen_range(0, 10);

            if action < 7 {
                assert_eq!(svmap.insert(k, v), btree.insert(k, v));
            } else {
                assert_eq!(svmap.remove(&k), btree.remove(&k));
            }

            assert_eq!(svmap.len(), btree.len());

            for ref k in 0..100 {
                assert_eq!(svmap.get(k), btree.get(k));
            }
        }
    }
}

