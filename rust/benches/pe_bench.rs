use criterion::{Criterion, criterion_group, criterion_main};
use project_euler_rust::registry::SolutionEntry;

fn bench_solutions(c: &mut Criterion) {
    let mut entries: Vec<&SolutionEntry> = inventory::iter::<SolutionEntry>().collect();
    entries.sort_by_key(|entry| entry.id);

    for &entry in &entries {
        let &SolutionEntry {
            id,
            implementations,
        } = entry;
        let mut g = c.benchmark_group(format!("{id}"));

        for &(name, f) in implementations {
            let name = if name.is_empty() { "solution" } else { name };
            g.bench_function(name, |b| b.iter(f));
        }

        g.finish();
    }
}

criterion_group!(benches, bench_solutions);
criterion_main!(benches);
