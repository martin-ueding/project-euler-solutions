use project_euler_rust::registry;

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
            for (name, implementation) in entry.implementations {
                println!("Implementation: {name}");
                check_solution(implementation(), entry.solution);
                measure(implementation);
            }
        }
    }
}

fn find_batch_size(implementation: &fn() -> i64) -> i32 {
    let mut batch_size = 1;
    loop {
        let benchmark_start = Instant::now();
        for _ in 0..batch_size {
            implementation();
        }
        if benchmark_start.elapsed().as_secs_f64() < 0.001 {
            batch_size *= 10;
        } else {
            break;
        }
    }
    batch_size
}

fn check_solution(actual: i64, solution: Option<i64>) {
    let emoji = match solution {
        Some(expected) => {
            if actual == expected {
                "✔️"
            } else {
                "❌"
            }
        }
        None => "❔",
    };

    println!("Solution: {actual} {emoji}");
}

fn measure(implementation: &fn() -> i64) {
    let batch_size = find_batch_size(implementation);
    let benchmark_start = Instant::now();
    let mut timings_s = Vec::<f64>::new();
    while benchmark_start.elapsed().as_secs_f64() < 1.0 && timings_s.len() < 100 {
        let start = Instant::now();
        for _ in 0..batch_size {
            implementation();
        }
        timings_s.push(start.elapsed().as_secs_f64() / batch_size as f64)
    }

    timings_s.sort_by(|a, b| a.total_cmp(b));
    report_timing_distribution(&timings_s);
    println!(
        "Batching: {} batches of {} iterations",
        timings_s.len(),
        batch_size
    );
}

fn report_timing_distribution(timings_s: &[f64]) {
    println!(
        "Timings: {} | {} | {} | {} | {}",
        format_duration(timings_s[0]),
        format_duration(timings_s[timings_s.len() / 4]),
        format_duration(timings_s[timings_s.len() / 2]),
        format_duration(timings_s[timings_s.len() * 3 / 4]),
        format_duration(timings_s[timings_s.len() - 1]),
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
