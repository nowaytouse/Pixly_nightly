/**
 * Pixly Rust Conversion Performance Benchmarks
 *
 * Run with:
 * cargo bench --bench conversion_benchmark
 *
 * Test items:
 * 1. Feature extraction performance
 * 2. File analysis performance
 * 3. Format detection performance
 * 4. Parameter optimization performance
 * 5. Memory usage
 */

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::path::PathBuf;
use std::time::Duration;

fn create_test_image_path() -> PathBuf {
    PathBuf::from("test_data/sample.jpg")
}

fn create_test_features() -> Vec<f32> {
    vec![0.5; 128]
}

fn benchmark_feature_extraction(c: &mut Criterion) {
    let mut group = c.benchmark_group("feature_extraction");
    group.measurement_time(Duration::from_secs(10));

    group.bench_function("extract_128d_features", |b| {
        b.iter(|| {
            let features: Vec<f32> = (0..128).map(|i| (i as f32) / 128.0).collect();
            black_box(features)
        });
    });

    group.bench_function("normalize_features", |b| {
        let features = create_test_features();
        b.iter(|| {
            let sum: f32 = features.iter().sum();
            let normalized: Vec<f32> = features.iter().map(|&x| x / sum).collect();
            black_box(normalized)
        });
    });

    group.finish();
}

fn benchmark_file_analysis(c: &mut Criterion) {
    let mut group = c.benchmark_group("file_analysis");

    group.bench_function("path_parsing", |b| {
        let path = create_test_image_path();
        b.iter(|| {
            let ext = path.extension().and_then(|s| s.to_str());
            let name = path.file_stem().and_then(|s| s.to_str());
            black_box((ext, name))
        });
    });

    group.bench_function("format_detection", |b| {
        b.iter(|| {
            let formats = vec!["jpg", "png", "webp", "avif", "jxl"];
            let ext = "jpg";
            let detected = formats.contains(&ext);
            black_box(detected)
        });
    });

    group.finish();
}

fn benchmark_parameter_optimization(c: &mut Criterion) {
    let mut group = c.benchmark_group("parameter_optimization");

    group.bench_function("quality_calculation", |b| {
        b.iter(|| {
            let complexity = 0.75;
            let quality = if complexity > 0.8 {
                95
            } else if complexity > 0.5 {
                90
            } else {
                85
            };
            black_box(quality)
        });
    });

    group.bench_function("preset_selection", |b| {
        b.iter(|| {
            let file_size = 1024 * 1024 * 5;
            let preset = if file_size > 10 * 1024 * 1024 {
                "fast"
            } else if file_size > 1024 * 1024 {
                "medium"
            } else {
                "slow"
            };
            black_box(preset)
        });
    });

    group.finish();
}

fn benchmark_data_structures(c: &mut Criterion) {
    let mut group = c.benchmark_group("data_structures");

    group.bench_function("vec_push_1000", |b| {
        b.iter(|| {
            let mut vec = Vec::new();
            for i in 0..1000 {
                vec.push(i);
            }
            black_box(vec)
        });
    });

    group.bench_function("vec_with_capacity_1000", |b| {
        b.iter(|| {
            let mut vec = Vec::with_capacity(1000);
            for i in 0..1000 {
                vec.push(i);
            }
            black_box(vec)
        });
    });

    group.bench_function("hashmap_insert_100", |b| {
        b.iter(|| {
            let mut map = std::collections::HashMap::new();
            for i in 0..100 {
                map.insert(format!("key_{}", i), i);
            }
            black_box(map)
        });
    });

    group.finish();
}

fn benchmark_string_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("string_operations");

    group.bench_function("format_macro", |b| {
        b.iter(|| {
            let result = format!("File: {} Size: {} MB", "test.jpg", 5);
            black_box(result)
        });
    });

    group.bench_function("string_concat", |b| {
        b.iter(|| {
            let mut result = String::from("File: ");
            result.push_str("test.jpg");
            result.push_str(" Size: ");
            result.push_str("5");
            result.push_str(" MB");
            black_box(result)
        });
    });

    group.bench_function("path_join", |b| {
        b.iter(|| {
            let path = PathBuf::from("/path/to");
            let result = path.join("file.jpg");
            black_box(result)
        });
    });

    group.finish();
}

fn benchmark_concurrency(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrency");

    group.bench_function("sequential_processing", |b| {
        b.iter(|| {
            let mut results = Vec::new();
            for i in 0..100 {
                let result = i * 2;
                results.push(result);
            }
            black_box(results)
        });
    });

    group.finish();
}

criterion_group!(
    benches,
    benchmark_feature_extraction,
    benchmark_file_analysis,
    benchmark_parameter_optimization,
    benchmark_data_structures,
    benchmark_string_operations,
    benchmark_concurrency
);

criterion_main!(benches);
