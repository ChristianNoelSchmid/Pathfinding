use std::collections::HashMap;
use std::cmp::Reverse;

use petgraph::graphmap::UnGraphMap;
use priority_queue::PriorityQueue;
use stopwatch::Stopwatch;

use super::io::*;

pub struct Program<'a>
{
    route_dat: UnGraphMap<&'a str, u64>,
    heur_map: HashMap<(&'a str, &'a str), u64>,
}

impl<'a> Program<'a>
{
    /// 
    /// Creates a new program, by building a Graph and heuristic 
    /// HashMap from arguments.
    /// 
    /// route_file_txt: the route information, by which the
    ///     Graph will be built
    /// 
    /// heur_file_txt: the heuristic information, by which the
    ///     heuristic HashMap will be built
    /// 
    pub fn new(route_file_txt: &'a String, heur_file_txt: &'a String) -> Self
    {
        Program 
        { 
            route_dat: build_map(route_file_txt), 
            heur_map: build_heur_data(heur_file_txt)
        }
    }

    ///
    /// Runs the Program, guiding the user through a loop until they
    /// enter "quit". Asks user to provide a starting point and destination,
    /// then calling the find_shortest_route method to traverse from start
    /// to finish using A* and Djikstra's (comparing the two)
    /// 
    pub fn run(&mut self)
    {
        // Loop until user quites
        loop
        {
            // Clear the screen and print all possible locations in Graph
            clear_screen();            
            println!("Your Locations:\n");
            for (i, node) in self.route_dat.nodes().enumerate()
            {
                print!("{0:<15}", node); 
                if i % 5 == 4 { println!(); }
            }

            // Prompt for and retrieve start and finish location(s)
            println!("--\nWhat city are you starting at?");
            println!("Type \"Quit\" at any time to exit.");
            let from = input(false);
            if from.to_lowercase() == "quit" { break; }

            println!("What city are you going to?");
            let to = input(false);
            if to.to_lowercase() == "quit" { break; }
          
            clear_screen();

            // Run the method, first with the A* heuristic, then with
            // Djikstra. Track the time taken for both to complete and display at
            // finish
            println!("\nRunning A* Algorithm...");
            match self.find_shortest_route(&from, &to, true)
            {
                Err(e) => println!("{}", e),
                Ok(elapsed) => 
                {
                    println!("\nRunning Djikstra Algorithm...");
                    let a_star_time = elapsed;
                    let djik_time = self.find_shortest_route(&from, &to, false).unwrap();

                    println!("--");
                    println!("A* time to compute: {} micros.", a_star_time);
                    println!("Djikstra time to compute: {} micros.\n", djik_time);
                }
            };
           
            // Wait for ENTER as user looks over results
            wait_for_enter();
        }        
    }

    /// 
    /// Computes the shortest route between two nodes on a Graph
    /// Uses either A* or Djikstra's algorithm, depending on a_star value
    /// 
    /// - start: the start location on the Graph
    /// - end: the end location on the Graph
    /// - a_star: determines if A* heuristic method is implemented
    /// 
    /// - Return: Either an Ok Result with the amount of time taken to compute path,
    ///   or an Err with message explaining problem
    /// 
    fn find_shortest_route(&self, start: &'a str, end: &'a str, a_star: bool) -> Result<u128, String>
    {
        let mut sw = Stopwatch::new();

        // If provided start or end node does not exist, prompt the
        // user of this, and return Err
        if !self.route_dat.contains_node(start) ||
           !self.route_dat.contains_node(end)
        {
            return Err(String::from("Cannot route: one or more locations do not exist."));
        }

        sw.start();

        // Create a priority queue, which will hold all route information,
        // and automatically supply the shortest distance route
        // Push start node onto queue
        let mut route_dists = PriorityQueue::new();
        route_dists.push(start, Reverse(0));

        //
        // A HashMap for each node's distance from start on the Graph.
        // The key is the node in question, while the value is the distance.
        //
        // Ensures quick retrieval of distances, and stores base distance while
        // performing A* search (routes_dists will store base distance + heuristic
        // in this case)
        //
        let mut dist: HashMap<&str, u64> = HashMap::new();
        dist.insert(start, 0);

        // A marker for each node in the Graph, representing which adjacent
        // node provides the path of least distance
        let mut prev: HashMap<&str, &str> = HashMap::new();

        // Counter for total # of nodes considered
        let mut node_counter = 0;

        // Loop through all routes
        loop
        {
            match route_dists.pop()
            {
                // While there any existing routes
                Some(min_route) => 
                {
                    node_counter += 1;
                    // If min_route is the destination node
                    if min_route.0 == end
                    {
                        sw.stop();

                        // Print # of nodes considered
                        println!("{} nodes considered", node_counter);

                        // Print shortest route (if A*)
                        if a_star { self.print_shortest_route(prev, end, dist[end]); }

                        // Return time taken to compute (in microseconds)
                        return Ok(sw.elapsed().as_micros());
                    }

                    // For every frontier node for the min_route node
                    for edge in self.route_dat.edges(min_route.0)
                    {
                        // min_route.0 and edge.0 will be the same value

                        // Find the total weight distance between min_route node and its
                        // edge node.
                        let alt_route = dist[min_route.0] + edge.2;

                        // If that value does not yet exist in dist, or if dist is greater,
                        // update dist and prev, and push alt_route into queue
                        if !dist.contains_key(edge.1) || alt_route < dist[edge.1]
                        {
                            // Set dist of edge node to alt_route value
                            dist.insert(edge.1, alt_route);
                            
                            // Set prev of edge node to min_route - it is the new
                            // previous node to the edge node
                            prev.insert(edge.1, min_route.0);

                            // Update edge node on routes priority queue to alt_route
                            // Include heuristic if a_star
                            if a_star { route_dists.push(edge.1, Reverse(alt_route + self.heur_map[&(edge.1, end)])); }
                            else { route_dists.push(edge.1, Reverse(alt_route)); }
                        }
                    }
                },

                // If no other routes exist, return Err - destination could not be reached
                None => return Err(String::from("Route could not be completed!"))
            };
        }
    }

