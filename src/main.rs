use std::error::Error;
use rand::Rng;

mod graph;
mod pagerank;
mod bfs_friends;



fn main() -> Result<(), Box<dyn Error>> {
    let path = "git_edges.csv";
    let adjacency_list = graph::make_graph(path)?; //creating directed graph <HashMap<i32, Vec<i32>>
    
    //get pagerank scores
    let num_vertices = 37700; // number of nodes in data set - 1
    
    //getting best number of steps
    let max_walks = 15000;
    let tolerance = 1e-2;
    let walk_length = 80;
    
    println!("\nThe following is the most_accurate_walk_count iterating...");
    let num_walks = pagerank::most_accurate_walk_count(&adjacency_list, num_vertices, max_walks, tolerance, walk_length);
    println!("the ideal number of walks is {}", num_walks);
    
    
    //printing 5 pageranks
    println!("\nThis is the pagerank of the 25 most connected verticies.");
    let pagerank_scores = pagerank::pagerank(&adjacency_list, num_vertices, num_walks, walk_length);
    let mut top_25_pagerank_vec: Vec<_> = pagerank_scores.iter().collect();
    top_25_pagerank_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap()); // Sort by PageRank score descending
    
    for (node, rank) in top_25_pagerank_vec.iter().take(25) {
        println!("Vertex: {}, PageRank: {}", node, rank);
    }

    //bfs_friends
    println!("");
    println!("Given that the data is relatively sparse with a density of 0.001, I sorted the pagerank scores by descending order");
    println!("\nThe following code will find the similiarity in friends with a user and the top 25 msot connected users");
    
    let mut rng = rand::thread_rng(); // Create a random number generator
    let random_top_25_user_index: usize = rng.gen_range(0..24);
    
    let (user, rank) = top_25_pagerank_vec[random_top_25_user_index];
    println!("The random user is {}! The vector their rank is {:?}", user, rank);
    
    for (node, _) in top_25_pagerank_vec.iter().take(25) { //could make it whole adj list
        println!("\nThe user {} has these friends in common with user {}", user, node);
        bfs_friends::find_common_friends(&adjacency_list, *user, **node);
    }
    
    //bfs friends circles
    println!("\n The social circles are:");
    let social_circles = bfs_friends::find_social_circles(&adjacency_list);

    let circles_vec: Vec<_> = social_circles.iter().collect();

    let mut under_ten = 0;
    let mut ten_to_hundred = 0;
    let mut hundred_five_hundred = 0;
    let mut five_hundred_plus = 0;

    for (_, circle_user_tuple) in circles_vec.iter().enumerate() {
        // Dereference the tuple to get the key and the user vector
        let (_circle_id, circle_users) = circle_user_tuple;

        if circle_users.len() < 10 {
            under_ten += 1;
        } else if circle_users.len() >= 10 && circle_users.len() <= 100 {
            ten_to_hundred += 1;
        } else if circle_users.len() > 100 && circle_users.len() <= 500 {
            hundred_five_hundred += 1;
        } else {
            five_hundred_plus += 1;
        }
    }

    println!("\nSocial Circles Stats:");
    println!("Circles with less than 10 users: {} ", under_ten);
    println!("Circles with 10-100 users: {}", ten_to_hundred);
    println!("Circles with 101-500 users: {}", hundred_five_hundred);
    println!("Circles with more than 500 users: {}", five_hundred_plus);
 
    Ok(())
    
}

    
    
       
