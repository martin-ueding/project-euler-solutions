use crate::primes::{PrimeList, get_num_divisors};

fn solution_iterator() -> i64 {
    let mut prime_generator = PrimeList::new();
    for n in 0.. {
        let triangle_number = n * (n + 1) / 2;
        if get_num_divisors(triangle_number, &mut prime_generator) > 500 {
            return triangle_number;
        }
    }
    0
}

fn solution_coprime() -> i64 {
    let mut prime_generator = PrimeList::new();
    for n in 1.. {
        let triangle_number = n * (n + 1) / 2;
        let num_divisors = if triangle_number % 2 == 0 {
            get_num_divisors(n / 2, &mut prime_generator)
                * get_num_divisors(n + 1, &mut prime_generator)
        } else {
            get_num_divisors((n + 1) / 2, &mut prime_generator)
                * get_num_divisors(n, &mut prime_generator)
        };

        if num_divisors > 500 {
            return triangle_number;
        }
    }
    0
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 12,
        implementations: &[("coprime", solution_coprime)],
        solution: Some(1786995),
    }
}
