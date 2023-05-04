//import the necessary libraries 
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

//define the Node, Edge, Graph 
type Node = usize;
type Edge = (Node,Node,f64);
type Graph = HashMap<Node,Vec<Edge>>;

//compute breadth first search
pub fn compute_bfs(graph: &Graph) -> Option<(Vec<Option<u32>>, Vec<Option<Vec<Node>>>)> {

    //create a node_set to be iterated over with
    let mut node_set = HashSet::new();
    for edges in graph.values() {
        for &(u, v, _) in edges {
            node_set.insert(u); 
            node_set.insert(v);
        }
    }

    //make sure that the iteration is always within bound by adding 1 
    let max_node_id = graph.keys().max().unwrap_or(&0);
    let num_nodes = (*max_node_id + 1) as usize;
    let mut distance: Vec<Option<u32>> = vec![None; num_nodes];
    let mut path: Vec<Option<Vec<Node>>> = vec![None; num_nodes];

    //make sure that bfs is always conducted in the same order 
    let mut node_ids: Vec<Node> = node_set.into_iter().collect();
    node_ids.sort();
    
    //find the shortest path - inspiration (lecture notes on BFS + Dijkstra)
    for start in node_ids  {
        if distance[start] == None {
            distance[start] = Some(0);
            path[start] = Some(vec![start]);

            //create a priority queue
            let mut queue = BinaryHeap::new();
            queue.push(std::cmp::Reverse((start, 0)));

            while let Some(std::cmp::Reverse((v, dist))) = queue.pop() {
                if distance[v] != Some(dist) {
                    continue;
                }
                for &(u, v, weight) in &graph[&v] {
                    if distance[u] == None {
                        distance[u] = Some(dist + weight as u32);
                        let mut p = path[v].clone().unwrap();
                        p.push(u);
                        path[u] = Some(p);
                        queue.push(Reverse((u, dist + weight as u32)));
                    }
                }
            }
        }
    }
    Some((distance, path))
}

//create a function to calculate betweenness centrality
//a simplify version based on frequency - returning the highest 

pub fn importance(graph: &Graph) -> Result<Vec<(Node, usize)>, &'static str> {
    let mut importance: HashMap<Node, f64> = HashMap::new();

    // calculate the importance of each node
    for (source, edges) in graph {
        for (target, _, weight) in edges {
            *importance.entry(*source).or_insert(0.0) += weight;
            *importance.entry(*target).or_insert(0.0) += weight;
        }
    }

    // sort the nodes by importance in descending order
    let mut nodes: Vec<(Node, f64)> = importance.into_iter().collect();
    nodes.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    // get the top 5 nodes by importance
    let mut result = Vec::new();
    for (node, _) in nodes.iter().take(5) {
        result.push((*node, graph.get(node).unwrap().len()));
    }

    Ok(result)
}

