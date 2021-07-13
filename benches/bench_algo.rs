use my_lib::algo::calculate_array_inversions;
use rand::Rng;


fn create_random_arr(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let mut v:Vec<u32> = Vec::with_capacity(n);
    for _ in 0..n {
        v.push( rng.gen() );
    }
    v
}

use criterion::{ criterion_group, criterion_main, Criterion};
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    const SMALL_ARR_SIZE:usize = 10_000;
    const LARGE_ARR_SIZE:usize = SMALL_ARR_SIZE * 20;
    let s_arr = create_random_arr(SMALL_ARR_SIZE);
    let l_arr = create_random_arr(LARGE_ARR_SIZE);


    let mut g = c.benchmark_group("Inversion");
    g.measurement_time(Duration::from_secs(1));
    g.bench_function(
       "inversion small",
       |b| b.iter(|| {
           calculate_array_inversions(&s_arr);
       }),
   );
    g.bench_function(
        "inversion large",
        |b| b.iter(|| {
            calculate_array_inversions(&l_arr);
        }),
    );
    g.finish();
}


criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
