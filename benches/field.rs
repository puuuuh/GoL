use criterion::{black_box, criterion_group, criterion_main, Criterion};
use life::Field;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut f = Box::new(Field::<1024, 1024>::default());
    let mut f1 = Box::new(Field::default());
    c.bench_function("fib 20", |b| {
        b.iter(|| {
            f.turn(&mut f1);
            std::mem::swap(&mut f, &mut f1);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
