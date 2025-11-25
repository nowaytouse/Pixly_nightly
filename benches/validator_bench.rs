//! Performance benchmark - Unified validator
//!
//! Compare sequential and parallel validation performance

use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use pixly_kernel::{UnifiedValidator, ValidatorInputFile};
use std::path::PathBuf;
use tempfile::TempDir;
use std::fs::File;

fn bench_validation(c: &mut Criterion) {
    let temp_dir = TempDir::new().unwrap();

    let file_counts = vec![10, 50, 100, 500];

    for count in file_counts {
        let files = create_test_files(&temp_dir, count);

        let mut group = c.benchmark_group(format!("validation_{}_files", count));

        group.bench_with_input(
            BenchmarkId::new("sequential", count),
            &files,
            |b, files| {
                b.iter(|| UnifiedValidator::validate_input_files(files))
            },
        );

        group.bench_with_input(
            BenchmarkId::new("parallel", count),
            &files,
            |b, files| {
                b.iter(|| UnifiedValidator::validate_input_files_parallel(files))
            },
        );

        group.finish();
    }
}

fn create_test_files(temp_dir: &TempDir, count: usize) -> Vec<ValidatorInputFile> {
    (0..count)
        .map(|i| {
            let path = temp_dir.path().join(format!("test_{}.png", i));
            File::create(&path).unwrap();

            ValidatorInputFile {
                file_path: path,
                name: format!("test_{}.png", i),
                ext: ".png".to_string(),
                size: 1024 * (i as u64 + 1),
                is_animated: false,
            }
        })
        .collect()
}

criterion_group!(benches, bench_validation);
criterion_main!(benches);
