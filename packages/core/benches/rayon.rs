use std::{hint::black_box, path::Path};

use criterion::{Criterion, criterion_group, criterion_main};
use lingora_core::prelude::{AuditEngine, LingoraToml};

fn bench_rayon(c: &mut Criterion) {
    let mut group = c.benchmark_group("rayon");

    group.sample_size(30);
    group.measurement_time(std::time::Duration::from_secs(15));

    group.bench_function("rayon", |b| {
        b.iter(|| {
            let toml =
                LingoraToml::try_from(Path::new("./tests/data/toml/Lingora_audit_engine.toml"))
                    .unwrap();
            let engine = black_box(AuditEngine::try_from(&toml).unwrap());
            let _ = black_box(engine.run());
        })
    });
}

criterion_group!(benches, bench_rayon);
criterion_main!(benches);
