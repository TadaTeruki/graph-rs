extern crate terrain_graph;
use terrain_graph::directed::DirectedGraph;

#[test]
fn test_new() {
    let graph = DirectedGraph::new(3);
    assert_eq!(graph.order(), 3);
    assert_eq!(graph.size(), 0);
}

#[test]
fn test_add_edge() {
    let mut graph = DirectedGraph::new(3);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 2);
    assert_eq!(graph.order(), 3);
    assert_eq!(graph.size(), 3);
    assert!(graph.has_edge(0, 1));
    assert!(graph.has_edge(0, 2));
    assert!(graph.has_edge(1, 2));
    assert!(!graph.has_edge(1, 0));
    assert!(!graph.has_edge(2, 0));
    assert!(!graph.has_edge(2, 1));
}

#[test]
fn test_degree() {
    let mut graph = DirectedGraph::new(3);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 2);
    assert_eq!(graph.outdegree(0), 2);
    assert_eq!(graph.outdegree(1), 1);
    assert_eq!(graph.outdegree(2), 0);
    assert_eq!(graph.indegree(0), 0);
    assert_eq!(graph.indegree(1), 1);
    assert_eq!(graph.indegree(2), 2);
}

#[test]
fn test_neighbors_of() {
    let mut graph = DirectedGraph::new(3);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    assert_eq!(graph.neighbors_of(0), &vec![1, 2]);
}

#[test]
fn test_delete_edge() {
    let mut graph = DirectedGraph::new(3);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.delete_edge(0, 1);
    assert_eq!(graph.size(), 1);
    assert!(!graph.has_edge(0, 1));
    assert_eq!(graph.outdegree(0), 1);
    assert_eq!(graph.indegree(1), 0);
}

#[test]
fn test_clear() {
    let mut graph = DirectedGraph::new(3);
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.clear();
    assert_eq!(graph.order(), 3);
    assert_eq!(graph.size(), 0);
}