use bm25::{Document, Language, SearchEngineBuilder};
use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};

/// Generate synthetic conversation turns for benchmarking
fn generate_documents(count: usize) -> Vec<Document<usize>> {
    let sample_messages = vec![
        "How do I implement the BM25 algorithm in Rust?",
        "What is the best way to handle async database queries?",
        "Can you explain the tetrahedral decision framework?",
        "I need help optimizing memory retrieval performance",
        "What are the key differences between hot, warm, and cold memory?",
        "How does boundary oscillation affect quality emergence?",
        "What is the significance scoring system based on?",
        "Can you help me understand recursive recognition patterns?",
        "How do I integrate LLM providers with the dual-LLM architecture?",
        "What is the purpose of identity-critical snapshots?",
    ];

    (0..count)
        .map(|idx| {
            let content = sample_messages[idx % sample_messages.len()].to_string();
            Document {
                id: idx,
                contents: content,
            }
        })
        .collect()
}

/// Benchmark: Build search engine with various corpus sizes
fn bench_search_engine_build(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_engine_build");

    for size in [100, 500, 1000, 5000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let documents = generate_documents(size);
            b.iter(|| {
                SearchEngineBuilder::<usize>::with_documents(
                    black_box(Language::English),
                    black_box(documents.clone()),
                )
                .build()
            });
        });
    }

    group.finish();
}

/// Benchmark: Search performance with various corpus sizes
fn bench_search_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("search_query");

    let queries = vec![
        "memory retrieval",
        "BM25 algorithm",
        "tetrahedral framework",
        "dual-LLM architecture",
    ];

    for size in [100, 500, 1000, 5000].iter() {
        for query in &queries {
            let documents = generate_documents(*size);
            let search_engine =
                SearchEngineBuilder::<usize>::with_documents(Language::English, documents).build();

            let param = format!("{}_docs/{}", size, query);
            group.bench_with_input(BenchmarkId::from_parameter(&param), query, |b, query| {
                b.iter(|| search_engine.search(black_box(*query), black_box(10)));
            });
        }
    }

    group.finish();
}

/// Benchmark: End-to-end search (build + query)
fn bench_end_to_end_search(c: &mut Criterion) {
    let mut group = c.benchmark_group("end_to_end_search");

    for size in [100, 500, 1000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
            let documents = generate_documents(size);
            b.iter(|| {
                let search_engine = SearchEngineBuilder::<usize>::with_documents(
                    Language::English,
                    documents.clone(),
                )
                .build();
                search_engine.search(black_box("memory retrieval optimization"), black_box(10))
            });
        });
    }

    group.finish();
}

/// Benchmark: Query term count impact (single word vs multi-word)
fn bench_query_complexity(c: &mut Criterion) {
    let mut group = c.benchmark_group("query_complexity");

    let documents = generate_documents(1000);
    let search_engine =
        SearchEngineBuilder::<usize>::with_documents(Language::English, documents).build();

    let queries = vec![
        ("1_word", "memory"),
        ("2_words", "memory retrieval"),
        ("3_words", "memory retrieval optimization"),
        ("5_words", "How do I optimize memory retrieval"),
        (
            "10_words",
            "How do I optimize memory retrieval performance with BM25 ranking algorithm",
        ),
    ];

    for (label, query) in queries {
        group.bench_with_input(BenchmarkId::from_parameter(label), &query, |b, query| {
            b.iter(|| search_engine.search(black_box(*query), black_box(10)));
        });
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_search_engine_build,
    bench_search_query,
    bench_end_to_end_search,
    bench_query_complexity,
);
criterion_main!(benches);
