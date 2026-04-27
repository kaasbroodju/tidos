#![cfg(not(feature = "i18n"))]

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use stats_alloc::{Region, StatsAlloc, INSTRUMENTED_SYSTEM};
use std::alloc::System;
use std::hint::black_box;
use std::sync::Once;
use tidos::{page, view, Component, Page};

#[global_allocator]
static GLOBAL: &StatsAlloc<System> = &INSTRUMENTED_SYSTEM;

// ---- Components ----

struct ListItem {
	pub index: usize,
	pub label: String,
}

impl Component for ListItem {
	fn to_render(&self, _page: &mut Page) -> String {
		view! {
			<li>
				<span>{self.index.to_string()}</span>
				<span>{&self.label}</span>
			</li>
		}
	}
}

// ---- Data generators ----

fn make_labels(n: usize) -> Vec<String> {
	(0..n).map(|i| format!("Item {i}")).collect()
}

fn make_list_items(n: usize) -> Vec<ListItem> {
	(0..n)
		.map(|i| ListItem {
			index: i,
			label: format!("Label for item {i}"),
		})
		.collect()
}

// ---- Render functions ----

fn render_static() -> Page {
	page! {
		<header>
			<nav>
				<a href="/">{"Home"}</a>
				<a href="/about">{"About"}</a>
				<a href="/contact">{"Contact"}</a>
			</nav>
		</header>
		<main>
			<h1>{"Welcome"}</h1>
			<p>{"This is a static page benchmark."}</p>
		</main>
		<footer>
			<p>{"Footer content"}</p>
		</footer>
	}
}

fn render_borrowed_list(items: &[ListItem]) -> Page {
	page! {
		<ul>
			{#for item in items}
				<li>{&item.label}</li>
			{/for}
		</ul>
	}
}

fn render_range_if(n: usize) -> Page {
	page! {
		<ul>
			{#for i in 0..n}
				<li>
					{#if i % 2 == 0}
						<span>{"even"}</span>
					{:else}
						<span>{"odd"}</span>
					{/if}
				</li>
			{/for}
		</ul>
	}
}

fn render_flat_list(labels: &[String]) -> Page {
	page! {
		<ul>
			{#for label in labels}
				<li>{label}</li>
			{/for}
		</ul>
	}
}

// ---- Memory stats (printed once before speed benchmarks) ----

static PRINT_ONCE: Once = Once::new();

fn print_memory_stats() {
	PRINT_ONCE.call_once(|| {
		eprintln!("\n=== Memory allocation stats (one render call) ===");
		eprintln!(
			"{:<20} {:>6} {:>14} {:>14} {:>12}",
			"Scenario", "N", "Bytes alloc", "Alloc calls", "HTML bytes"
		);
		eprintln!("{}", "-".repeat(70));

		for &n in &[10_usize, 100, 1000] {
			let labels = make_labels(n);
			let reg = Region::new(&GLOBAL);
			let page = render_flat_list(&labels);
			let stats = reg.change();
			eprintln!(
				"{:<20} {:>6} {:>14} {:>14} {:>12}",
				"flat_list",
				n,
				stats.bytes_allocated,
				stats.allocations,
				page.template.len()
			);
		}

		let reg = Region::new(&GLOBAL);
		let page = render_static();
		let stats = reg.change();
		eprintln!(
			"{:<20} {:>6} {:>14} {:>14} {:>12}",
			"static",
			"-",
			stats.bytes_allocated,
			stats.allocations,
			page.template.len()
		);

		for &n in &[10_usize, 100, 1000] {
			let items = make_list_items(n);
			let reg = Region::new(&GLOBAL);
			let page = render_borrowed_list(&items);
			let stats = reg.change();
			eprintln!(
				"{:<20} {:>6} {:>14} {:>14} {:>12}",
				"borrowed_list",
				n,
				stats.bytes_allocated,
				stats.allocations,
				page.template.len()
			);
		}

		for &n in &[10_usize, 100, 1000] {
			let reg = Region::new(&GLOBAL);
			let page = render_range_if(n);
			let stats = reg.change();
			eprintln!(
				"{:<20} {:>6} {:>14} {:>14} {:>12}",
				"range_if",
				n,
				stats.bytes_allocated,
				stats.allocations,
				page.template.len()
			);
		}

		eprintln!();
	});
}

// ---- Criterion speed benchmarks ----

fn bench_flat_list(c: &mut Criterion) {
	print_memory_stats();

	let mut group = c.benchmark_group("flat_list");
	for &n in &[10_usize, 100, 1000] {
		let labels = make_labels(n);
		group.throughput(Throughput::Elements(n as u64));
		group.bench_with_input(BenchmarkId::from_parameter(n), &labels, |b, labels| {
			b.iter(|| black_box(render_flat_list(labels)));
		});
	}
	group.finish();
}

fn bench_static(c: &mut Criterion) {
	c.bench_function("static", |b| {
		b.iter(|| black_box(render_static()));
	});
}

fn bench_borrowed_list(c: &mut Criterion) {
	let mut group = c.benchmark_group("borrowed_list");
	for &n in &[10_usize, 100, 1000] {
		let items = make_list_items(n);
		group.throughput(Throughput::Elements(n as u64));
		group.bench_with_input(BenchmarkId::from_parameter(n), &items, |b, items| {
			b.iter(|| black_box(render_borrowed_list(items)));
		});
	}
	group.finish();
}

fn bench_range_if(c: &mut Criterion) {
	let mut group = c.benchmark_group("range_if");
	for &n in &[10_usize, 100, 1000] {
		group.throughput(Throughput::Elements(n as u64));
		group.bench_with_input(BenchmarkId::from_parameter(n), &n, |b, &n| {
			b.iter(|| black_box(render_range_if(n)));
		});
	}
	group.finish();
}

criterion_group!(
	benches,
	bench_static,
	bench_flat_list,
	bench_borrowed_list,
	bench_range_if,
);
criterion_main!(benches);
