


use criterion::{Criterion, criterion_main, criterion_group};
use pprof::criterion::{PProfProfiler, Output};
use rand::Rng;
use trueskill::{Rating, quality::{free_for_all, quality_1vs1}};

const MU: f64 = 25.;
const SIGMA: f64 = MU / 3.;
const BETA: f64 = SIGMA / 2.;

fn free_for_all_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let sigmas = vec![5.1, 5.0, 6.7, 6.3, 7.0, 4.2];
    let ps: Vec<Vec<Rating<f64>>> = (0..20).into_iter()
        .map(|_| vec![Rating::new(rng.gen_range::<f64, _>(20.0..30.), sigmas[rng.gen_range(0..sigmas.len())])] )
        .collect();
    c.bench_function("free_for_all", |b| {
        b.iter(|| { free_for_all(&ps, BETA); });
    });
}

fn quality_1vs1_benchmark(c: &mut Criterion) {
    let mut rng = rand::thread_rng();
    let sigmas = vec![5.1, 5.0, 6.7, 6.3, 7.0, 4.2];
    let ps: Vec<Vec<Rating<f64>>> = (0..20).into_iter()
        .map(|_| vec![Rating::new(rng.gen_range::<f64, _>(20.0..30.), sigmas[rng.gen_range(0..sigmas.len())])] )
        .collect();

    c.bench_function("1vs1", |b| {
        b.iter(|| { quality_1vs1(&ps[0], &ps[1], None, BETA); });
    });

}

criterion_group!(name=benches;
                 config= Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
                 targets = free_for_all_benchmark, quality_1vs1_benchmark
);
criterion_main!(benches);
