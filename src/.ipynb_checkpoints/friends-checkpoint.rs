fn find_friends_of_friends(adjacency_list: &HashMap<i32, Vec<i32>>, user: i32) -> HashSet<i32> 

fn calculate_friend_of_friend_probability(
    adjacency_list: &HashMap<i32, Vec<i32>>, 
    user: i32
) -> f64 

fn common_friends(adjacency_list: &HashMap<i32, Vec<i32>>, person_a: i32, person_b: i32) -> i32
//AKA similarity of social circles
//use bfs mod

fn calculate_inter_group_likelihood(
    group_a: &Vec<i32>, 
    group_b: &Vec<i32>, 
    adjacency_list: &HashMap<i32, Vec<i32>>
) -> f64 