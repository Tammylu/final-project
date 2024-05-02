use std::collections::VecDeque;
use std::collections::HashMap; 

// Function to perform Breadth-First Search (BFS) and compute distances
pub fn bfs(adjacency_list: &HashMap<usize, Vec<usize>>, start: usize) -> HashMap<usize, usize> {
    // Initialize a queue to manage the nodes as they are visited
   let mut queue = VecDeque::new();
   // Initialize a hashmap to store the distance from the start node to each node
   let mut distances = HashMap::new();
   
   // Start the BFS with the initial node
   queue.push_back(start);
   // The distance from the start node to itself is always 0
   distances.insert(start, 0);
   
   // Continue the search until there are no more nodes to process
   while let Some(current) = queue.pop_front() {
       let current_distance = distances[&current]; 
       // Check if the current node has any neighbors
       if let Some(neighbors) = adjacency_list.get(&current) {
           for &neighbor in neighbors {
               // If the neighbor hasn't been visited yet (i.e., not in the distances map)
               if !distances.contains_key(&neighbor) {
                   queue.push_back(neighbor);
                   distances.insert(neighbor, current_distance + 1);
                   println!("Discovered node: {} with distance: {}", neighbor, current_distance + 1);

               }
           }
       }
   }
   distances
}