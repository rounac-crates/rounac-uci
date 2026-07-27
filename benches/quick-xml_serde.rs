//! Benchmarking quick-xml serde performance with UCI messages.
//!

mod msg_utils;

use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use msg_utils::service_status::{ServiceStatus, service_status};
use quick_xml::{de, se};

fn service_status_bench(c: &mut Criterion) {
	let mut group = c.benchmark_group("service_status");
	group.throughput(Throughput::Elements(1));

	// Benchmark serialize
	group.bench_function(
		BenchmarkId::new("ss_serialize", format_args!("random")),
		|b| {
			b.iter_batched_ref(
				|| service_status(),
				|msg| se::to_string(msg).unwrap(),
				BatchSize::SmallInput,
			);
		},
	);

	// Benchmark deserialize
	group.bench_function(
		BenchmarkId::new("ss_deserialize", format_args!("random")),
		|b| {
			b.iter_batched_ref(
				|| se::to_string(&service_status()).unwrap(),
				|msg| de::from_str::<ServiceStatus>(msg).unwrap(),
				BatchSize::SmallInput,
			);
		},
	);
}

criterion_group!(service_status_group, service_status_bench);
criterion_main!(service_status_group);
