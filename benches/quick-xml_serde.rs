//! Benchmarking quick-xml serde performance with UCI messages.
//!

mod msg_utils;

use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use msg_utils::service_status::{ServiceStatus, service_status};
use quick_xml::{de, se};
use std::time::Duration;

fn quick_xml_bench(c: &mut Criterion) {
	let mut group = c.benchmark_group("quick_xml_bench");
	group.throughput(Throughput::Elements(1));
	group.warm_up_time(Duration::from_secs(5));
	group.sample_size(500);
	group.measurement_time(Duration::from_secs(30));

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

criterion_group!(benches, quick_xml_bench);
criterion_main!(benches);
