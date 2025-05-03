use petgraph::graph::UnGraph;
use petgraph::algo::dijkstra;
use crate::data::Dataset;
use crate::util::euclidean_distance;

/// Builds a graph from the dataset's feature matrix
pub fn build_graph(data: &ndarray::Array2<f64>) -> UnGraph<usize, f64> {
    let mut graph = UnGraph::<usize, f64>::new_undirected();
    let nodes: Vec<_> = (0..data.nrows()).map(|i| graph.add_node(i)).collect();
    let threshold = 0.5;
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

/// Analyzes the graph: prints class distribution and finds max distance
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

    let mut dist = std::collections::HashMap::new();
    if let Some(pos) = dataset.headers.iter().position(|h| h == "NObeyesdad") {
        for rec in &dataset.records {
            *dist.entry(rec.get(pos).unwrap_or("").to_string()).or_insert(0) += 1;
        }
        println!("\nClass distribution: {:?}", dist);
    }

    find_furthest_pair(graph, dataset);
}

/// Finds and prints the pair of nodes with the maximum distance
fn find_furthest_pair(graph: &UnGraph<usize, f64>, dataset: &Dataset) {
    let mut max_distance = 0.0;
    let mut max_pair = (0, 0);

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