use std::collections::VecDeque; //improting vecqdeu
use std::collections::HashMap;
use std::collections::HashSet;

pub fn bfs(start_vertex: i32, adj_list: &HashMap<i32, Vec<i32>>) -> Vec<i32>  { //given tree give node and max
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new(); 
    let mut traversal_order = Vec::new(); 
    
    visited.insert(start_vertex);
    queue.push_back(start_vertex);
    
    while let Some(current_vertex) = queue.pop_front() { //while there are nodes to deque/pop
        traversal_order.push(current_vertex);
        
        if let Some(neighbors) = adj_list.get(&current_vertex) {
            for &neighbor in neighbors {
                if !visited.contains(&neighbor) {
                    visited.insert(neighbor);
                    queue.push_back(neighbor);
                }
            }
        } 
    }

    traversal_order //return
}


