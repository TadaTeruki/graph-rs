extern crate terrain_graph;
use terrain_graph::undirected::UndirectedGraph;

#[test]
fn test_new() {
    let graph = UndirectedGraph::new(3);
    assert_eq!(graph.order(), 3);
    assert_eq!(graph.size(), 0);
}

#[test]
fn test_add_edge() {
    let mut graph = UndirectedGraph::new(3);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 2);
    assert_eq!(graph.order(), 3);
    assert_eq!(graph.size(), 3);
    assert!(graph.has_edge(0, 1));
    assert!(graph.has_edge(0, 2));
    assert!(graph.has_edge(1, 2));
    assert!(graph.has_edge(1, 0));
    assert!(graph.has_edge(2, 0));
    assert!(graph.has_edge(2, 1));
}

#[test]
fn test_degree() {
    let mut graph = UndirectedGraph::new(3);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 2);
    assert_eq!(graph.degree(0), 2);
    assert_eq!(graph.degree(1), 2);
    assert_eq!(graph.degree(2), 2);
}

#[test]
fn test_neighbors_of() {
    let mut graph = UndirectedGraph::new(3);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    let mut adj_0 = graph.neighbors_of(0).clone();
    adj_0.sort();
    assert_eq!(adj_0, vec![1, 2]);
}

#[test]
fn test_delete_edge() {
    let mut graph = UndirectedGraph::new(3);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.delete_edge(0, 1);
    assert_eq!(graph.size(), 1);
    assert!(!graph.has_edge(0, 1));
    assert_eq!(graph.degree(0), 1);
    assert_eq!(graph.degree(1), 0);
}

#[test]
fn test_clear() {
    let mut graph = UndirectedGraph::new(3);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.clear();
    assert_eq!(graph.order(), 3);
    assert_eq!(graph.size(), 0);
}