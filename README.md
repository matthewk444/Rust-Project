# Rust-Project 
Obesity Graph Analysis Projectv

A. Project Overview

Goal
To build a graph-based representation of individuals from an obesity dataset, enabling the analysis of similarities and distances between them using Euclidean distance in a normalized feature space. The primary question is: How can graph structures help visualize and understand relationships between individuals’ health profiles and obesity levels?

Dataset
	•	Source: UCI Machine Learning Repository - Obesity Levels Dataset
	•	Size: 2111 entries, 17 features
	•	Format: CSV (included locally as ObesityDataSet_raw_and_data_sinthetic.csv)
https://archive.ics.uci.edu/dataset/544/estimation+of+obesity+levels+based+on+eating+habits+and+physical+condition

⸻

B. Data Processing

Loading
Data is loaded using the csv crate. Each row is parsed into a StringRecord, and selected columns are extracted depending on the user’s choice (all features, eating only, or physical activity only).

Transformations
	•	Non-numeric columns are one-hot encoded.
	•	All features are normalized to the range [0, 1].
	•	The final result is a 2D matrix (ndarray::Array2<f64>) used as graph node features.

⸻

C. Code Structure

Modules

data.rs

Handles loading and preprocessing of the CSV dataset.
Key struct: Dataset – Holds the normalized data matrix, original records, and headers.

graph.rs

Constructs a graph and analyzes its properties.
Key functions:
	•	build_graph(data: &Array2<f64>): Builds an undirected graph with edges between similar individuals.
	•	analyze_graph(graph, dataset): Computes Dijkstra distances and class distributions.
	•	find_furthest_pair(...): Finds the most distant nodes to estimate graph diameter.

util.rs

Contains utility functions and tests.
Key function:
	•	euclidean_distance(a: Vec<f64>, b: Vec<f64>) -> f64: Computes the Euclidean distance between two feature vectors.

Main Workflow
	1.	User chooses a feature subset (all/eating/physical).
	2.	Data is loaded and preprocessed in data::load_data.
	3.	A graph is built where each node is a person, and edges represent closeness.
	4.	Graph analysis is performed:
	•	Number of nodes/edges
	•	Class distribution of obesity levels
	•	Dijkstra distances from a root node
	•	Farthest connected node pair identified

⸻

D. Tests

Command

cargo test

Output

running 4 tests
test util::tests::test_csv_not_empty ... ok
test util::tests::test_euclidean_distance_zero ... ok
test util::tests::test_euclidean_distance_known ... ok
test util::tests::test_euclidean_distance_small_diff ... ok

test result: ok. 4 passed; 0 failed; 0 ignored

Test Coverage
	•	test_euclidean_distance_zero: Checks that distance between identical vectors is zero.
	•	test_euclidean_distance_known: Verifies the known (3,4,5) triangle returns 5.
	•	test_euclidean_distance_small_diff: Confirms a small change yields a correct distance.
	•	test_csv_not_empty: Ensures dataset loads and contains records.

⸻

E. Results

