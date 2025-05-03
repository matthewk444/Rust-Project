/// Computes Euclidean distance between two feature vectors
pub fn euclidean_distance(a: Vec<f64>, b: Vec<f64>) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| (x - y).powi(2)).sum::<f64>().sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    use csv::Reader;

    #[test]
    fn test_euclidean_distance_zero() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        let dist = euclidean_distance(a, b);
        assert!((dist - 0.0).abs() < 1e-6);
    }

    #[test]
    fn test_euclidean_distance_known() {
        let a = vec![0.0, 0.0];
        let b = vec![3.0, 4.0];
        let dist = euclidean_distance(a, b);
        assert!((dist - 5.0).abs() < 1e-6);
    }

    #[test]
    fn test_euclidean_distance_small_diff() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.1, 3.0];
        let dist = euclidean_distance(a, b);
        assert!((dist - 0.1).abs() < 1e-6);
    }

    #[test]
    fn test_csv_not_empty() {
        let mut rdr = Reader::from_path("ObesityDataSet_raw_and_data_sinthetic.csv")
            .expect("Failed to open CSV file");
        let records: Vec<_> = rdr.records().collect::<Result<_, _>>()
            .expect("Failed to read records from CSV");
        assert!(!records.is_empty(), "CSV file should not be empty");
    }
}
