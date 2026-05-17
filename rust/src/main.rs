mod primes;
mod registry;
mod solutions;

use std::time::Instant;

fn main() {
    let id: i32 = std::env::args()
        .nth(1)
        .expect("missing id")
        .parse()
        .expect("invalid id");
    println!("Problem: {id}");
    println!("Language: Rust");

    for entry in inventory::iter::<registry::SolutionEntry> {
        if entry.id == id {
            measure(entry.solve);
            return;
        }
    }
}

fn measure<F>(f: F)
where
    F: Fn() -> i64,
{
    let benchmark_start = Instant::now();
    let mut result = 0;
    let mut timings_s = Vec::<f64>::new();
    while benchmark_start.elapsed().as_secs_f64() < 1.0 && timings_s.len() < 100 {
        let start = Instant::now();
        result = f();
        timings_s.push(start.elapsed().as_secs_f64())
    }
    timings_s.sort_by(|a, b| a.total_cmp(b));

    println!("Solution: {result}");
    println!(
        "Timings: {} | {} | {} | {} | {} | {} iterations",
        format_duration(timings_s[0]),
        format_duration(timings_s[timings_s.len() / 4]),
        format_duration(timings_s[timings_s.len() / 2]),
        format_duration(timings_s[timings_s.len() * 3 / 4]),
        format_duration(timings_s[timings_s.len() - 1]),
        timings_s.len()
    );
}

fn format_duration(seconds: f64) -> String {
    let ms = seconds * 1_000.0;
    let us = seconds * 1_000_000.0;
    let ns = seconds * 1_000_000_000.0;

    if ms > 1.0 {
        format!("{ms:.3} ms")
    } else if us > 1.0 {
        format!("{us:.3} µs")
    } else {
        format!("{ns:.3} ns")
    }
}
