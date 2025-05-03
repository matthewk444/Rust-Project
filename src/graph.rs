//! Builds and analyzes a graph based on similarities in the dataset.

use petgraph::graph::UnGraph;
use petgraph::algo::dijkstra;
use crate::data::Dataset;
use crate::util::euclidean_distance;

/// Constructs an undirected graph where nodes are data points,
/// and edges connect similar points based on Euclidean distance.
///
/// Input: feature matrix (ndarray)
/// Output: graph with edges between points within a distance threshold
pub fn build_graph(data: &ndarray::Array2<f64>) -> UnGraph<usize, f64> {
    let mut graph = UnGraph::<usize, f64>::new_undirected();
    let nodes: Vec<_> = (0..data.nrows()).map(|i| graph.add_node(i)).collect();
    let threshold = 0.5;

    // Connect nodes with edges if their distance is below threshold
    for i in 0..data.nrows() {
        for j in (i + 1)..data.nrows() {
            let dist = euclidean_distance(data.row(i).to_vec(), data.row(j).to_vec());
            if dist <= threshold {
                graph.add_edge(nodes[i], nodes[j], dist);
            }
        }
    }

    graph
}

/// Performs basic analysis on the graph:
/// - Runs Dijkstra from node 0
/// - Prints class labels (e.g. obesity level)
/// - Prints class distribution
/// - Finds furthest connected pair (diameter estimate)
pub fn analyze_graph(graph: &UnGraph<usize, f64>, dataset: &Dataset) {
    println!("Graph: {} nodes, {} edges", graph.node_count(), graph.edge_count());

    let dists = dijkstra(graph, petgraph::graph::NodeIndex::new(0), None, |e| *e.weight());

    println!("\nDijkstra traversal from node 0:");
    if let Some(pos) = dataset.headers.iter().position(|h| h == "NObeyesdad") {
        for (node_idx, dist) in dists.iter() {
            let rec = &dataset.records[node_idx.index()];
            let label = rec.get(pos).unwrap_or("?");
            println!("Node {} (distance {:.2}): Obesity level = {}", node_idx.index(), dist, label);
        }
    }

    // Count how many records belong to each class
    let mut dist = std::collections::HashMap::new();
    if let Some(pos) = dataset.headers.iter().position(|h| h == "NObeyesdad") {
        for rec in &dataset.records {
            *dist.entry(rec.get(pos).unwrap_or("").to_string()).or_insert(0) += 1;
        }
        println!("\nClass distribution: {:?}", dist);
    }

    find_furthest_pair(graph, dataset);
}

/// Estimates the graph diameter by finding the longest shortest path
/// among the first 100 nodes.
fn find_furthest_pair(graph: &UnGraph<usize, f64>, dataset: &Dataset) {
    let mut max_distance = 0.0;
    let mut max_pair = (0, 0);

    // Run Dijkstra from first 100 nodes and find max distance
    for node in graph.node_indices().take(100) {
        let distances = dijkstra(graph, node, None, |e| *e.weight());
        for (target, dist) in distances {
            if dist > max_distance {
                max_distance = dist;
                max_pair = (node.index(), target.index());
            }
        }
    }

    println!(
        "Max path length (graph diameter estimate): {:.2} between nodes {} and {}",
        max_distance, max_pair.0, max_pair.1
    );

    // Print full data for both endpoints of the furthest pair
    if let Some(_pos) = dataset.headers.iter().position(|h| h == "NObeyesdad") {
        let rec1 = &dataset.records[max_pair.0];
        let rec2 = &dataset.records[max_pair.1];

        println!("\nNode {} data:", max_pair.0);
        for (h, v) in dataset.headers.iter().zip(rec1.iter()) {
            println!("  {}: {}", h, v);
        }

        println!("\nNode {} data:", max_pair.1);
        for (h, v) in dataset.headers.iter().zip(rec2.iter()) {
            println!("  {}: {}", h, v);
        }
    }
}