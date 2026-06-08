type Point = (i64, i64);

fn lagrange_polynomial(x: f64, points: &[Point]) -> f64 {
    let result: f64 = points
        .iter()
        .enumerate()
        .map(|(i, pi)| pi.1 as f64 * base_polynomial(x, i, points))
        .sum();
    result.round()
}

fn base_polynomial(x: f64, i: usize, points: &[Point]) -> f64 {
    points
        .iter()
        .enumerate()
        .filter(|(j, _p_j)| *j != i)
        .map(|(_j, p_j)| (x - p_j.0 as f64) / (points[i].0 as f64 - p_j.0 as f64))
        .fold(1_f64, |acc, elem| acc * elem)
}

fn reference_polynomial(x: i64) -> i64 {
    1 - x + x.pow(2) - x.pow(3) + x.pow(4) - x.pow(5) + x.pow(6) - x.pow(7) + x.pow(8) - x.pow(9)
        + x.pow(10)
}

fn get_first_incorrect_term(points: &[Point], k: usize) -> Option<i64> {
    points.iter().find_map(|p| {
        let prediction = lagrange_polynomial(p.0 as f64, &points[0..k]) as i64;
        (prediction != p.1).then_some(prediction)
    })
}

fn sum_first_incorrect_terms(points: &[Point]) -> i64 {
    (1..points.len() + 1)
        .map(|k| get_first_incorrect_term(&points, k as usize))
        .flatten()
        .sum()
}

fn solution() -> i64 {
    let points: Vec<Point> = (1..12).map(|x| (x, reference_polynomial(x))).collect();
    sum_first_incorrect_terms(&points)
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 101,
        implementations: &[("Lagrange polynomial", solution)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_first_incorrect_terms() {
        let points = vec![(1, 1), (2, 8), (3, 27), (4, 64), (5, 125), (6, 216)];
        assert_eq!(sum_first_incorrect_terms(&points), 74);
    }

    #[test]
    fn test_solution() {
        assert_eq!(solution(), 37_076_114_526);
    }
}
