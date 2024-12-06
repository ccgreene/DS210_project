use std::collections::HashMap;
use rand::Rng;
use ndarray::Array1;


fn random_walk(start_vertex: i32, adjacency_list: &HashMap<i32, Vec<i32>>, walk_length: i32, num_vertices: i32) -> i32 {
    let mut current_vertex = start_vertex;
    let mut rng = rand::thread_rng(); // Random number generator

    for _ in 0..walk_length {
        if let Some(neighbors) = adjacency_list.get(&current_vertex) {
            if !neighbors.is_empty() && rng.gen_bool(0.85) { //85% chance to follow a random edge
                current_vertex = neighbors[rng.gen_range(0..neighbors.len())];
            } else {
                current_vertex = rng.gen_range(0..num_vertices); //15% chance teleport to a random vertex
            }
            
        } else {
            current_vertex = rng.gen_range(0..num_vertices); //if no outgoing edges teleport to a random vertex
        }
    }
    current_vertex
}


pub fn pagerank(adjacency_list: &HashMap<i32, Vec<i32>>, num_vertices: i32, num_walks: i32, walk_length: i32) -> HashMap<i32, f64> {
    let mut pagerank_counts = HashMap::new();

    for vertex in 0..num_vertices {
        pagerank_counts.insert(vertex, 0); //assign every node an edge that is 0 if they have no edges
    }

    for vertex in 0..num_vertices {
        for _ in 0..walk_length {
            let end_vertex = random_walk(vertex, adjacency_list, walk_length, num_vertices); //assign random walk value to vertex and add it
            *pagerank_counts.entry(end_vertex).or_insert(0) += 1; //add to count
        }
    }

    let total_walks = (num_walks * num_vertices) as f64; //noramlize values
    pagerank_counts
        .into_iter()
        .map(|(vertex, count)| (vertex, count as f64 / total_walks))
        .collect()
}

//takes a pagerank adj list and a step, and then returns the most accurate number of walks
fn has_converged(pr_current: &Array1<f64>, pr_previous: &Array1<f64>, tolerance: f64) -> bool { //finding the dif between two pageranks of prev and current 
    let diff = pr_current - pr_previous;
    let euc = diff.mapv(|x| x.powi(2)).sum().sqrt(); //finds euc
    euc < tolerance
}


pub fn most_accurate_walk_count(adjacency_list: &HashMap<i32, Vec<i32>>, num_vertices: i32, max_walks: i32, tolerance: f64, walk_length: i32) -> i32 {
    let initial_num_walks = 1;
    let initial_walk_length = 1;
    
    let mut prev_pagerank = pagerank(adjacency_list, num_vertices, initial_num_walks, initial_walk_length); // HashMap<i32, f64> 
    let mut iteration = 0;
    let step = 10;

    for num_walks in (300..=max_walks).step_by(step) { //starting at 300 to spare you the run time
        let current_pagerank = pagerank(adjacency_list, num_vertices, num_walks, walk_length); //HashMap<i32, f64> 

        // Compare with previous to check for convergence
        let pagerank_current = Array1::from(current_pagerank.values().cloned().collect::<Vec<f64>>());
        let pagerank_previous = Array1::from(prev_pagerank.values().cloned().collect::<Vec<f64>>());

        if has_converged(&pagerank_current, &pagerank_previous, tolerance) {
            return num_walks;
        }

        prev_pagerank = current_pagerank.clone(); //update previous Pr
        
        iteration += 1;
        println!("iteration: {}, number of walks: {}", iteration, num_walks);
    }

    max_walks //If no convergence return the maximum walks used
}


pub fn add_edge(adjacency_list: &mut HashMap<i32, Vec<i32>>, start: i32, end: i32) { //adding edge to hash
    adjacency_list.entry(start).or_insert_with(Vec::new).push(end); //add end
}


#[cfg(test)]

#[test]

fn test_termination_counts() {
    let mut adj_list_test = HashMap::new(); //make hash with edges manually
    add_edge(&mut adj_list_test, 0, 1);
    add_edge(&mut adj_list_test, 0, 2);
    add_edge(&mut adj_list_test, 1, 3);
    add_edge(&mut adj_list_test, 2, 3);


    let num_vertices = 4;
    let pagerank_counts = pagerank(&adj_list_test, num_vertices); //find pagerank

    //vertex 3 has a higher pagerank score beavuae its the most likely end point
    assert!(pagerank_counts[&3] > pagerank_counts[&0]);
    assert!(pagerank_counts[&3] > pagerank_counts[&1]);
    assert!(pagerank_counts[&3] > pagerank_counts[&2]);

    let total_walks = 80 * num_vertices;  // check that the counts sum to the total number of walks
    let counted_walks: i32 = pagerank_counts.values().map(|&count| (count * total_walks as f64) as i32).sum(); //normalize as we did above
    assert_eq!(counted_walks, total_walks); //make sure =
}

