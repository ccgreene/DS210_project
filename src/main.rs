use std::error::Error;

mod graph;
mod pagerank;
mod bfs;


fn main() -> Result<(), Box<dyn Error>> {
    let path = "git_edges.csv";
    let adjacency_list = graph::make_graph(path)?; //creating directed graph <HashMap<i32, Vec<i32>>
    
    //get pagerank scores
    let num_vertices = 37700; // number of nodes in data set
    
    //getting best number of steps
    let max_walks = 15000;
    let tolerance = 1e-2;
    let walk_length = 80;
    
    println!("");
    println!("The following is the most_accurate_walk_count iterating...");
    let num_walks = pagerank::most_accurate_walk_count(&adjacency_list, num_vertices, max_walks, tolerance, walk_length);
    println!("the ideal number of walks is {}", num_walks);
    
    //printing 5 pageranks
    println!("");
    let pagerank_scores = pagerank::pagerank(&adjacency_list, num_vertices, num_walks, walk_length);

    //user and friend
    //given 25 sample nodes, how connected are they using the friends module? 
    println!("");
    let mut pagerank_vec: Vec<_> = pagerank_scores.iter().collect();
    let user_vec = pagerank_vec.iter().take(25); //these are 
    
    
    
       
    Ok(())
}

//prompt for user1, user2

    //let walk_count = pagerank::most_accurate_walk_count(&pagerank_scores, num_vertices, max_walks, tolerance);
    //let (best_num_walks, best_pagerank) = pagerank::walk_convergence(&adjacency_list, num_vertices, step);
//    let mut pagerank_vec: Vec<_> = best_pagerank.iter().collect();

//     println!("Top 5 vertices by Pagerank:");
//     for (vertex, score) in pagerank_vec.iter().take(5) {
//         println!("Vertex: {}, Pagerank: {}", vertex, score);
//     }
    
//     println!("the best number of walks is {}", best_num_walks);


    //let bfs_order = bfs::bfs(1, &adjacency_list);

