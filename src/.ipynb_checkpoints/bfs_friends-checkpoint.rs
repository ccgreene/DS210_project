use std::collections::VecDeque; //improting vecqdeu
use std::collections::HashMap;
use std::collections::HashSet;

pub fn bfs(adj_list: &HashMap<i32, Vec<i32>>, start_vertex: i32, visited: &mut HashSet<i32>) -> Vec<i32>  { //given node and visited for friends fn
    let mut queue = VecDeque::new();  
    let mut traversal_order = Vec::new(); 
    
    visited.insert(start_vertex); //insert start vertex into visited
    queue.push_back(start_vertex); //push start vertex to end of queue
    
    while let Some(current_vertex) = queue.pop_front() { //while there are nodes, pop them in fron of the que 
        traversal_order.push(current_vertex); //add the nodes to the traversal order
        
        if let Some(neighbors) = adj_list.get(&current_vertex) { //if there are neighbors, get the neighbors
            for &neighbor in neighbors { //for every neighbor 
                if !visited.contains(&neighbor) { //if it hasnt been visisted
                    visited.insert(neighbor); //insert into visisted 
                    queue.push_back(neighbor); //push to end of queue
                }
            }
        } 
    }

    traversal_order //return order
}


pub fn find_common_friends(adj_list: &HashMap<i32, Vec<i32>>, user1: i32, user2: i32) -> HashSet<i32> { //test would be usr1 having all same friends as user1
    if let (Some(user1_friends), Some(user2_friends)) = (adj_list.get(&user1), adj_list.get(&user2)) {
        
        let user1_set: HashSet<_> = user1_friends.iter().cloned().collect();
        let user2_set: HashSet<_> = user2_friends.iter().cloned().collect();

        let common_friends: HashSet<_> = user1_set.intersection(&user2_set).cloned().collect(); //find common friends

        let total_friends = user1_set.len() + user2_set.len() - common_friends.len(); //calculate  similarity
        let similarity = if total_friends > 0 {
            (common_friends.len() as f64 / total_friends as f64) * 100.0
        } else {
            0.0
        };
        
        if !common_friends.is_empty() {
            println!("User {} and User {} have {} common friends: {:?}", user1, user2, common_friends.len(), common_friends);
            println!("The similarity between User {} and User {}: {:.2}%", user1, user2, similarity);
        } else { 
            println!("User {} or User {} has no friends listed in the adjacency list.", user1, user2); 
        }

        common_friends
        
    } else {
        println!("User {} or User {} has no friends listed in the adjacency list.", user1, user2);
        
        HashSet::new()
    }
}

pub fn find_social_circles(adjacency_list: &HashMap<i32, Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    let mut visited = HashSet::new(); // Use HashSet for faster lookups
    let mut iteration = 0;
    let mut circle_hash = HashMap::new();

    for &node in adjacency_list.keys() {
        if !visited.contains(&node) {
            let social_circle = bfs(adjacency_list, node, &mut visited); // Perform BFS to find the social circle
            iteration += 1;
            
            // Insert the social circle into the HashMap with the iteration number as the key
            circle_hash.insert(iteration, social_circle);
        }
    }

    circle_hash // Return the HashMap containing social circles
}
