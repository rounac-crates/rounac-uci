//! Benchmarking json serde performance with UCI messages.
//!

mod msg_utils;

use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use msg_utils::service_status::{ServiceStatus, service_status};
use serde_json::{from_str, to_string};

fn json_bench(c: &mut Criterion) {
	let mut group = c.benchmark_group("json_bench");
	group.throughput(Throughput::Elements(1));

	// Benchmark serialize
	group.bench_function(
		BenchmarkId::new("ss_serialize", format_args!("random")),
		|b| {
			b.iter_batched_ref(
				|| service_status(),
				|msg| to_string(msg).unwrap(),
				BatchSize::SmallInput,
			);
		},
	);

	// Benchmark deserialize
	group.bench_function(
		BenchmarkId::new("ss_deserialize", format_args!("random")),
		|b| {
			b.iter_batched_ref(
				|| to_string(&service_status()).unwrap(),
				|msg| from_str::<ServiceStatus>(msg).unwrap(),
				BatchSize::SmallInput,
			);
		},
	);
}

criterion_group!(benches, json_bench);
criterion_main!(benches);
