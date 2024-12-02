use std::error::Error;

mod graph;
mod pagerank;
mod bfs;

fn main() -> Result<(), Box<dyn Error>> {
    let path = "git_edges.csv";
    let adjacency_list = graph::make_graph(path)?; //creating directed graph
    
    //get pagerank scores
    let num_vertex = 10000;
    let pagerank_scores = pagerank::pagerank(&adjacency_list, num_vertex);
    let pagerank_vec: Vec<_> = pagerank_scores.iter().collect();
    
    //print first 5 pageranks
    //for (vertex, score) in pagerank_vec.iter().take(5) {
      //  println!("Vertex: {}, PageRank: {}", vertex, score);
    //}
    
    let bfs_order = bfs::bfs(1, &adjacency_list);
    
    
    //prompt for user1, user2
    
    Ok(())
}





//Fn make_directed_graph () -> give directed graph of all developers as nodes

//pagerank

//Fn bfs(maybe a tree) -> diameter, path existence between nodes,  Subtree Sizes and Connectivity, Locating the Closest Common Ancestor (Maybe, seems cool), Distance-Based Connectivity



//Fn friend_of_friend
//Fn how_simialr_ are_my_friends_to_My_friends_friends
    //Overlap between nodes
//Fn Decide how to measure similarity 
//Fn find social circles (would these be subtrees with the main tree? What directed graph type should I use) Could I use graph clustering if I only have nodes and edges? Would I be doing that? Is what I’m doing graph clustering? Do I need more data to graph cluster?
//Fn find social circles
//Fn compare social circle
//Fn likelihood of someone in one social circle being connected to someone in a dif social circle
