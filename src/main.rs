use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io;
use csv::{Reader, StringRecord};
use petgraph::graph::UnGraph;
use petgraph::algo::dijkstra;
use ndarray::{Array2, Axis};
use ndarray_stats::QuantileExt;
use petgraph::visit::EdgeRef;


fn main() -> Result<(), Box<dyn Error>> {
    // Ask user which graph type to build
    println!("Choose graph type: (1) All features, (2) Eating only, (3) Physical only");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice: usize = input.trim().parse().unwrap_or(1);

    // Load CSV and header
    let mut rdr = Reader::from_path("ObesityDataSet_raw_and_data_sinthetic.csv")?;
    let headers = rdr.headers()?.clone();
    let mut records: Vec<StringRecord> = Vec::new();
    for result in rdr.records() {
        let rec = result?;
        records.push(rec);
    }
    let n_samples = records.len();
    if let Some(pos) = headers.iter().position(|h| h == "NObeyesdad") {
        let mut dist = HashMap::new();
        for rec in &records {
            let obesity_level = rec.get(pos).unwrap_or("").to_string();
            *dist.entry(obesity_level).or_insert(0) += 1;
        }
        println!("Class distribution: {:?}", dist);
    }

    // Filter columns based on choice
    let selected_columns: Vec<usize> = match choice {
        2 => vec![4, 5, 8, 10, 11, 13], // Eating habits only
        3 => vec![2, 3, 6, 7, 9, 12],   // Physical condition only
        _ => (0..headers.len() - 1).collect(), // All except target
    };

    // Identify categorical columns in selection
    let mut categorical_cols: Vec<usize> = Vec::new();
    for &i in &selected_columns {
        if records.iter().any(|r| r.get(i).unwrap_or("").parse::<f64>().is_err()) {
            categorical_cols.push(i);
        }
    }

    // Build unique values map per categorical column
    let mut categories: HashMap<usize, HashSet<String>> = HashMap::new();
    for &ci in &categorical_cols {
        let mut set = HashSet::new();
        for rec in &records {
            if let Some(val) = rec.get(ci) {
                set.insert(val.to_string());
            }
        }
        categories.insert(ci, set);
    }

    // Prepare feature matrix
    let num_numeric = selected_columns.len() - categorical_cols.len();
    let cat_total: usize = categories.values().map(|s| s.len()).sum();
    let feature_len = num_numeric + cat_total;
    let mut data = Array2::zeros((n_samples, feature_len));

    for (i, rec) in records.iter().enumerate() {
        let mut f_idx = 0;
        for &j in &selected_columns {
            if !categorical_cols.contains(&j) {
                let v = rec.get(j).unwrap_or("").parse::<f64>().unwrap_or(0.0);
                data[[i, f_idx]] = v;
                f_idx += 1;
            }
        }
        for &ci in &categorical_cols {
            let vals = &categories[&ci];
            let cur = rec.get(ci).unwrap_or("");
            for v in vals {
                data[[i, f_idx]] = if v == cur { 1.0 } else { 0.0 };
                f_idx += 1;
            }
        }
    }

    for mut col in data.axis_iter_mut(Axis(1)) {
        let min = *col.min().unwrap();
        let max = *col.max().unwrap();
        if max > min {
            col.mapv_inplace(|x| (x - min) / (max - min));
        }
    }

    // Build graph
    let mut graph = UnGraph::<usize, f64>::new_undirected();
    let nodes: Vec<_> = (0..n_samples).map(|i| graph.add_node(i)).collect();
    let threshold = 0.5;
    for i in 0..n_samples {
        for j in (i + 1)..n_samples {
            let dist = euclidean_distance(data.row(i).to_vec(), data.row(j).to_vec());
            if dist <= threshold {
                graph.add_edge(nodes[i], nodes[j], dist);
            }
        }
    }
    println!("Graph: {} nodes, {} edges", graph.node_count(), graph.edge_count());

    // Dijkstra for shortest paths from node 0
    let dists = dijkstra(&graph, nodes[0], None, |e| *e.weight());
    println!("\nDijkstra traversal from node 0:");
    if let Some(pos) = headers.iter().position(|h| h == "NObeyesdad") {
        for (node_idx, dist) in dists.iter() {
            let rec = &records[node_idx.index()];
            let label = rec.get(pos).unwrap_or("?");
            println!("Node {} (distance {:.2}): Obesity level = {}", node_idx.index(), dist, label);
        }
    }

    // Class distribution
    let mut dist = HashMap::new();
    if let Some(pos) = headers.iter().position(|h| h == "NObeyesdad") {
        for rec in &records {
            *dist.entry(rec.get(pos).unwrap_or("").to_string()).or_insert(0) += 1;
        }
        println!("\nClass distribution: {:?}", dist);
    }
    find_furthest_pair(&graph, &records, &headers);

    Ok(())
}

fn euclidean_distance(a: Vec<f64>, b: Vec<f64>) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum::<f64>().sqrt()
}
fn find_furthest_pair(
    graph: &UnGraph<usize, f64>,
    data: &[StringRecord],
    headers: &StringRecord,
) {
    let mut max_distance = 0.0;
    let mut max_pair = (0, 0);

    for node in graph.node_indices().take(100) {
        let distances = petgraph::algo::dijkstra(&graph, node, None, |e| *e.weight());
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

    if let Some(pos) = headers.iter().position(|h| h == "NObeyesdad") {
        let rec1 = &data[max_pair.0];
        let rec2 = &data[max_pair.1];

        println!("\nNode {} data:", max_pair.0);
        for (h, v) in headers.iter().zip(rec1.iter()) {
            println!("  {}: {}", h, v);
        }

        println!("\nNode {} data:", max_pair.1);
        for (h, v) in headers.iter().zip(rec2.iter()) {
            println!("  {}: {}", h, v);
        }
    }
}
