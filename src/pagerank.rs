use std::error::Error;
use std::collections::HashMap;
use rand::Rng;


// Simulating a random walk with 80 steps
pub fn random_walk(start_vertex: i32, adjacency_list: &HashMap<i32, Vec<i32>>, num_vertices: i32) -> i32 {
    let mut current_vertex = start_vertex;
    let mut rng = rand::thread_rng(); //gen random numbers

    for _ in 0..1000 { 
        if let Some(neighbors) = adjacency_list.get(&current_vertex) { //if neighbors proceed
            if rng.gen_bool(0.5) { // 50% of the time
                current_vertex = neighbors[rng.gen_range(0..neighbors.len())]; // choose a random outgoing edge
            } else {
                current_vertex = rng.gen_range(0..num_vertices); //50% probability to jump to a random vertex
            }
        } else {
            current_vertex = rng.gen_range(0..num_vertices);  // if no outgoing edges, jump to a random vertex
        }
    }
    current_vertex //return current vertex
}

// Simulate multiple random walks for each vertex
pub fn pagerank(adjacency_list: &HashMap<i32, Vec<i32>>, num_vertices: i32) -> HashMap<i32, f64> {
    let mut pagerank_hash = HashMap::new();

    //do 100 random walks for every vertex
    for vertex in 0..num_vertices { 
        for _ in 0..100 { 
            let end_vertex = random_walk(vertex, adjacency_list, num_vertices); //random walak of end vertex given by pagerank
            *pagerank_hash.entry(end_vertex).or_insert(0) += 1; //for every vertex, add to the count
        }
    }

    let total_walks = 100 * num_vertices; //normalize values
    pagerank_hash
        .into_iter()
        .map(|(vertex, count)| (vertex, count as f64 / total_walks as f64))
        .collect() //iterate trhough and produce all normalized values
}

#[cfg(test)]

#[test]
fn add_edge(adjacency_list: &mut HashMap<i32, Vec<i32>>, start: i32, end: i32) {
    adjacency_list.entry(start).or_insert_with(Vec::new).push(end);
}

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
