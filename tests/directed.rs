extern crate terrain_graph;

#[test]
fn test_directed_graph() {
    let mut graph = terrain_graph::directed::DirectedGraph::new(3);

    assert_eq!(graph.order(), 3);
    assert_eq!(graph.size(), 0);

    // Adding edges.
    graph.add_edge(0, 1);
    graph.add_edge(0, 2);
    graph.add_edge(1, 2);

    // Testing basic properties after adding edges.
    assert_eq!(graph.order(), 3);
    assert_eq!(graph.size(), 3);
    assert!(graph.has_edge(0, 1));
    assert!(graph.has_edge(0, 2));
    assert!(graph.has_edge(1, 2));
    assert!(!graph.has_edge(1, 0));
    assert!(!graph.has_edge(2, 0));
    assert!(!graph.has_edge(2, 1));
    assert_eq!(graph.outdegree(0), 2);
    assert_eq!(graph.outdegree(1), 1);
    assert_eq!(graph.outdegree(2), 0);
    assert_eq!(graph.indegree(0), 0);
    assert_eq!(graph.indegree(1), 1);
    assert_eq!(graph.indegree(2), 2);

    // Testing adjacent function.
    assert_eq!(graph.neighbors_of(0), &vec![1, 2]);

    // Removing edges.
    graph.delete_edge(0, 1);
    assert_eq!(graph.size(), 2);
    assert!(!graph.has_edge(0, 1));
    assert_eq!(graph.outdegree(0), 1);
    assert_eq!(graph.indegree(1), 0);

    // Clearing the graph.
    graph.clear();
    assert_eq!(graph.order(), 3);
    assert_eq!(graph.size(), 0);
}
