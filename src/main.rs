//import the modules that will be required 
mod read;
mod central;

//import the libraries that will be used 
use std::collections::HashMap;

//find the top 4 most important nodes in the graph
fn main() {
    // create the graph based on the file
    let filename = "roadNet-PA.txt";
    let edges: Vec<(usize, usize, f64)> = read::read_file(filename);
    let graph = read::build_graph(edges);

    // Find all the important nodes 
    let important = central::importance(&graph).unwrap();

    // Print the top 4 most central nodes
    println!("Top 4 most central nodes:");
    let mut nodes = important.iter().take(4).cloned().collect::<Vec<_>>();

    //sort it by biggest to smallest if frequency is the same
    nodes.sort_unstable_by(|a, b| b.1.cmp(&a.1).then_with(|| b.0.cmp(&a.0)));
    for (i, node) in nodes.iter().enumerate() {
        println!("{}. Node {} (frequency: {})", i + 1, node.0, node.1);
    }
}

//2 test
#[test]
fn test_compute_bfs() {
    let mut graph: HashMap<usize, Vec<(usize, usize, f64)>> = HashMap::new();
    graph.insert(0, vec![(1, 0, 1.0), (3, 0, 1.0)]);
    graph.insert(1, vec![(0, 1, 1.0), (2, 1, 1.0), (4, 1, 1.0)]);
    graph.insert(2, vec![(1, 2, 1.0), (4, 2, 1.0)]);
    graph.insert(3, vec![(0, 3, 1.0), (4, 3, 1.0)]);
    graph.insert(4, vec![(1, 4, 1.0), (2, 4, 1.0), (3, 4, 1.0)]);
    
    let (distance, path) = central::compute_bfs(&graph).unwrap();
    assert_eq!(distance[0], Some(0));
    assert_eq!(distance[1], Some(1));
    assert_eq!(distance[2], Some(2));
    assert_eq!(distance[3], Some(1));
    assert_eq!(distance[4], Some(2));
    
    assert_eq!(path[0], Some(vec![0]));
    assert_eq!(path[1], Some(vec![0, 1]));
    assert_eq!(path[2], Some(vec![0, 1, 2]));
    assert_eq!(path[3], Some(vec![0, 3]));
    assert_eq!(path[4], Some(vec![0, 1, 4]));
}
//ex: pass the test 

#[test]
fn fail_compute_bfs() {
    let mut graph: HashMap<usize, Vec<(usize, usize, f64)>> = HashMap::new();
    graph.insert(0, vec![(1, 0, 1.0), (3, 0, 1.0)]);
    graph.insert(1, vec![(0, 1, 1.0), (2, 1, 1.0), (4, 1, 1.0)]);
    graph.insert(2, vec![(1, 2, 1.0), (4, 2, 1.0)]);
    graph.insert(3, vec![(0, 3, 1.0), (4, 3, 1.0)]);
    graph.insert(4, vec![(1, 4, 1.0), (2, 4, 1.0), (3, 4, 1.0)]);
    
    let (distance, path) = central::compute_bfs(&graph).unwrap();
    assert_eq!(distance[0], Some(0));
    assert_eq!(distance[1], Some(1));
    assert_eq!(distance[2], Some(2));
    assert_eq!(distance[3], Some(1));
    assert_eq!(distance[4], Some(2));
    
    assert_eq!(path[0], Some(vec![0]));
    assert_eq!(path[1], Some(vec![2, 1]));
    assert_eq!(path[2], Some(vec![0, 1, 3]));
    assert_eq!(path[3], Some(vec![0, 1]));
    assert_eq!(path[4], Some(vec![0, 1, 4]));
}
//ex: fail the test
