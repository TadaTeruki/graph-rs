use std::{collections::BinaryHeap, error::Error, io};

/// An edge-attributed undirected graph.
pub struct EdgeAttributedUndirectedGraph<Attr: Copy + Clone> {
    adjacency_list: Vec<Vec<(usize, Attr)>>,
    edge_count: usize,
}

#[derive(Debug, PartialEq)]
struct PathVertex<Cmp: PartialOrd> {
    vertex: usize,
    cost: Cmp,
}

impl<Cmp: PartialOrd> Eq for PathVertex<Cmp> {}

impl<Cmp: PartialOrd> Ord for PathVertex<Cmp> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cost.partial_cmp(&other.cost).unwrap_or(std::cmp::Ordering::Equal)
    }
}

impl<Cmp: PartialOrd> PartialOrd for PathVertex<Cmp> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

pub enum PathFinishCase {
    Vertex(usize),
    Func(fn(usize) -> bool)
}

impl<Attr: Copy + Clone> EdgeAttributedUndirectedGraph<Attr>{
    /// Initialize an empty graph with order vertices.
    pub fn new(order: usize) -> Self {
        EdgeAttributedUndirectedGraph {
            adjacency_list: vec![vec![]; order],
            edge_count: 0,
        }
    }

    /// Add an edge between vertices v and w, with a specified weight.
    pub fn add_edge(&mut self, v: usize, w: usize, attr: Attr) {
        self.adjacency_list[v].push((w, attr));
        self.adjacency_list[w].push((v, attr));
        self.edge_count += 1;
    }

    /// Remove the edge between vertices v and w.
    pub fn delete_edge(&mut self, v: usize, w: usize) {
        self.adjacency_list[v].retain(|&(x, _)| x != w);
        self.adjacency_list[w].retain(|&(x, _)| x != v);
        self.edge_count -= 1;
    }

    /// Check if there is an edge between vertices v and w.
    pub fn has_edge(&self, v: usize, w: usize) -> (bool, Option<Attr>) {
        for &(x, attr) in self.adjacency_list[v].iter() {
            if x == w {
                return (true, Some(attr));
            }
        }
        (false, None)
    }

    /// Return the number of vertices in the graph.
    pub fn order(&self) -> usize {
        self.adjacency_list.len()
    }

    /// Return a reference to the adjacency list of vertex v.
    pub fn neighbors_of(&self, v: usize) -> &Vec<(usize, Attr)> {
        self.adjacency_list[v].as_ref()
    }

    /// Return the number of edges in the graph.
    pub fn size(&self) -> usize {
        self.edge_count
    }

    /// Remove all edges from the graph.
    pub fn clear(&mut self) {
        for v in self.adjacency_list.iter_mut() {
            v.clear();
        }
        self.edge_count = 0;
    }

    /// Return the degree of vertex v.
    pub fn degree(&self, v: usize) -> usize {
        self.adjacency_list[v].len()
    }

    /// Create the shortest path between two vertices, using a cost function.
    pub fn create_shortest_path<Cmp: Ord + Default>(&self, v_start: usize, finish_case: PathFinishCase, cost_func: fn(Attr) -> Cmp) -> Result<(Vec<usize>, Vec<Attr>), Box<dyn Error>>{
        if v_start > self.order() {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "cannot create path; vertices are out of bounds",
            )));
        }

        if let PathFinishCase::Vertex(v_finish) = finish_case {
            if v_finish > self.order() {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "cannot create path; vertices are out of bounds",
                )));
            }

            if v_start == v_finish {
                return Ok((vec![v_start], vec![]));
            }
        }

        // TODO: use fibonacci heap instead of binary heap
        let mut priority_queue: BinaryHeap<PathVertex<Cmp>> = BinaryHeap::new();
        let mut visited: Vec<bool> = vec![false; self.order()];
        let mut previous: Vec<Option<usize>> = vec![None; self.order()];

        priority_queue.push(PathVertex { vertex: v_start, cost: Cmp::default() });
        visited[v_start] = true;
        previous[v_start] = Some(v_start);

        let mut v_finish = 0;
        while let Some(sp) = priority_queue.pop() {
            let v = sp.vertex;
            match finish_case {
                PathFinishCase::Vertex(v_fin) => {
                    if v == v_fin {
                        v_finish = v_fin;
                        break;
                    }
                }
                PathFinishCase::Func(f) => {
                    if f(v) {
                        v_finish = v;
                        break;
                    }
                }
            }
            for &(w, attr) in self.neighbors_of(v) {
                if visited[w] { continue; }
                priority_queue.push(PathVertex { vertex: w, cost: cost_func(attr) });
                visited[w] = true;
                previous[w] = Some(v);
            }
        }

        if previous[v_finish] == None {
            return Err(Box::new(io::Error::new(
                io::ErrorKind::InvalidInput,
                "cannot create path; no path exists",
            )));
        }

        let mut path: Vec<usize> = vec![];
        let mut path_attr: Vec<Attr> = vec![];
        let mut v = v_finish;
        while v != v_start {
            path.push(v);

            let (has_edge, attr) = {
                if let Some(prev) = previous[v] {
                    self.has_edge(v, prev)
                }
                else {
                    (false, None)
                }
            };
            if !has_edge {
                return Err(Box::new(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "cannot create path; no path exists",
                )));
            }
            path_attr.push(attr.unwrap());
            v = previous[v].unwrap();
        };
        path.push(v_start);

        path.reverse();
        path_attr.reverse();
        Ok((path, path_attr))
    }
}
