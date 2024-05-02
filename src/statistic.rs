use std::collections::HashMap; 

// Function to compute and display statistical data about node distances
pub fn compute_and_display_statistics(all_distances: &HashMap<usize, HashMap<usize, usize>>) {
    let mut averages = Vec::new();
    let mut maximums = Vec::new();
    
    // Iterate over all nodes and their corresponding distances hash map
    for (&node, distances) in all_distances {
        let sum: usize = distances.values().sum();
        let count = distances.values().len();
        let avg = sum as f64 / count as f64;
        let max = *distances.values().max().unwrap_or(&0);
    
    // Store the computed average and maximum for the current node
        averages.push((node, avg));
        maximums.push((node, max));
        // Print the node's average and maximum distances
        println!("Node {}: Avg = {:.2}, Max = {}", node, avg, max);
    }

    // Sort and display results
    averages.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    maximums.sort_by(|a, b| a.1.cmp(&b.1));

    println!("Five smallest averages:");
    for (node, avg) in averages.iter().take(5) {
        println!("Node {}: {:.2}", node, avg);
    }

    println!("Five biggest averages:");
    for (node, avg) in averages.iter().rev().take(5) {
        println!("Node {}: {:.2}", node, avg);
    }

    println!("Five biggest maximums:");
    for (node, max) in maximums.iter().rev().take(5) {
        println!("Node {}: {}", node, max);
    }

    println!("Five Smallest maximum:");
    for (node, min) in maximums.iter().rev().take(5) {
        println!("Node {}: {}", node, min);
    }
    
}
