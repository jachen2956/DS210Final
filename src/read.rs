//import the necessary libraries 
use std::fs::File;
use std::io::{BufRead,BufReader};
use std::collections::HashMap;

//define Node, Edge, Graph that will be used 
type Node = usize;
type Edge = (Node,Node,f64);
type Graph = HashMap<Node,Vec<Edge>>;

//read the file 
pub fn read_file(filename:&str) -> Vec<Edge> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut edges = Vec::new();

    //skip the 1st four lines b/c its irrelevant 
    for line in reader.lines().skip(4) {
        let line: String = line.unwrap();
        let location: Vec<&str> = line.split('\t').collect();
        let node: usize = location[0].parse().expect("Error parsing node");
        let new_node : usize = location[1].parse().expect("Error parsing node");
        let distance:f64 = 1.0;

        //define edge 
        let edge: (usize, usize, f64) = (node,new_node,distance);
        edges.push(edge);

}
    edges
}

//create the graph based on the file 
pub fn build_graph(edges:Vec<Edge>) -> Graph {
    
    //create a empty graph 
    let mut graph = Graph::new();

    //for edges put it into the empty graph
    for (node,new_node,distance) in edges {
        graph.entry(node)
        .or_insert_with(Vec::new)
        .push((node,new_node,distance));

    }
    graph  
}
