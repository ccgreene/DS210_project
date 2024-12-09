use std::collections::HashMap;
use rand::Rng;


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
        for _ in 0..num_walks {
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



pub fn most_accurate_walk_count(adjacency_list: &HashMap<i32, Vec<i32>>, num_vertices: i32, max_walks: i32, tolerance: f64, walk_length: i32) -> i32 {
    let initial_num_walks = 1; 
    let initial_walk_length = 1; 
    
    let mut prev_pagerank = pagerank(adjacency_list, num_vertices, initial_num_walks, initial_walk_length); // Initial PageRank
    let mut iteration = 0;
    let step = 10;

    for num_walks in (275..=max_walks).step_by(step) { // Start from 275 to reduce runtime
        let current_pagerank = pagerank(adjacency_list, num_vertices, num_walks, walk_length); // Current pagerank

        // compute the difference between previous and current pagerank
        let diff = current_pagerank
            .iter()
            .map(|(vertex, &value)| {
                let previous_value = prev_pagerank.get(vertex).unwrap_or(&0.0);
                (value - previous_value).powi(2)
            })
            .sum::<f64>()
            .sqrt();
        
        println!("iteration: {}, number of walks: {}, diff: {:.6}", iteration, num_walks, diff);

        if diff < tolerance {
            return num_walks;
        }

        prev_pagerank = current_pagerank; //update prev pagerank
        iteration += 1;
    }

    max_walks // return max_walks if no convergence
}


#[warn(dead_code)] //this is a helper fn for the tests
fn add_edge(adjacency_list: &mut HashMap<i32, Vec<i32>>, start: i32, end: i32) { //adding edge to hash
    adjacency_list.entry(start).or_insert_with(Vec::new).push(end); //add end
}

#[cfg(test)]

#[test]
fn test_random_walk() {
    let mut adjacency_list = HashMap::new(); //creating very simple adj list where every walk should go to 2
    add_edge(&mut adjacency_list, 0, 1);
    add_edge(&mut adjacency_list, 0, 2);
    add_edge(&mut adjacency_list, 1, 2);
    add_edge(&mut adjacency_list, 2, 0);

    let start_vertex = 0;
    let walk_length = 5;
    let num_vertices = 3;

    let result = random_walk(start_vertex, &adjacency_list, walk_length, num_vertices);
    assert!(result >= 0 && result < num_vertices);
}

#[test]
fn test_pagerank() {
    let mut adjacency_list = HashMap::new();
    add_edge(&mut adjacency_list, 0, 1);
    add_edge(&mut adjacency_list, 0, 2);
    add_edge(&mut adjacency_list, 1, 2);
    add_edge(&mut adjacency_list, 2, 0);

    let num_vertices = 3;
    let num_walks = 1000;
    let walk_length = 10;

    let pagerank = pagerank(&adjacency_list, num_vertices, num_walks, walk_length);

    assert_eq!(pagerank.len() as i32, num_vertices); //all verticies have a pagerank and it is included

    let total_pr: f64 = pagerank.values().sum(); //pageranks add up ot around 1
    println!("{}", total_pr);
    println!("{:?}", pagerank);
    assert!((total_pr - 1.0).abs() < 1e-5); //total_pr shold be around 1
}

#[test]
    fn test_most_accurate_walk_count() {
        let mut adjacency_list = HashMap::new();
        adjacency_list.insert(0, vec![1, 2]);
        adjacency_list.insert(1, vec![2]);
        adjacency_list.insert(2, vec![0, 3]);
        adjacency_list.insert(3, vec![3]); // instance of no friends or self loop

        let num_vertices = 4; 
        let max_walks = 5000; 
        let tolerance = 1e-2; 
        let walk_length = 10; 

        let num_walks = most_accurate_walk_count(&adjacency_list, num_vertices, max_walks, tolerance, walk_length);

        assert!(num_walks <= max_walks, "Accurate walks exceed max_walks"); //verfiy that it returns a valid number less than the max
        assert!(num_walks >= 275, "Accurate walks less than the initial walk count");

        println!("Most accurate walk count: {}", accurate_walks);
    }

