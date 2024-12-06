use std::error::Error;
use rand::Rng;
use plotters::prelude::*;

mod graph;
mod pagerank;
mod bfs_friends;
//mod visual;



fn main() -> Result<(), Box<dyn Error>> {
    let path = "git_edges.csv";
    let adjacency_list = graph::make_graph(path)?; //creating directed graph <HashMap<i32, Vec<i32>>
    
    //get pagerank scores
    let num_vertices = 37700; // number of nodes in data set - 1
    
    //getting best number of steps
    let max_walks = 15000;
    let tolerance = 1e-2;
    let walk_length = 100;
    
    // println!("\nThe following is the most_accurate_walk_count iterating... This will take around 10 minutes :) ");
    // let num_walks = pagerank::most_accurate_walk_count(&adjacency_list, num_vertices, max_walks, tolerance, walk_length);
    // println!("the ideal number of walks is {}", num_walks);
    let num_walks = 380;
    
    
    //printing 5 pageranks
    println!("\nThis is the pagerank of the 25 most connected verticies.");
    let pagerank_scores = pagerank::pagerank(&adjacency_list, num_vertices, num_walks, walk_length);
    let mut top_25_pagerank_vec: Vec<_> = pagerank_scores.iter().collect();
    top_25_pagerank_vec.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap()); // Sort by Pagercaank score descending
    
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
    
    for (circle_number, circle_user_tuple) in circles_vec.iter().take(10) {
        println!("10 random circles look like:");
        println!("circle id: {} users: {:?}", circle_number, circle_user_tuple);
    }


    let mut under_ten = 0;
    let mut ten_to_hundred = 0;
    let mut hundred_five_hundred = 0;
    let mut five_hundred_plus = 0;

    for (circle_number, circle_user_tuple) in circles_vec.iter().enumerate() {
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
    println!("Circles with less than 10 users: {} ", under_ten); //8151
    println!("Circles with 10-100 users: {}", ten_to_hundred); //22
    println!("Circles with 101-500 users: {}", hundred_five_hundred); //1
    println!("Circles with more than 500 users: {}", five_hundred_plus); //5
    


    // categories and data
    let categories = vec!["0-10", "10-100", "100-500", "500+"];
    let data = vec![under_ten, ten_to_hundred, hundred_five_hundred, five_hundred_plus];

    let drawing_area = SVGBackend::new("histogram_vertical.svg", (300, 200)).into_drawing_area();
    drawing_area.fill(&WHITE)?;

    let mut chart_builder = ChartBuilder::on(&drawing_area);
    chart_builder.margin(5).set_left_and_bottom_label_area_size(20);

    let mut chart_context = chart_builder.build_cartesian_2d(
        0..categories.len() as u32,      // X-axis (number of categories)
        0..*data.iter().max().unwrap_or(&0), // Y-axis (max value in data)
    )?;

    chart_context.configure_mesh().draw()?; // Configure and draw grid lines and labels

    chart_context.draw_series( // Draw the histogram bars
        Histogram::vertical(&chart_context)
            .style(BLUE.filled())
            .margin(10)
            .data(data.into_iter().enumerate().map(|(i, x)| {
                // Return the category index as the X-axis position and the frequency as the Y value
                (i as u32, x)  // The X is the category index, Y is the count
            })),
    )?;
    

    Ok(())
    
}

