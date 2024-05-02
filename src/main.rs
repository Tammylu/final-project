use std::io::{self, BufRead, BufReader};
use std::collections::{HashMap, VecDeque};

mod statistic;
use crate::statistic::compute_and_display_statistics;

mod bfs;
use crate::bfs::bfs;

mod dataprocess;
use crate::dataprocess::{read_file,construct_graph,shuffle_and_sample,create_adjacency_list};

pub fn main() {
    // Read edges from a file and sample a subset for processing
    let edges = read_file("roadNet-PA.txt");
    let sample_size = 1000000; 
    let sampled_edges = shuffle_and_sample(&edges, sample_size);
    // Print sampled edges to verify correct sampling
    println!("Sampled Edges:");
    for (from, to) in &sampled_edges {
        println!("From: {}, To: {}", from, to);
    }
    // Create an adjacency list from the sampled edges
    let adjacency_list = create_adjacency_list(&sampled_edges);
    // Dictionary to hold distances computed by BFS for each node
    let mut all_distances = HashMap::new();

     // Compute BFS for each node in the adjacency list and collect distances
    for &start in adjacency_list.keys() {
        let distances = bfs(&adjacency_list, start);
        all_distances.insert(start, distances);
    }

    // Compute and display statistics for the nodes in the graph
    compute_and_display_statistics(&all_distances);
}

#[test]
pub fn test_bfs() {
    // Create a small graph as adjacency list
    let mut adjacency_list = HashMap::new();
    adjacency_list.insert(0, vec![1, 2]); 
    adjacency_list.insert(1, vec![2]);    
    adjacency_list.insert(2, vec![0, 3]); 
    adjacency_list.insert(3, vec![]);     

    // Run BFS starting from node 0
    let distances = bfs(&adjacency_list, 1);

    // Check the distance between node 1 and node 2
    assert_eq!(distances.get(&2), Some(&1), "Distance between node 1 and 2 should be 1");
}

#[test]
pub fn test_distance_from_node_to_itself() {
// Create a small graph as adjacency list
    let mut adjacency_list = HashMap::new();
    adjacency_list.insert(0, vec![1, 2]); // Node 0 connects to Node 1 and 2
    adjacency_list.insert(1, vec![2]);    // Node 1 connects to Node 2
    adjacency_list.insert(2, vec![0, 3]); // Node 2 connects to Node 0 and 3
    adjacency_list.insert(3, vec![]);     // Node 3 has no connections
    
    // Run BFS starting from node 0
    let distances = bfs(&adjacency_list, 0);
    
    // Check the distance from node 0 to itself
    assert_eq!(distances.get(&0), Some(&0), "Distance from node 0 to itself should be 0");
}

#[test]
pub fn test_bfs_no_direct_connections() {
    let mut adjacency_list = HashMap::new();
    adjacency_list.insert(0, vec![1, 2]);
    adjacency_list.insert(1, vec![2]);
    adjacency_list.insert(2, vec![0, 3]);
    adjacency_list.insert(3, vec![]); // Node 3 has no direct connections

    // Run BFS starting from node 3
    let distances = bfs(&adjacency_list, 3);

    // Check that the distances hashmap for node 3 contains only itself
    assert_eq!(distances.len(), 1, "Node 3 should only have a distance record to itself");
    assert_eq!(distances.get(&3), Some(&0), "The distance from node 3 to itself should be 0");
}



