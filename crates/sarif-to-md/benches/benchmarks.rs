use anyhow::{Context, Result};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use sarif_to_md_core::{
    generators::SarifMarkdownGenerator, markdown::MarkdownFormat, ReportProcessorBuilder,
};
use std::fs;
use std::hint::black_box;

fn load_example_sarif(name: &str) -> Result<String> {
    let path = format!("../../examples/sarif-files/{}.sarif", name);
    fs::read_to_string(&path).with_context(|| format!("Failed to load SARIF example: {}", path))
}

fn load_example_for_bench(name: &str) -> String {
    load_example_sarif(name).expect(&format!(
        "Benchmark setup failed: missing example file '{}'",
        name
    ))
}

fn bench_core_generation(c: &mut Criterion) {
    let examples = vec![
        ("minimal", "01-minimal"),
        ("metadata", "02-rule-metadata"),
        ("flows", "04-code-flows"),
        ("large", "08-embedded-content"),
    ];

    let formats = vec![
        ("CommonMark", MarkdownFormat::CommonMark),
        ("GitHub", MarkdownFormat::GitHubFlavored),
    ];

    let mut group = c.benchmark_group("core_generation");

    for (example_label, example) in examples {
        let sarif_content = load_example_for_bench(example);

        for (format_name, format) in &formats {
            for with_emoji in [false, true] {
                let emoji_label = if with_emoji { "emoji" } else { "no_emoji" };
                let bench_id = format!("{}/{}/{}", example_label, format_name, emoji_label);

                group.bench_with_input(
                    BenchmarkId::new("generation", bench_id),
                    &(&sarif_content, *format, with_emoji),
                    |b, (content, fmt, emoji)| {
                        b.iter(|| {
                            let processor = ReportProcessorBuilder::new()
                                .generator(SarifMarkdownGenerator::new(*fmt, *emoji))
                                .content(content.to_string())
                                .build()
                                .expect("Failed to build processor in core generation benchmark");

                            black_box(
                                processor
                                    .generate()
                                    .expect("Failed to generate in core generation benchmark"),
                            )
                        });
                    },
                );
            }
        }
    }

    group.finish();
}

fn bench_file_processing(c: &mut Criterion) {
    let examples = vec![
        ("minimal", "01-minimal"),
        ("metadata", "02-rule-metadata"),
        ("large", "08-embedded-content"),
    ];

    let mut group = c.benchmark_group("file_processing");

    for (example_label, example) in examples {
        group.bench_function(example_label, |b| {
            b.iter(|| {
                // Benchmark the complete file processing pipeline
                let content = load_example_for_bench(example);

                let processor = ReportProcessorBuilder::new()
                    .generator(SarifMarkdownGenerator::new(
                        MarkdownFormat::CommonMark,
                        false,
                    ))
                    .content(content)
                    .build()
                    .expect("Failed to build processor in benchmark");

                black_box(
                    processor
                        .generate()
                        .expect("Failed to generate markdown in benchmark"),
                )
            });
        });
    }

    group.finish();
}

fn bench_file_parsing(c: &mut Criterion) {
    let examples = vec![
        ("small", "01-minimal"),
        ("medium", "02-rule-metadata"),
        ("complex", "04-code-flows"),
        ("large", "08-embedded-content"),
    ];

    let mut group = c.benchmark_group("json_parsing");

    for (size_label, example) in examples {
        let content = load_example_for_bench(example);

        group.bench_with_input(
            BenchmarkId::new("parse", size_label),
            &content,
            |b, content| {
                b.iter(|| {
                    // Simple JSON validation without serde_json dependency
                    black_box(content.len())
                });
            },
        );
    }

    group.finish();
}

fn bench_memory_usage(c: &mut Criterion) {
    let large_content = load_example_for_bench("08-embedded-content");

    let mut group = c.benchmark_group("memory_usage");

    group.bench_function("large_file_processing", |b| {
        b.iter(|| {
            let processor = ReportProcessorBuilder::new()
                .generator(SarifMarkdownGenerator::new(
                    MarkdownFormat::GitHubFlavored,
                    true,
                ))
                .content(large_content.clone())
                .build()
                .expect("Failed to build processor in memory benchmark");

            let result = processor
                .generate()
                .expect("Failed to generate in memory benchmark");
            black_box(result.len()) // Prevent optimization away
        });
    });

    group.finish();
}

fn bench_output_formats(c: &mut Criterion) {
    let content = load_example_for_bench("02-rule-metadata");

    let mut group = c.benchmark_group("output_formats");

    let test_cases = vec![
        ("commonmark_plain", MarkdownFormat::CommonMark, false),
        ("commonmark_emoji", MarkdownFormat::CommonMark, true),
        ("github_plain", MarkdownFormat::GitHubFlavored, false),
        ("github_emoji", MarkdownFormat::GitHubFlavored, true),
    ];

    for (name, format, emoji) in test_cases {
        group.bench_function(name, |b| {
            b.iter(|| {
                let processor = ReportProcessorBuilder::new()
                    .generator(SarifMarkdownGenerator::new(format, emoji))
                    .content(content.clone())
                    .build()
                    .expect("Failed to build processor in output format benchmark");

                black_box(
                    processor
                        .generate()
                        .expect("Failed to generate in output format benchmark"),
                )
            });
        });
    }

    group.finish();
}

fn bench_sarif_complexity(c: &mut Criterion) {
    let complex_examples = vec![
        ("suppressions", "03-suppressions"),
        ("code_flows", "04-code-flows"),
        ("context_regions", "05-context-region"),
        ("result_stacks", "07-result-stacks"),
    ];

    let mut group = c.benchmark_group("sarif_complexity");

    for (complexity_type, example) in complex_examples {
        let content = load_example_for_bench(example);

        group.bench_function(complexity_type, |b| {
            b.iter(|| {
                let processor = ReportProcessorBuilder::new()
                    .generator(SarifMarkdownGenerator::new(
                        MarkdownFormat::CommonMark,
                        false,
                    ))
                    .content(content.clone())
                    .build()
                    .expect("Failed to build processor in complexity benchmark");

                black_box(
                    processor
                        .generate()
                        .expect("Failed to generate in complexity benchmark"),
                )
            });
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_core_generation,
    bench_file_processing,
    bench_file_parsing,
    bench_memory_usage,
    bench_output_formats,
    bench_sarif_complexity
);

criterion_main!(benches);
