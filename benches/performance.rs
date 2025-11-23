use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use minecraft_server_launcher::config::Config;
use minecraft_server_launcher::utils::checksum::{calculate_file_sha256, validate_file_checksum};
use minecraft_server_launcher::utils::validation::{validate_jar_file, validate_jar_and_calculate_checksum};
use std::fs;
use tempfile::TempDir;
use hex;

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

fn bench_checksum_calculation(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test_file.bin");
    
    let sizes = vec![1024, 1024 * 1024, 10 * 1024 * 1024];
    
    for size in sizes {
        let data = vec![0u8; size];
        fs::write(&file_path, &data).unwrap();
        
        let mut group = c.benchmark_group("checksum_calculation");
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_function(format!("sha256_{}KB", size / 1024), |b| {
            b.iter(|| {
                black_box(calculate_file_sha256(&file_path)).unwrap();
            });
        });
        group.finish();
    }
}

fn bench_checksum_validation(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let file_path = temp_dir.path().join("test_file.bin");
    
    let data = vec![0u8; 1024 * 1024];
    fs::write(&file_path, &data).unwrap();
    
    let expected_hash = calculate_file_sha256(&file_path).unwrap();
    
    c.bench_function("checksum_validation_valid", |b| {
        b.iter(|| {
            black_box(validate_file_checksum(&file_path, Some(&expected_hash))).unwrap();
        });
    });
    
    c.bench_function("checksum_validation_invalid", |b| {
        let invalid_hash = "a".repeat(64);
        b.iter(|| {
            let _ = black_box(validate_file_checksum(&file_path, Some(&invalid_hash)));
        });
    });
    
    c.bench_function("checksum_validation_none", |b| {
        b.iter(|| {
            black_box(validate_file_checksum(&file_path, None)).unwrap();
        });
    });
}

fn bench_jar_validation_comparison(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let jar_path = temp_dir.path().join("test.jar");
    
    use std::io::Write;
    use zip::ZipWriter;
    use zip::write::FileOptions;
    
    let file = std::fs::File::create(&jar_path).unwrap();
    let mut zip = ZipWriter::new(file);
    zip.start_file("test.txt", FileOptions::default()).unwrap();
    zip.write_all(b"test content").unwrap();
    zip.finish().unwrap();
    drop(zip);
    
    let checksum = calculate_file_sha256(&jar_path).unwrap();
    let checksum_path = jar_path.with_extension("jar.sha256");
    fs::write(&checksum_path, &checksum).unwrap();
    
    let mut group = c.benchmark_group("jar_validation_comparison");
    
    group.bench_function("old_method_validation_only", |b| {
        b.iter(|| {
            black_box(validate_jar_file(&jar_path)).unwrap();
        });
    });
    
    use minecraft_server_launcher::utils::validation::validate_jar_and_calculate_checksum;
    
    group.bench_function("new_method_with_checksum", |b| {
        b.iter(|| {
            let calculated = black_box(validate_jar_and_calculate_checksum(&jar_path)).unwrap();
            let expected_bytes = hex::decode(checksum.trim()).unwrap();
            assert_eq!(calculated.as_slice(), expected_bytes.as_slice());
        });
    });
    
    group.bench_function("checksum_only", |b| {
        b.iter(|| {
            black_box(validate_file_checksum(&jar_path, Some(&checksum))).unwrap();
        });
    });
    
    group.finish();
}

fn bench_jar_validation_with_checksum(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();
    let jar_path = temp_dir.path().join("test.jar");
    
    use std::io::Write;
    use zip::ZipWriter;
    use zip::write::FileOptions;
    
    let file = std::fs::File::create(&jar_path).unwrap();
    let mut zip = ZipWriter::new(file);
    zip.start_file("test.txt", FileOptions::default()).unwrap();
    zip.write_all(b"test content").unwrap();
    zip.finish().unwrap();
    drop(zip);
    
    c.bench_function("jar_validation_with_checksum", |b| {
        b.iter(|| {
            black_box(validate_jar_and_calculate_checksum(&jar_path)).unwrap();
        });
    });
}

criterion_group!(
    benches,
    bench_config_default,
    bench_config_validate,
    bench_config_work_directory,
    bench_checksum_calculation,
    bench_checksum_validation,
    bench_jar_validation_comparison,
    bench_jar_validation_with_checksum
);
criterion_main!(benches);

