//! Handles loading and preprocessing the dataset.

use std::collections::{HashMap, HashSet};
use std::error::Error;
use csv::{Reader, StringRecord};
use ndarray::Array2;

/// Represents the dataset with normalized features and raw records.
pub struct Dataset {
    pub data: Array2<f64>,
    pub records: Vec<StringRecord>,
    pub headers: StringRecord,
}


/// `choice`: 
/// - 1 = all features
/// - 2 = eating-related features
/// - 3 = physical-related features
pub fn load_data(path: &str, choice: usize) -> Result<Dataset, Box<dyn Error>> {
    let mut rdr = Reader::from_path(path)?;
    let headers = rdr.headers()?.clone();
    let mut records: Vec<StringRecord> = Vec::new();
    for result in rdr.records() {
        records.push(result?);
    }

    // Pick columns to use based on choice
    let selected_columns: Vec<usize> = match choice {
        2 => vec![4, 5, 8, 10, 11, 13],
        3 => vec![2, 3, 6, 7, 9, 12],
        _ => (0..headers.len() - 1).collect(),
    };

    // Find categorical columns
    let mut categorical_cols = vec![];
    for &i in &selected_columns {
        if records.iter().any(|r| r.get(i).unwrap_or("").parse::<f64>().is_err()) {
            categorical_cols.push(i);
        }
    }

    // Collect categories for one-hot encoding
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

    // Create feature matrix with one-hot encoding and raw numeric values
    let n_samples = records.len();
    let num_numeric = selected_columns.len() - categorical_cols.len();
    let cat_total: usize = categories.values().map(|s| s.len()).sum();
    let feature_len = num_numeric + cat_total;
    let mut data = Array2::zeros((n_samples, feature_len));

    for (i, rec) in records.iter().enumerate() {
        let mut f_idx = 0;

        // Fill numeric values
        for &j in &selected_columns {
            if !categorical_cols.contains(&j) {
                let v = rec.get(j).unwrap_or("").parse::<f64>().unwrap_or(0.0);
                data[[i, f_idx]] = v;
                f_idx += 1;
            }
        }

        // Fill one-hot categorical values
        for &ci in &categorical_cols {
            let vals = &categories[&ci];
            let cur = rec.get(ci).unwrap_or("");
            for v in vals {
                data[[i, f_idx]] = if v == cur { 1.0 } else { 0.0 };
                f_idx += 1;
            }
        }
    }

    // Normalize each column to range [0, 1]
    for mut col in data.columns_mut() {
        let min = col.fold(f64::INFINITY, |a, &b| a.min(b));
        let max = col.fold(f64::NEG_INFINITY, |a, &b| a.max(b));
        if max > min {
            col.mapv_inplace(|x| (x - min) / (max - min));
        }
    }

    Ok(Dataset { data, records, headers })
}