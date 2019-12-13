use a_star::prog::Program;

fn main() 
{
    // Import route data
    let route_dat_text = std::fs::read_to_string("routes.txt")
        .expect("Undefined io error when reading \"routes.txt\"");

    // Import heuristic data
    let heur_dat_text = std::fs::read_to_string("euclidian.txt")
        .expect("Undefined io error when reading \"euclidian.txt\"");

    // Create and run Program
    let mut prog = Program::new(&route_dat_text, &heur_dat_text);
    prog.run(); 
}



