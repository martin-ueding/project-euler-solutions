mod registry;
mod solutions;

use std::time::Instant;

fn main() {
    let id: i32 = std::env::args()
        .nth(1)
        .expect("missing id")
        .parse()
        .expect("invalid id");

    for entry in inventory::iter::<registry::SolutionEntry> {
        if entry.id == id {
            let (solution, duration) = measure(entry.solve);
            let duration_str = format_duration(duration);
            println!("Problem:  {id}");
            println!("Solution: {solution}");
            println!("Duration: {duration_str}");
            return;
        }
    }
}

fn measure<F>(f: F) -> (i64, f64)
where
    F: Fn() -> i64,
{
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();

    (result, elapsed.as_secs_f64())
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
