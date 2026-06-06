use std::collections::HashSet;

fn divisor_sums() -> Vec<i64> {
    let mut result: Vec<i64> = vec![0; 1_000_001];
    for divisor in 1..1_000_001i64 {
        for multiplier in 2.. {
            let index = divisor * multiplier;
            if index >= result.len() as i64 {
                break;
            }
            result[index as usize] += divisor;
        }
    }
    result
}

fn get_chain_length(start: i64, next: &mut Vec<i64>) -> Option<(i64, i64)> {
    let mut explored: HashSet<i64> = HashSet::new();
    let mut number = start;
    loop {
        explored.insert(number);
        number = next[number as usize];
        if number == 0 || number > 1_000_000 || explored.contains(&number) {
            break;
        }
    }
    if number == start {
        let chain_length = explored.len() as i64;
        let min_element = *explored.iter().min().unwrap();
        return Some((chain_length, min_element));
    } else if number == 0 {
        explored.into_iter().for_each(|n| next[n as usize] = 0);
    }
    None
}

fn solution_sieve() -> i64 {
    let mut next = divisor_sums();
    (1..1_000_001)
        .map(|start| get_chain_length(start, &mut next))
        .flatten()
        .max()
        .unwrap()
        .1
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 95,
        implementations: &[("", solution_sieve)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        assert_eq!(solution_sieve(), 14_316);
    }
}
