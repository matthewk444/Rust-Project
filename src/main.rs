mod data;
mod graph;
mod util;

use std::error::Error;
use std::io;

fn main() -> Result<(), Box<dyn Error>> {
    println!("Choose graph type: (1) All features, (2) Eating only, (3) Physical only");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let choice: usize = input.trim().parse().unwrap_or(1);

    let dataset = data::load_data("ObesityDataSet_raw_and_data_sinthetic.csv", choice)?;
    let graph = graph::build_graph(&dataset.data);

    graph::analyze_graph(&graph, &dataset);

    Ok(())
}