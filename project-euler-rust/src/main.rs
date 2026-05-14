mod solutions;

use std::time::Instant;

fn measure<F>(f: F) -> i32
where
    F: Fn() -> i32,
{
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();

    let duration = format_duration(elapsed.as_secs_f64());
    println!("Duration: {duration}");

    result
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

fn main() {
    let solution = measure(solutions::s1::solution_1);
    println!("Solution: {solution}");
}
