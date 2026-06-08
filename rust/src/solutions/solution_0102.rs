use itertools::Itertools;
use std::fs;

type Point = (i64, i64);

fn solution() -> i64 {
    load_data()
        .into_iter()
        .map(|points| triangle_contains_origin(&points))
        .filter(|b| *b)
        .count() as i64
}

fn load_data() -> Vec<Vec<Point>> {
    fs::read_to_string("../data/0102_triangles.txt")
        .expect("File couldn't be found!")
        .lines()
        .map(line_to_points)
        .collect()
}

fn line_to_points(line: &str) -> Vec<Point> {
    line.split(',')
        .map(|s| s.trim().parse::<i64>().unwrap())
        .tuples()
        .collect()
}

fn triangle_contains_origin(points: &[Point]) -> bool {
    points
        .iter()
        .circular_tuple_windows()
        .map(|(a, b)| line_orientation(a, b))
        .all_equal()
}

fn line_orientation(a: &Point, b: &Point) -> i64 {
    cross_product(&(a.0 - b.0, a.1 - b.1), a).signum()
}

fn cross_product(v: &Point, w: &Point) -> i64 {
    v.0 * w.1 - v.1 * w.0
}

inventory::submit! {
    crate::registry::SolutionEntry {
        id: 102,
        implementations: &[("cross products", solution)],
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_triangle_contains_origin_does() {
        let points = vec![(-340, 495), (-153, -910), (835, -947)];
        assert!(triangle_contains_origin(&points));
    }

    #[test]
    fn test_triangle_contains_origin_doesnt() {
        let points = vec![(-175, 41), (-421, -714), (574, -645)];
        assert!(!triangle_contains_origin(&points));
    }

    #[test]
    fn test_solution() {
        assert_eq!(solution(), 228);
    }
}
