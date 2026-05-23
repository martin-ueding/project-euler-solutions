mod primes;
mod registry;
mod solutions;
mod specnum;

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
            measure(entry);
        }
    }
}

fn measure(entry: &registry::SolutionEntry) {
    if let Some(name) = entry.name {
        println!("Implementation: {name}");
    }
    let benchmark_start = Instant::now();
    let mut result = 0;
    let mut timings_s = Vec::<f64>::new();
    while benchmark_start.elapsed().as_secs_f64() < 1.0 && timings_s.len() < 100 {
        let start = Instant::now();
        result = (entry.solve)();
        timings_s.push(start.elapsed().as_secs_f64())
    }
    timings_s.sort_by(|a, b| a.total_cmp(b));

    let emoji = match entry.solution {
        Some(target) => {
            if target == result {
                "✔️"
            } else {
                "❌"
            }
        }
        None => "❔",
    };

    println!("Solution: {result} {emoji}");
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
    let ns: f64 = seconds * 1_000_000_000.0;

    if seconds >= 10.0 {
        format!("{seconds:.1} s")
    } else if seconds >= 1.0 {
        format!("{seconds:.2} s")
    } else if ms >= 100.0 {
        format!("{ms:.0} ms")
    } else if ms >= 10.0 {
        format!("{ms:.1} ms")
    } else if ms >= 1.0 {
        format!("{ms:.2} ms")
    } else if us >= 100.0 {
        format!("{us:.0} µs")
    } else if us >= 10.0 {
        format!("{us:.1} µs")
    } else if us >= 1.0 {
        format!("{us:.2} µs")
    } else if ns >= 100.0 {
        format!("{ns:.0} ns")
    } else if ns >= 10.0 {
        format!("{ns:.1} ns")
    } else if ns >= 1.0 {
        format!("{ns:.2} ns")
    } else {
        format!("{ns:.6} ns")
    }
}
