use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::Rng;
use raymarch::vec3::Vec3;

fn dot_prod(lhs: &Vec3, rhs: &Vec3) -> f32 {
    let x = lhs.vec[0] * rhs.vec[0];
    let y = lhs.vec[1] * rhs.vec[1];
    let z = lhs.vec[2] * rhs.vec[2];
    x + y + z
}

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("fib 20", |b| b.iter(|| fibonacci(black_box(20))));
    let mut group = c.benchmark_group("dot-prod SIMD");
    let mut rand_num = rand::thread_rng();
    for _ in 0..5 {
        let (x, y, z, p, d, q) = rand_num.gen::<(f32, f32, f32, f32, f32, f32)>();
        let input = Vec3::new(x, y, z);
        let input2 = Vec3::new(p, d, q);
        group.bench_with_input(
            BenchmarkId::new("SIMD", format!("{:?}", &input.vec)),
            &(input, input2),
            |b, (input, input2)| b.iter(|| Vec3::dot_prod(&input, input2)),
        );
        group.bench_with_input(
            BenchmarkId::new("NORMAL", format!("{:?}", &input.vec)),
            &(input, input2),
            |b, (input, input2)| b.iter(|| dot_prod(&input, &input2)),
        );
    }
    group.finish();

    // c.bench_function("dot-prod SIMD", || )
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
