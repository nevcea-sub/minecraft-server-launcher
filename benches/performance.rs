use criterion::{black_box, criterion_group, criterion_main, Criterion};
use minecraft_server_launcher::config::Config;

fn bench_config_default(c: &mut Criterion) {
    c.bench_function("config_default", |b| {
        b.iter(|| {
            black_box(Config::default());
        });
    });
}

fn bench_config_validate(c: &mut Criterion) {
    let config = Config::default();
    c.bench_function("config_validate", |b| {
        b.iter(|| {
            black_box(config.validate()).unwrap();
        });
    });
}

fn bench_config_work_directory(c: &mut Criterion) {
    let config = Config::default();
    c.bench_function("config_work_directory", |b| {
        b.iter(|| {
            black_box(config.work_directory());
        });
    });
}

fn bench_string_operations(c: &mut Criterion) {
    c.bench_function("string_from", |b| {
        b.iter(|| {
            black_box(String::from("test"));
        });
    });
    
    c.bench_function("string_to_string", |b| {
        b.iter(|| {
            black_box("test".to_string());
        });
    });
}

criterion_group!(benches, bench_config_default, bench_config_validate, bench_config_work_directory, bench_string_operations);
criterion_main!(benches);

