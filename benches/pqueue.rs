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
       
        group.bench_with_input(BenchmarkId::new("bin_heap", size), &vec, |b, v| {
            b.iter(|| {
                use std::collections::BinaryHeap;

                let mut bin_heap = BinaryHeap::<i32>::new();

                for x in v {
                    bin_heap.push(*x);
                }
            });
        });

        
        group.bench_with_input(BenchmarkId::new("svqueue", size), &vec, |b, v| {
            b.iter(|| {
                use dynamization::sorted_vec::SVQueue;

                let mut q = SVQueue::<i32>::new();

                for x in v {
                    q.push(*x);
                }
            });
        });
        

        group.bench_with_input(BenchmarkId::new("svqueue_simple", size), &vec, |b, v| {
            b.iter(|| {
                use dynamization::sorted_vec::SVQueue;
                use dynamization::strategy::SimpleBinary;

                let mut q = SVQueue::<i32>::with_strategy::<SimpleBinary>();

                for x in v {
                    q.push(*x);
                }
            });
        });
        
        group.bench_with_input(BenchmarkId::new("svqueue_skew", size), &vec, |b, v| {
            b.iter(|| {
                use dynamization::sorted_vec::SVQueue;
                use dynamization::strategy::SkewBinary;

                let mut q = SVQueue::<i32>::with_strategy::<SkewBinary>();

                for x in v {
                    q.push(*x);
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
       
        group.bench_with_input(BenchmarkId::new("bin_heap", size), &vec, |b, v| {
            b.iter_batched(|| {
                use std::collections::BinaryHeap;

                let mut bin_heap = BinaryHeap::<i32>::new();

                for x in v {
                    bin_heap.push(*x);
                }

                bin_heap
            }, |mut bin_heap| {
                while let Some(_) = bin_heap.pop() {}
            }, BatchSize::SmallInput);
        });

        
        group.bench_with_input(BenchmarkId::new("svqueue", size), &vec, |b, v| {
            b.iter_batched(|| {
                use dynamization::sorted_vec::SVQueue;

                let mut q = SVQueue::<i32>::new();

                for x in v {
                    q.push(*x);
                }
                
                q
            }, |mut q| {
                while let Some(_) = q.pop() {}
            }, BatchSize::SmallInput);
        });
        

        group.bench_with_input(BenchmarkId::new("svqueue_simple", size), &vec, |b, v| {
            b.iter_batched(|| {
                use dynamization::sorted_vec::SVQueue;
                use dynamization::strategy::SimpleBinary;

                let mut q = SVQueue::<i32>::with_strategy::<SimpleBinary>();

                for x in v {
                    q.push(*x);
                }
                
                q
            }, |mut q| {
                while let Some(_) = q.pop() {}
            }, BatchSize::SmallInput);
        });
        
        group.bench_with_input(BenchmarkId::new("svqueue_skew", size), &vec, |b, v| {
            b.iter_batched(|| {
                use dynamization::sorted_vec::SVQueue;
                use dynamization::strategy::SkewBinary;

                let mut q = SVQueue::<i32>::with_strategy::<SkewBinary>();

                for x in v {
                    q.push(*x);
                }
                
                q
            }, |mut q| {
                while let Some(_) = q.pop() {}
            }, BatchSize::SmallInput);
        });
    }

    group.finish();
}


criterion_group!(benches, insertion, deletion);
criterion_main!(benches);