Sample Output
Choose graph type: (1) All features, (2) Eating only, (3) Physical only
3
(Theres more) 
Node 583 (distance 0.56): Obesity level = Insufficient_Weight
Node 1980 (distance 0.60): Obesity level = Obesity_Type_III
Node 1882 (distance 0.65): Obesity level = Obesity_Type_III
Node 260 (distance 0.39): Obesity level = Overweight_Level_I
Node 1788 (distance 0.90): Obesity level = Obesity_Type_II
Node 1633 (distance 0.64): Obesity level = Obesity_Type_II
Node 489 (distance 0.71): Obesity level = Normal_Weight
Node 1457 (distance 0.37): Obesity level = Obesity_Type_I
Node 1080 (distance 0.83): Obesity level = Overweight_Level_II
Node 2058 (distance 0.63): Obesity level = Obesity_Type_III
Node 407 (distance 0.64): Obesity level = Overweight_Level_I
Node 1483 (distance 0.23): Obesity level = Obesity_Type_I
Node 1403 (distance 0.44): Obesity level = Obesity_Type_I
Node 2017 (distance 0.59): Obesity level = Obesity_Type_III
Node 2010 (distance 0.60): Obesity level = Obesity_Type_III
Node 557 (distance 0.41): Obesity level = Insufficient_Weight
Node 361 (distance 0.96): Obesity level = Obesity_Type_I
Node 888 (distance 0.29): Obesity level = Overweight_Level_I
Node 23 (distance 0.98): Obesity level = Obesity_Type_I
Node 779 (distance 0.55): Obesity level = Overweight_Level_I
Node 1852 (distance 0.97): Obesity level = Obesity_Type_III
Node 1356 (distance 0.24): Obesity level = Obesity_Type_I
Node 692 (distance 0.64): Obesity level = Insufficient_Weight
Node 196 (distance 0.42): Obesity level = Normal_Weight
Node 738 (distance 0.68): Obesity level = Insufficient_Weight
Node 1261 (distance 0.76): Obesity level = Obesity_Type_I
Node 158 (distance 0.52): Obesity level = Overweight_Level_I
Node 1302 (distance 0.82): Obesity level = Obesity_Type_I
Node 1679 (distance 0.51): Obesity level = Obesity_Type_II
Node 87 (distance 0.41): Obesity level = Overweight_Level_I
Node 1133 (distance 0.56): Obesity level = Overweight_Level_II
Node 1562 (distance 0.54): Obesity level = Obesity_Type_II
Node 414 (distance 0.96): Obesity level = Obesity_Type_I
Node 1953 (distance 0.67): Obesity level = Obesity_Type_III
Node 806 (distance 0.75): Obesity level = Overweight_Level_I
Node 317 (distance 0.80): Obesity level = Normal_Weight
Node 1787 (distance 0.90): Obesity level = Obesity_Type_II
Node 1234 (distance 0.66): Obesity level = Obesity_Type_I
Node 1420 (distance 0.61): Obesity level = Obesity_Type_I
Node 628 (distance 0.84): Obesity level = Insufficient_Weight
Node 967 (distance 0.55): Obesity level = Overweight_Level_I
Node 907 (distance 1.11): Obesity level = Overweight_Level_I
Node 1484 (distance 0.48): Obesity level = Obesity_Type_I
Node 564 (distance 0.89): Obesity level = Insufficient_Weight
Node 765 (distance 0.09): Obesity level = Overweight_Level_I
Node 648 (distance 0.48): Obesity level = Insufficient_Weight
Node 354 (distance 0.34): Obesity level = Normal_Weight
Node 1686 (distance 0.61): Obesity level = Obesity_Type_II
Node 1073 (distance 0.66): Obesity level = Overweight_Level_II
Node 1521 (distance 0.54): Obesity level = Obesity_Type_II
Node 478 (distance 0.36): Obesity level = Normal_Weight
Node 1845 (distance 0.88): Obesity level = Obesity_Type_III
Node 1626 (distance 0.73): Obesity level = Obesity_Type_II
Node 1889 (distance 0.61): Obesity level = Obesity_Type_III
Node 519 (distance 1.05): Obesity level = Insufficient_Weight
Node 1016 (distance 0.67): Obesity level = Overweight_Level_II
Node 151 (distance 0.90): Obesity level = Normal_Weight
Node 1384 (distance 0.60): Obesity level = Obesity_Type_I
Node 195 (distance 0.72): Obesity level = Overweight_Level_I
Node 1750 (distance 0.49): Obesity level = Obesity_Type_II
Node 1223 (distance 0.15): Obesity level = Obesity_Type_I
Node 601 (distance 0.77): Obesity level = Insufficient_Weight
Node 1144 (distance 0.43): Obesity level = Overweight_Level_II
Node 1439 (distance 0.21): Obesity level = Obesity_Type_I
Node 2092 (distance 0.63): Obesity level = Obesity_Type_III
Node 817 (distance 0.20): Obesity level = Overweight_Level_I
Node 1942 (distance 0.62): Obesity level = Obesity_Type_III
Node 290 (distance 0.77): Obesity level = Normal_Weight
Node 712 (distance 0.71): Obesity level = Insufficient_Weight
Node 952 (distance 0.38): Obesity level = Overweight_Level_I
Node 1589 (distance 0.55): Obesity level = Obesity_Type_II
Node 1615 (distance 0.87): Obesity level = Obesity_Type_II
Node 1320 (distance 0.71): Obesity level = Obesity_Type_I
Node 451 (distance 0.37): Obesity level = Normal_Weight
Node 1373 (distance 0.96): Obesity level = Obesity_Type_I
Node 1807 (distance 0.69): Obesity level = Obesity_Type_III
Node 1170 (distance 0.54): Obesity level = Overweight_Level_II
Node 1916 (distance 0.60): Obesity level = Obesity_Type_III
Node 870 (distance 0.64): Obesity level = Overweight_Level_I
Node 324 (distance 0.08): Obesity level = Overweight_Level_I
Node 1197 (distance 0.41): Obesity level = Overweight_Level_II
Node 1984 (distance 0.78): Obesity level = Obesity_Type_III
Node 1724 (distance 0.87): Obesity level = Obesity_Type_II
Node 2043 (distance 0.63): Obesity level = Obesity_Type_III
Node 667 (distance 0.90): Obesity level = Insufficient_Weight
Node 425 (distance 0.67): Obesity level = Normal_Weight
Node 1062 (distance 0.46): Obesity level = Overweight_Level_II
Node 843 (distance 0.73): Obesity level = Overweight_Level_I
Node 98 (distance 1.02): Obesity level = Insufficient_Weight
Node 1339 (distance 0.48): Obesity level = Obesity_Type_I
Node 48 (distance 0.65): Obesity level = Normal_Weight
Node 233 (distance 0.98): Obesity level = Overweight_Level_II
Node 978 (distance 0.54): Obesity level = Overweight_Level_II
Node 2065 (distance 0.92): Obesity level = Obesity_Type_III
Node 1450 (distance 0.69): Obesity level = Obesity_Type_I
Node 921 (distance 0.80): Obesity level = Overweight_Level_I
Node 820 (distance 0.85): Obesity level = Overweight_Level_I
Node 392 (distance 0.90): Obesity level = Normal_Weight
Node 1757 (distance 0.79): Obesity level = Obesity_Type_II
Node 1228 (distance 0.88): Obesity level = Obesity_Type_I
Node 731 (distance 0.85): Obesity level = Insufficient_Weight
Node 456 (distance 0.97): Obesity level = Normal_Weight
Node 1838 (distance 1.00): Obesity level = Obesity_Type_III
Node 1055 (distance 0.60): Obesity level = Overweight_Level_II
Node 590 (distance 0.43): Obesity level = Insufficient_Weight
Node 1640 (distance 0.67): Obesity level = Obesity_Type_II
Node 857 (distance 0.78): Obesity level = Overweight_Level_I
Node 1875 (distance 0.91): Obesity level = Obesity_Type_III
Node 1183 (distance 0.86): Obesity level = Overweight_Level_II
Node 347 (distance 0.54): Obesity level = Overweight_Level_II
Node 61 (distance 1.00): Obesity level = Normal_Weight
Node 985 (distance 0.20): Obesity level = Overweight_Level_II
Node 1576 (distance 0.49): Obesity level = Obesity_Type_II
Node 691 (distance 0.67): Obesity level = Insufficient_Weight
Node 1939 (distance 0.62): Obesity level = Obesity_Type_III
Node 1309 (distance 0.26): Obesity level = Obesity_Type_I
Node 1119 (distance 0.23): Obesity level = Overweight_Level_II
Node 1349 (distance 0.83): Obesity level = Obesity_Type_I
Node 101 (distance 0.15): Obesity level = Normal_Weight
Node 283 (distance 0.68): Obesity level = Normal_Weight
Node 1130 (distance 0.14): Obesity level = Overweight_Level_II
Node 509 (distance 0.85): Obesity level = Insufficient_Weight
Node 910 (distance 0.89): Obesity level = Overweight_Level_I
Node 1036 (distance 0.63): Obesity level = Overweight_Level_II
Node 2054 (distance 0.94): Obesity level = Obesity_Type_III
Node 411 (distance 0.50): Obesity level = Overweight_Level_I
Node 1413 (distance 0.15): Obesity level = Obesity_Type_I
Node 1012 (distance 0.19): Obesity level = Overweight_Level_II
Node 1768 (distance 0.75): Obesity level = Obesity_Type_II
Node 1920 (distance 0.59): Obesity level = Obesity_Type_III
Node 112 (distance 0.49): Obesity level = Normal_Weight
Node 264 (distance 0.81): Obesity level = Normal_Weight
Node 1595 (distance 0.89): Obesity level = Obesity_Type_II
Node 1265 (distance 0.33): Obesity level = Obesity_Type_I
Node 627 (distance 1.20): Obesity level = Insufficient_Weight
Node 775 (distance 1.00): Obesity level = Overweight_Level_I
Node 537 (distance 0.68): Obesity level = Insufficient_Weight
Node 34 (distance 0.76): Obesity level = Normal_Weight
Node 255 (distance 1.29): Obesity level = Overweight_Level_I
Node 641 (distance 1.01): Obesity level = Insufficient_Weight
Node 1282 (distance 0.85): Obesity level = Obesity_Type_I
Node 1399 (distance 0.73): Obesity level = Obesity_Type_I
Node 1164 (distance 0.71): Obesity level = Overweight_Level_II
Node 2006 (distance 0.63): Obesity level = Obesity_Type_III
Node 758 (distance 0.49): Obesity level = Overweight_Level_I
Node 138 (distance 1.19): Obesity level = Normal_Weight
Node 1503 (distance 0.64): Obesity level = Obesity_Type_I
Node 1514 (distance 0.75): Obesity level = Obesity_Type_II
Node 1629 (distance 0.88): Obesity level = Obesity_Type_II
Node 526 (distance 1.01): Obesity level = Insufficient_Weight
Node 381 (distance 0.93): Obesity level = Normal_Weight
Node 3 (distance 0.94): Obesity level = Overweight_Level_I
Node 1011 (distance 0.60): Obesity level = Overweight_Level_II
Node 2037 (distance 0.60): Obesity level = Obesity_Type_III
Node 1029 (distance 0.81): Obesity level = Overweight_Level_II
Node 1902 (distance 0.61): Obesity level = Obesity_Type_III
Node 884 (distance 1.16): Obesity level = Overweight_Level_I
Node 1137 (distance 0.56): Obesity level = Overweight_Level_II
Node 1973 (distance 1.13): Obesity level = Obesity_Type_III
Node 608 (distance 0.36): Obesity level = Insufficient_Weight
Node 482 (distance 0.82): Obesity level = Normal_Weight
Node 2091 (distance 0.63): Obesity level = Obesity_Type_III
Node 903 (distance 0.34): Obesity level = Overweight_Level_I
Node 67 (distance 0.24): Obesity level = Obesity_Type_I
Node 1335 (distance 0.70): Obesity level = Obesity_Type_I
Node 1424 (distance 1.29): Obesity level = Obesity_Type_I
Node 257 (distance 0.67): Obesity level = Obesity_Type_I
Node 1783 (distance 0.79): Obesity level = Obesity_Type_II
Node 1558 (distance 0.56): Obesity level = Obesity_Type_II
Node 240 (distance 0.80): Obesity level = Normal_Weight
Node 1792 (distance 0.75): Obesity level = Obesity_Type_II
Node 1488 (distance 0.24): Obesity level = Obesity_Type_I
Node 30 (distance 0.80): Obesity level = Overweight_Level_I
Node 131 (distance 0.34): Obesity level = Normal_Weight
Node 1723 (distance 0.86): Obesity level = Obesity_Type_II
Node 1380 (distance 0.85): Obesity level = Obesity_Type_I
Node 1909 (distance 0.94): Obesity level = Obesity_Type_III
Node 850 (distance 0.52): Obesity level = Overweight_Level_I
Node 668 (distance 0.90): Obesity level = Insufficient_Weight
Node 544 (distance 1.07): Obesity level = Insufficient_Weight
Node 786 (distance 0.70): Obesity level = Overweight_Level_I
Node 1254 (distance 0.45): Obesity level = Obesity_Type_I
Node 1666 (distance 0.75): Obesity level = Obesity_Type_II
Node 1849 (distance 0.60): Obesity level = Obesity_Type_III
Node 1622 (distance 0.94): Obesity level = Obesity_Type_II
Node 1856 (distance 0.63): Obesity level = Obesity_Type_III
Node 374 (distance 0.82): Obesity level = Normal_Weight
Node 418 (distance 0.88): Obesity level = Obesity_Type_I
Node 1659 (distance 0.87): Obesity level = Obesity_Type_II
Node 1443 (distance 0.69): Obesity level = Obesity_Type_I
Node 310 (distance 0.85): Obesity level = Insufficient_Weight
Node 1227 (distance 0.64): Obesity level = Obesity_Type_I
Node 597 (distance 0.20): Obesity level = Insufficient_Weight
Node 2072 (distance 0.89): Obesity level = Obesity_Type_III
Node 176 (distance 0.97): Obesity level = Normal_Weight
Node 1730 (distance 0.87): Obesity level = Obesity_Type_II
Node 94 (distance 0.90): Obesity level = Normal_Weight
Node 732 (distance 0.88): Obesity level = Insufficient_Weight
Node 1316 (distance 0.65): Obesity level = Obesity_Type_I
Node 948 (distance 0.11): Obesity level = Overweight_Level_I
Node 687 (distance 0.65): Obesity level = Insufficient_Weight
Node 813 (distance 0.60): Obesity level = Overweight_Level_I
Node 1190 (distance 0.50): Obesity level = Overweight_Level_II
Node 445 (distance 1.13): Obesity level = Normal_Weight
Node 1100 (distance 0.56): Obesity level = Overweight_Level_II
Node 1811 (distance 0.96): Obesity level = Obesity_Type_III
Node 1477 (distance 0.77): Obesity level = Obesity_Type_I
Node 1693 (distance 0.73): Obesity level = Obesity_Type_II
Node 1704 (distance 0.80): Obesity level = Obesity_Type_II
Node 229 (distance 0.71): Obesity level = Obesity_Type_II
Node 974 (distance 0.64): Obesity level = Overweight_Level_I
Node 563 (distance 1.10): Obesity level = Insufficient_Weight
Node 471 (distance 0.36): Obesity level = Obesity_Type_I
Node 839 (distance 0.89): Obesity level = Overweight_Level_I
Node 698 (distance 1.02): Obesity level = Insufficient_Weight
Node 1066 (distance 1.01): Obesity level = Overweight_Level_II
Node 1946 (distance 0.64): Obesity level = Obesity_Type_III
Node 705 (distance 0.91): Obesity level = Insufficient_Weight
Node 1569 (distance 0.69): Obesity level = Obesity_Type_II
Node 328 (distance 1.41): Obesity level = Normal_Weight
Node 1201 (distance 0.53): Obesity level = Overweight_Level_II

