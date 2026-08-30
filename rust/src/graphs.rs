use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
};

type VertexID = i32;
type Weight = i32;

#[derive(Clone)]
pub struct Edge {
    from: i32,
    to: i32,
    weight: Weight,
}

impl Edge {
    pub fn new(from: i32, to: i32, weight: Weight) -> Self {
        Edge { from, to, weight }
    }
}

#[derive(Clone)]
pub struct Vertex {
    id: VertexID,
    weight: Weight,
}

impl Vertex {
    pub fn new(id: VertexID, weight: Weight) -> Self {
        Vertex { id, weight }
    }
}

pub trait Graph {
    fn edges(&self, v: &VertexID) -> &[Edge];
    fn vertex(&self, id: &VertexID) -> &Vertex;
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct UnvisitedVertex {
    total_distance: Weight,
    id: VertexID,
}

pub fn min_distance(g: &dyn Graph, start: &Vertex, target: &Vertex) -> Weight {
    let mut unvisited_vertices: BinaryHeap<Reverse<UnvisitedVertex>> = BinaryHeap::new();
    unvisited_vertices.push(Reverse(UnvisitedVertex {
        total_distance: start.weight,
        id: start.id,
    }));
    let mut total_distances: HashMap<VertexID, Weight> = HashMap::new();
    total_distances.insert(start.id, start.weight);
    while !unvisited_vertices.is_empty() {
        let cur = unvisited_vertices
            .pop()
            .expect("We ran out of reachable unvisited vertices.")
            .0;
        for edge in g.edges(&cur.id) {
            let vertex = g.vertex(&edge.to);
            let distance = cur.total_distance + edge.weight + vertex.weight;
            if total_distances.contains_key(&vertex.id) {
                if distance
                    < *total_distances
                        .get(&vertex.id)
                        .expect("We've just checked.")
                {
                    total_distances.insert(vertex.id, distance);
                    unvisited_vertices.push(Reverse(UnvisitedVertex {
                        total_distance: distance,
                        id: vertex.id,
                    }))
                }
            } else {
                total_distances.insert(vertex.id, distance);
                unvisited_vertices.push(Reverse(UnvisitedVertex {
                    total_distance: distance,
                    id: vertex.id,
                }))
            }
        }
    }
    *total_distances
        .get(&target.id)
        .expect("We ran until we reached the target.")
}

pub struct ExplicitGraph {
    vertices: HashMap<VertexID, Vertex>,
    edges: HashMap<VertexID, Vec<Edge>>,
}

impl ExplicitGraph {
    pub fn new(vertices: &[Vertex], edges: &[Edge]) -> Self {
        let mut vertex_map: HashMap<VertexID, Vertex> = HashMap::new();
        let mut edge_map: HashMap<VertexID, Vec<Edge>> = HashMap::new();
        for vertex in vertices {
            vertex_map.insert(vertex.id, vertex.clone());
        }
        for edge in edges {
            if !edge_map.contains_key(&edge.from) {
                edge_map.insert(edge.from, vec![]);
            }
            edge_map
                .get_mut(&edge.from)
                .expect("We've just inserted this array.")
                .push(edge.clone());
        }
        ExplicitGraph {
            vertices: vertex_map,
            edges: edge_map,
        }
    }
}

impl Graph for ExplicitGraph {
    fn edges(&self, v: &VertexID) -> &[Edge] {
        self.edges.get(v).map(Vec::as_slice).unwrap_or(&[])
    }

    fn vertex(&self, id: &VertexID) -> &Vertex {
        self.vertices
            .get(id)
            .expect("Given vertex id must be contained in graph.")
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn dijkstra_solves_explicit_graph() {
        let graph = ExplicitGraph::new(
            &vec![
                Vertex::new(1, 2),
                Vertex::new(2, 2),
                Vertex::new(3, 2),
                Vertex::new(4, 2),
            ],
            &vec![
                Edge::new(1, 2, 1),
                Edge::new(1, 3, 10),
                Edge::new(2, 3, 2),
                Edge::new(2, 4, 10),
                Edge::new(3, 4, 3),
            ],
        );

        assert_eq!(min_distance(&graph, graph.vertex(&1), graph.vertex(&4)), 14)
    }
}
