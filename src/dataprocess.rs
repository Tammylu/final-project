use std::fs::File;
use std::io::{self, BufRead, BufReader};
use petgraph::graph::{DiGraph, NodeIndex};
use rand::{thread_rng, seq::SliceRandom};
use std::collections::HashMap; 

// Read edges from a file and return them as a vector of tuples
pub fn read_file(path: &str) -> Vec<(usize, usize)> {
    let mut result: Vec<(usize, usize)> = Vec::new();
    let file = File::open(path).expect("Could not open file");
    let buf_reader = BufReader::new(file).lines();
    for (idx, line) in buf_reader.enumerate() {
        let line_str = line.expect("Error reading line from file");
        let v: Vec<&str> = line_str.trim().split_whitespace().collect();
        let x = v[0].parse::<usize>().expect("Failed to parse 'from' node identifier");
        let y = v[1].parse::<usize>().expect("Failed to parse 'to' node identifier");
        result.push((x, y));
    }
    println!("Read {} lines of edge data.", result.len());
    result
}

// Function to construct a directed graph (DiGraph) from a list of edges and a given number of nodes
pub fn construct_graph(edges: Vec<(usize, usize)>, num_nodes: usize) -> DiGraph<(), ()> {
    let mut graph = DiGraph::<(), ()>::with_capacity(num_nodes, edges.len());
    let mut node_indices: HashMap<usize, NodeIndex> = HashMap::new();
    
    // Iterate over each edge defined by a pair of node identifiers
    for (from_node, to_node) in edges {
        let from_index = *node_indices.entry(from_node).or_insert_with(|| graph.add_node(()));
        let to_index = *node_indices.entry(to_node).or_insert_with(|| graph.add_node(()));
        graph.add_edge(from_index, to_index, ());
    }

    graph
}

//Function to randomly shuffle and sample a subset of edges
pub fn shuffle_and_sample(edges: &Vec<(usize, usize)>, sample_size: usize) -> Vec<(usize, usize)> {
    let mut rng = thread_rng();
    let mut shuffled_edges = edges.clone();
    shuffled_edges.shuffle(&mut rng);
    shuffled_edges.into_iter().take(sample_size).collect()
}

//Function to create an adjacency list from a list of edges
pub fn create_adjacency_list(edges: &[(usize, usize)]) -> HashMap<usize, Vec<usize>> {
    let mut adjacency_list: HashMap<usize, Vec<usize>> = HashMap::new();
    for &(from, to) in edges {
        adjacency_list.entry(from).or_insert_with(Vec::new).push(to);
    }
    adjacency_list
}