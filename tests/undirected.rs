extern crate graph;

#[test]
fn test_undirected_graph() {
    let mut graph = graph::undirected::UndirectedGraph::new(3);

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
    assert_eq!(graph.degree(0), 2);
    assert_eq!(graph.degree(1), 2);
    assert_eq!(graph.degree(2), 2);

    // Testing adjacent function.
    let mut adj_0 = graph.neighbors_of(0).clone();
    adj_0.sort();
    assert_eq!(adj_0, vec![1, 2]);

    // Removing edges.
    graph.delete_edge(0, 1);
    assert_eq!(graph.size(), 2);
    assert!(!graph.has_edge(0, 1));
    assert_eq!(graph.degree(0), 1);
    assert_eq!(graph.degree(1), 1);

    // Clearing the graph.
    graph.clear();
    assert_eq!(graph.order(), 3);
    assert_eq!(graph.size(), 0);
}