Class distribution: {"Obesity_Type_I": 351, "Obesity_Type_II": 297, "Normal_Weight": 287, "Obesity_Type_III": 324, "Insufficient_Weight": 272, "Overweight_Level_II": 290, "Overweight_Level_I": 290}
Max path length (graph diameter estimate): 2.30 between nodes 1 and 137

Node 1 data:
  Gender: Female
  Age: 21
  Height: 1.52
  Weight: 56
  family_history_with_overweight: yes
  FAVC: no
  FCVC: 3
  NCP: 3
  CAEC: Sometimes
  SMOKE: yes
  CH2O: 3
  SCC: yes
  FAF: 3
  TUE: 0
  CALC: Sometimes
  MTRANS: Public_Transportation
  NObeyesdad: Normal_Weight

Node 137 data:
  Gender: Male
  Age: 44
  Height: 1.6
  Weight: 80
  family_history_with_overweight: yes
  FAVC: no
  FCVC: 2
  NCP: 3
  CAEC: Sometimes
  SMOKE: yes
  CH2O: 3
  SCC: no
  FAF: 0
  TUE: 0
  CALC: no
  MTRANS: Motorbike
  NObeyesdad: Obesity_Type_I

Interpretation
	•	Most individuals are connected within a small threshold distance (dense graph).
	•	Class distribution shows the data is reasonably balanced.
	•	The graph’s diameter is small, indicating strong feature similarity across individuals.
  •	Weigths of my graph were non-negative so I utilized Djikstras Algorithm
 

 _________________________________________________________________________________________________-
 

⸻

F. Usage Instructions

Build & Run

cargo build --release
cargo run

User Interaction

Choose graph type: (1) All features, (2) Eating only, (3) Physical only

Expected Runtime
	•	Build time: ~3 seconds
	•	Run time: ~1–2 seconds 


G. Ai Usage

Utilized Chat GPT to help debug, and make a SSH key for Github 

I also used AI to help my with one-hot encoding, and helping me build the algorithm to traverse through the graph I made. 

I had a lot of errors regarding Dijkstra algorithm, and Chat GPT helped me debug and build a more effective method of code. 
