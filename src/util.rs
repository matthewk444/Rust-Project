//! Utility functions for various operations, including Euclidean distance computation.

/// Computes the Euclidean distance between two feature vectors.
///
/// The Euclidean distance is a measure of the "straight line" distance
/// between two points in a multi-dimensional space.
///
/// # Inputs
/// - `a`: A vector of `f64` values representing the first feature vector.
/// - `b`: A vector of `f64` values representing the second feature vector.
///
/// # Output
/// - The Euclidean distance between `a` and `b` as an `f64` value.
pub fn euclidean_distance(a: Vec<f64>, b: Vec<f64>) -> f64 {
    a.iter()
        .zip(b.iter())
        .map(|(x, y)| (x - y).powi(2)) // Calculate squared difference
        .sum::<f64>()                    // Sum squared differences
        .sqrt()                           // Take the square root to get the distance
}

#[cfg(test)]
mod tests {
    use super::*;
    use csv::Reader;

    /// Test case for Euclidean distance when vectors are identical.
    /// Expected output is 0 because the distance between identical vectors is 0.
    #[test]
    fn test_euclidean_distance_zero() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        let dist = euclidean_distance(a, b);
        assert!((dist - 0.0).abs() < 1e-6); // The distance should be 0
    }

    /// Test case for Euclidean distance with known input.
    /// The expected distance between vectors [0, 0] and [3, 4] is 5.
    #[test]
    fn test_euclidean_distance_known() {
        let a = vec![0.0, 0.0];
        let b = vec![3.0, 4.0];
        let dist = euclidean_distance(a, b);
        assert!((dist - 5.0).abs() < 1e-6); // The distance should be 5
    }

    /// Test case for Euclidean distance with a small difference in values.
    /// The expected distance between vectors [1.0, 2.0, 3.0] and [1.0, 2.1, 3.0] is 0.1.
    #[test]
    fn test_euclidean_distance_small_diff() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.1, 3.0];
        let dist = euclidean_distance(a, b);
        assert!((dist - 0.1).abs() < 1e-6); // The distance should be around 0.1
    }

    /// Test case to ensure the CSV file is not empty.
    /// This checks if the dataset file can be read correctly and contains data.
    #[test]
    fn test_csv_not_empty() {
        let mut rdr = Reader::from_path("ObesityDataSet_raw_and_data_sinthetic.csv")
            .expect("Failed to open CSV file");
        let records: Vec<_> = rdr.records().collect::<Result<_, _>>()
            .expect("Failed to read records from CSV");
        assert!(!records.is_empty(), "CSV file should not be empty");
    }
}