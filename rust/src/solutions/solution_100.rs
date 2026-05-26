//! Arranged Probability (Problem 100)

fn solution() -> i64 {
    // Start with the fundamental solution to the negative Pell's equation.
    let mut x: i64 = 1;
    let mut y: i64 = 1;

    loop {
        // Relate back to the number of disks `n` and the number of blue disks `b`.
        let n = (x + 1) / 2;
        let b = (y + 1) / 2;

        // If we exceed the limit, we're done.
        if n > 1_000_000_000_000 {
            return b;
        }

        // Recursion relation.
        let x_new = 3 * x + 4 * y;
        let y_new = 3 * y + 2 * x;
        x = x_new;
        y = y_new;
    }
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 100,
        implementations: &[("recursion relation", solution)],
        solution: Some(756_872_327_473),
    }
}
