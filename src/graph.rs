use std::collections::HashMap;
use std::error::Error;
use csv::ReaderBuilder;

//Initializing the graph by using an adajancy list
pub fn add_edge(adjacency_list: &mut HashMap<i32, Vec<i32>>, start: i32, end: i32) { //adding edge to hash
    adjacency_list.entry(start).or_insert_with(Vec::new).push(end); //add end
}

pub fn make_graph(path: &str) -> Result<HashMap<i32, Vec<i32>>, Box<dyn Error>> { //making directed graph
    let mut reader = ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;
    
    //Get headers and column indices becuase need to be usize not string
    let headers = reader.headers()?;
    let id_1_index = headers.iter().position(|h| h == "id_1").ok_or("Missing id_1 column")?;
    let id_2_index = headers.iter().position(|h| h == "id_2").ok_or("Missing id_2 column")?;
    
    let mut adj_list: HashMap<i32, Vec<i32>> = HashMap::new();
    
    for result in reader.records() {
        let record = result?; //Parse each row
        
        //if there is both id1 and id2, add to adj list
        if let (Some(start), Some(end)) = (record.get(id_1_index), record.get(id_2_index)) {
            if let (Ok(start), Ok(end)) = (start.parse::<i32>(), end.parse::<i32>()) {
                add_edge(&mut adj_list, start, end);
            }
        }
    }
    Ok(adj_list)
}