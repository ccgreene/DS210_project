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
    let mut visited = HashSet::new(); //Hashset for faster lookups
    let mut circle_hash = HashMap::new();

    for &node in adjacency_list.keys() {
        if !visited.contains(&node) { //if unvisited node then do bfs and find circle
            let social_circle = bfs(adjacency_list, node, &mut visited);
            //use first node of the circle as the key
            circle_hash.insert(node, social_circle);
        }
    }

    circle_hash //return the HashMap with social circles
}


#[cfg(test)]

#[test]
fn test_find_common_friends() {
    let mut adj_list: HashMap<i32, Vec<i32>> = HashMap::new();

    adj_list.insert(1, vec![2, 3]);
    adj_list.insert(2, vec![1, 3]);
    adj_list.insert(3, vec![2, 4, 1]);
    adj_list.insert(4, vec![1]);
    
    let common_friends = find_common_friends(&adj_list, 1, 2); //user 1 and 2 have only friend 3 in common
    assert_eq!(common_friends, HashSet::from([3]));
    
    let common_friends = find_common_friends(&adj_list, 1, 4); //user 1 and user 4 have no friends in common
    assert_eq!(common_friends, HashSet::new());

}

#[test]
fn test_find_social_circles() {
    let mut adj_list: HashMap<i32, Vec<i32>> = HashMap::new();
        
    // Three social circles: [1, 2, 3], [4, 5], [6]
    adj_list.insert(1, vec![2, 3]); 
    adj_list.insert(2, vec![1, 3]); 
    adj_list.insert(3, vec![1, 2]);  
    adj_list.insert(4, vec![5]);    
    adj_list.insert(5, vec![4]);   
    adj_list.insert(6, vec![]);  // 6 has no friends
        
    let circles = find_social_circles(&adj_list);
        
    assert_eq!(circles.len(), 3); // Make sure it has the correct number of circles

    for (key, circle) in &circles {
        // Check that the key corresponds to the correct circle members
        if *key == 1 {
            //circle [1, 2, 3]
            assert!(circle.contains(&1));
            assert!(circle.contains(&2));
            assert!(circle.contains(&3));
        } else if *key == 4 {
            //circle [4, 5]
            assert!(circle.contains(&4));
            assert!(circle.contains(&5));
        } else if *key == 6 {
            //circle [6]
            assert!(circle.contains(&6));
        }
    }
}