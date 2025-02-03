use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench_vec_new(c: &mut Criterion) {
    c.bench_function("vec_new", |b| {
        b.iter(|| {
            let mut v = Vec::new();
            for i in 0..1_000_000 {
                v.push(black_box(i)); // ✅ 防止循环被优化
            }
            black_box(v) // ✅ 防止Vec被优化
        })
    });
}

fn bench_vec_with_capacity(c: &mut Criterion) {
    c.bench_function("vec_with_capacity", |b| {
        b.iter(|| {
            let mut v = Vec::with_capacity(1_000_000);
            for i in 0..1_000_000 {
                v.push(black_box(i)); // ✅ 同上
            }
            black_box(v)
        })
    });
}

criterion_group!(benches, bench_vec_new, bench_vec_with_capacity);
criterion_main!(benches);