use std::collections::{HashMap, HashSet};

//gives common friends and similarity between two people
fn find_common_friends(adj_list: &HashMap<i32, Vec<i32>>, user1: i32, user2: i32) -> HashSet<i32> {
    if let (Some(user1_friends), Some(user2_friends)) = (adj_list.get(&user1), adj_list.get(&user2) {
        let user1_set: HashSet<_> = user1_friends.iter().cloned().collect();
        let user2_set: HashSet<_> = user2_friends.iter().cloned().collect();
        
        let common_friends: HashSet<_> = user1_set.intersection(&user2_set).cloned().collect()
        let jaccard_similarity = (common_friends.len()/(user1_set.len() + user2_set.len())) * 100
        
        println!("user1 and user2 have {} many friends in common. This is the list: {:?}", common_friends.len(), common_friends);
        println!("user1 and user2 have {} similarity", jaccard_similarity)
    } else { 
        println!("There are no common friends.") 
        }
}


 
//using bfs traversal order
fn find_social_circles(adjacency_list: &HashMap<i32, Vec<i32>>) -> Vec<Vec<i32>> {
    let mut visited = HashSet::new();
    let mut circles = Vec::new();

    for &node in adjacency_list.keys() {
        if !visited.contains(&node) {
            let social_circle = bfs(adjacency_list, node, &mut visited);
            circles.push(social_circle);
        }
    }
    circles
}

fn friend_of_friend_probability(adjacency_list: &HashMap<i32, Vec<i32>>, user: i32) -> f64 {
    let user_friends = adjacency_list.get(&user)
    let user_set: HashSet<_> = user_friends.iter().cloned().collect()
    
    if user_set.is_empty {
        return println!("User has no friends.");
    }
    
    for friend in user_set
}


// fn common_friends(adjacency_list: &HashMap<i32, Vec<i32>>, person_a: i32, person_b: i32) -> i32
// //AKA similarity of social circles
// //use bfs mod

// fn calculate_inter_group_likelihood(
//     group_a: &Vec<i32>, 
//     group_b: &Vec<i32>, 
//     adjacency_list: &HashMap<i32, Vec<i32>>
// ) -> f64 

    
    
    
    
    
    
    
//fn find_social_circles(adjacency_list: &HashMap<i32, Vec<i32>>) -> Vec<Vec<i32>> 

//Fn friend_of_friend
//Fn how_simialr_ are_my_friends_to_My_friends_friends
    //Overlap between nodes
//Fn Decide how to measure similarity 
//Fn find social circles (would these be subtrees with the main tree? What directed graph type should I use) Could I use graph clustering if I only have nodes and edges? Would I be doing that? Is what I’m doing graph clustering? Do I need more data to graph cluster?
//Fn find social circles
//Fn compare social circle
//Fn likelihood of someone in one social circle being connected to someone in a dif social circle