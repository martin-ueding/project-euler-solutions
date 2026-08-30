use std::fs;

use crate::graphs::{Edge, Graph, Vertex, VertexID};

pub fn load_triangle_numbers(path: &str) -> Vec<Vec<i32>> {
    fs::read_to_string(path)
        .expect("File couldn't be found!")
        .lines()
        .map(line_to_integers)
        .collect()
}

fn line_to_integers(line: &str) -> Vec<i32> {
    line.split(' ')
        .map(|s| s.trim().parse::<i32>().unwrap())
        .collect()
}

pub struct TriangleGraph {
    pub lines: Vec<Vec<i32>>,
}

pub const VIRTUAL_END_VERTEX_ID: VertexID = 1000000;

impl Graph for TriangleGraph {
    fn edges(&self, v: &VertexID) -> Vec<Edge> {
        if *v == VIRTUAL_END_VERTEX_ID {
            vec![]
        } else {
            let row = v / self.lines.len();
            if row == self.lines.len() - 1 {
                vec![Edge::new(*v, VIRTUAL_END_VERTEX_ID, 0)]
            } else {
                vec![
                    Edge::new(*v, v + self.lines.len(), 0),
                    Edge::new(*v, v + self.lines.len() + 1, 0),
                ]
            }
        }
    }

    fn vertex(&self, id: &VertexID) -> Vertex {
        if *id == VIRTUAL_END_VERTEX_ID {
            Vertex::new(*id, 0)
        } else {
            let row = id / self.lines.len();
            let col = id % self.lines.len();
            Vertex::new(*id, -self.lines[row][col])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn loads_triangle() {
        let triangle = load_triangle_numbers("../data/0018_triangle.txt");
        assert_eq!(triangle[0], &[75]);
        assert_eq!(triangle[1], &[95, 64]);
        assert_eq!(triangle[4], &[20, 4, 82, 47, 65]);
    }
}
