//! Main module — handles user input, loads data, builds graph, and runs analysis.

mod data;  // Handles data loading and preprocessing
mod graph; // Handles graph construction and analysis
mod util;  // Utility functions (e.g., distance calculation)

use std::error::Error;
use std::io;

/// Main function: runs the program
/// - Asks the user to select feature type
/// - Loads and processes the data
/// - Builds and analyzes the graph
fn main() -> Result<(), Box<dyn Error>> {
    // Ask user which graph type to build
    println!("Choose graph type: (1) All features, (2) Eating only, (3) Physical only");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice: usize = input.trim().parse().unwrap_or(1); // Default to 1 if input is invalid

    // Load dataset and generate feature matrix
    let dataset = data::load_data("ObesityDataSet_raw_and_data_sinthetic.csv", choice)?;

    // Build graph from normalized data
    let graph = graph::build_graph(&dataset.data);

    // Run graph analysis (Dijkstra, class distribution, furthest nodes)
    graph::analyze_graph(&graph, &dataset);

    Ok(())
}