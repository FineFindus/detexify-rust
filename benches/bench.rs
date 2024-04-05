use criterion::{ criterion_group, criterion_main, Criterion};
use detexify::{Point, Stroke, StrokeSample};

pub fn criterion_benchmark(c: &mut Criterion) {
    let classifier = detexify::Classifier::default();
    let sample = StrokeSample::new(vec![
        Stroke::new(vec![
            Point {
                x: 0.22204075572547421,
                y: 0.0,
            },
            Point {
                x: 0.7779592442745257,
                y: 0.9999999999999997,
            },
        ]),
        detexify::Stroke::new(vec![
            Point {
                x: 0.7266524285871323,
                y: 0.0005339279825375283,
            },
            Point {
                x: 0.2733475714128677,
                y: 1.0,
            },
        ]),
    ])
    .unwrap();
    c.bench_function("classify", |b| {
        b.iter(|| classifier.classify(sample.clone()))
    });

    // c.bench_function("classify-par", |b| {
    //     b.iter(|| classifier.classify_par(sample.clone()))
    // });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
