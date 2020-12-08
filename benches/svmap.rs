use criterion::{
    criterion_group, criterion_main, Criterion, BenchmarkId, BatchSize
};


pub fn insertion(c: &mut Criterion) {
    use rand::{ Rng, SeedableRng };

    let mut rng = rand::rngs::StdRng::seed_from_u64(42);

    let mut group = c.benchmark_group("insertion");

    for size in &[10, 100, 1_000, 10_000, 100_000, 1_000_000] {
        let size = *size;

        let mut vec = Vec::<i32>::new();
        
        for _ in 0..size {
            vec.push(rng.gen());
        }
       
        group.bench_with_input(BenchmarkId::new("btree", size), &vec, |b, v| {
            b.iter(|| {
                use std::collections::BTreeMap;

                let mut btree = BTreeMap::<i32,i32>::new();

                for x in v.chunks(2) {
                    btree.insert(x[0], x[1]);
                }
            });
        });

        
        group.bench_with_input(BenchmarkId::new("svmap", size), &vec, |b, v| {
            b.iter(|| {
                use dynamization::sorted_vec::SVMap;

                let mut m = SVMap::<i32,i32>::new();

                for x in v.chunks(2) {
                    m.insert(x[0], x[1]);
                }
            });
        });
        

        group.bench_with_input(BenchmarkId::new("svmap_simple", size), &vec, |b, v| {
            b.iter(|| {
                use dynamization::sorted_vec::SVMap;
                use dynamization::strategy::SimpleBinary;

                let mut m = SVMap::<i32,i32>::with_strategy::<SimpleBinary>();

                for x in v.chunks(2) {
                    m.insert(x[0], x[1]);
                }
            });
        });
        
        group.bench_with_input(BenchmarkId::new("svmap_skew", size), &vec, |b, v| {
            b.iter(|| {
                use dynamization::sorted_vec::SVMap;
                use dynamization::strategy::SkewBinary;

                let mut m = SVMap::<i32,i32>::with_strategy::<SkewBinary>();

                for x in v.chunks(2) {
                    m.insert(x[0], x[1]);
                }
            });
        });
    }

    group.finish();
}


pub fn deletion(c: &mut Criterion) {
    use rand::{ Rng, SeedableRng };

    let mut rng = rand::rngs::StdRng::seed_from_u64(42);

    let mut group = c.benchmark_group("deletion");

    for size in &[10, 100, 1_000, 10_000, 100_000, 1_000_000] {
        let size = *size;

        let mut vec = Vec::<i32>::new();
        
        for _ in 0..size {
            vec.push(rng.gen());
        }
       
        group.bench_with_input(BenchmarkId::new("btree", size), &vec, |b, v| {
            b.iter_batched(|| {
                use std::collections::BTreeMap;

                let mut m = BTreeMap::<i32,i32>::new();

                for x in v.chunks(2) {
                    m.insert(x[0], x[1]);
                }

                (m, v)
            }, |(mut m, v)| {
                for x in v { m.remove(x); }
            }, BatchSize::SmallInput);
        });

        
        group.bench_with_input(BenchmarkId::new("svmap", size), &vec, |b, v| {
            b.iter_batched(|| {
                use dynamization::sorted_vec::SVMap;

                let mut m = SVMap::<i32,i32>::new();

                for x in v.chunks(2) {
                    m.insert(x[0], x[1]);
                }
                
                (m, v)
            }, |(mut m, v)| {
                for x in v { m.remove(x); }
            }, BatchSize::SmallInput);
        });
        

        group.bench_with_input(BenchmarkId::new("svmap_simple", size), &vec, |b, v| {
            b.iter_batched(|| {
                use dynamization::sorted_vec::SVMap;
                use dynamization::strategy::SimpleBinary;

                let mut m = SVMap::<i32,i32>::with_strategy::<SimpleBinary>();

                for x in v.chunks(2) {
                    m.insert(x[0], x[1]);
                }
                
                (m, v)
            }, |(mut m, v)| {
                for x in v { m.remove(x); }
            }, BatchSize::SmallInput);
        });
        
        group.bench_with_input(BenchmarkId::new("svmap_skew", size), &vec, |b, v| {
            b.iter_batched(|| {
                use dynamization::sorted_vec::SVMap;
                use dynamization::strategy::SkewBinary;

                let mut m = SVMap::<i32,i32>::with_strategy::<SkewBinary>();

                for x in v.chunks(2) {
                    m.insert(x[0], x[1]);
                }
                
                (m, v)
            }, |(mut m, v)| {
                for x in v { m.remove(x); }
            }, BatchSize::SmallInput);
        });
    }

    group.finish();
}


criterion_group!(benches, insertion, deletion);
criterion_main!(benches);