    ///
    /// Prints the shortest route starting at destination, recursively working back to start
    /// 
    /// - prev: all HashMap data associated with searched nodes and their minimum weighted
    ///         previous nodes
    /// - to: the ending location
    /// - total_dist: the total distance in miles required to traverse path
    /// 
    fn print_shortest_route(&self, prev: HashMap<&str, &str>, to: &str, total_dist: u64)
    {
        // Pass end location into helper method
        let prv: &str = prev[to];
        self.print_shortest_route_helper(&prev, prv, to);

        // Print total distance after path has been printed
        println!("Total distance: {:.1} mi.", (total_dist as f64) / 10.0);
    }

    ///
    /// Helper function that performs DFS, printing shortest path from start to finish
    /// 
    /// - prev: all HashMap data associated with searched nodes and their minimum weighted
    ///         previous nodes
    /// - prv: the current previous node being considered
    /// - next: the node directly after prv in the shortest path
    /// 
    fn print_shortest_route_helper(&self, prev: &HashMap<&str, &str>, prv: &str, next: &str)
    {
        // If start node has yet to be reached, call method on previous node in path
        if prev.contains_key(prv) { self.print_shortest_route_helper(prev, prev[prv], prv); }

        // Print node information
        println!("Take {} to {}: {:.1} mi.", prv, next, (*self.route_dat.edge_weight(prv, next).unwrap() as f64) / 10.0);
    }
}

/// 
/// Build an Undirected Adjacency List Graph off of
/// the supplied input
/// 
/// - route_dat: the input data, as a borrowed String
/// 
/// - return: an UnGraphMap with u64 weight edges. The float value
///   provided from route_dat is rounded to 1 decimal place, and multipled
///   by 10, to maintain precision, but allow complete ordering
/// 
fn build_map<'a>(route_dat: &'a String) -> UnGraphMap<&'a str, u64>
{
    // Define the graph to return
    let mut graph = UnGraphMap::new();

    // Split the route data into separate lines
    let route_dat = route_dat.split('\n')
        .collect::<Vec<&'a str>>();

    // For each line, add two Nodes and
    // Edge into the graph
    for line in route_dat
    {
        // Trim parens
        let line = line.trim_matches(|c| { c == '(' || c == ')' });

        // Split by commas
        let data = line.split(',')
            .map(|val| { val.trim() })
            .collect::<Vec<&'a str>>();
        
        // 1st item - the starting node
        // 2nd item - the ending node
        let (route_from, route_to) = (data[0], data[1]);

        // Round weight to nearest 10th, and convert to u64
        let weight = (data[2].parse::<f64>().unwrap() * 10.0).round() as u64;

        // Add the edge to the Graph.
        graph.add_edge(route_from, route_to, weight);
    }

    // Return the graph
    graph
}

///
/// Retrieves all Heuristic data from euclidian.txt
/// Returns as a HashMap, with key values being the 2-ple of the
/// two borrowed String slices, and the value being the distance between.
/// 
/// - input: the input-data, as a borrowed String
/// 
/// - return: the generated HashMap, with u64 type values. The float value
///   provided from route_dat is rounded to 1 decimal place, and multipled
///   by 10, to maintain precision, but allow complete ordering
/// 
fn build_heur_data<'a>(input: &'a String) -> HashMap<(&'a str, &'a str), u64>
{
    // HashMap of data - returned value
    let mut dist_dat = HashMap::new();

    // Split input by line
    let input = input.split('\n')
        .collect::<Vec<&'a str>>();

    // For each line of input
    for line in input
    {
        // Collect the data, seperated by spaces
        let data = line.split(' ').collect::<Vec<&str>>();

        // Assign from and to node (edge) to vars
        let (from, to) = (data[0], data[1]);

        // Round distance to nearest 10th and convert to u64
        let dist = (data[2].parse::<f64>().unwrap() * 10.0).round() as u64;

        // Insert data
        dist_dat.insert((from, to), dist);
    }

    dist_dat
}