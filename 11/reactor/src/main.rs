use std::collections::{HashMap, HashSet};
use std::fs;

use reactor::AdjacencyList;

fn main() {
    let file = fs::read_to_string("../input.txt").expect("Failed to open input.txt");

    // initialise adjacency list
    let mut graph_str: HashMap<&str, Vec<&str>> = HashMap::new();

    // for each line
    for line in file.lines() {
        // read from: to to to to
        let (from, tos) = line.split_once(": ").expect("should find a colon");
        let tos = tos.split_whitespace();

        // add entry to adjacency list
        graph_str.insert(from, tos.collect());
    }

    // eprintln!("{:?}", graph_str);

    let (graph, you, out, svr, fft, dac) = to_idx_graph(&graph_str);
    eprintln!("you: {}, out: {}, svr: {}, fft: {}, dac: {}", you, out, svr, fft, dac);

    // call solve
    // let result = reactor::total_routes(&graph, you, out);
    let result = reactor::total_routes_through_nodes(&graph, svr, out, &[fft, dac]);

    println!("{:?}", result);
}

fn to_idx_graph(
    graph_str: &HashMap<&str, Vec<&str>>,
) -> (AdjacencyList, usize, usize, usize, usize, usize) {
    let mut keys: HashSet<&str> = HashSet::new();
    for (from, edges) in graph_str {
        keys.insert(from);
        for to in edges {
            keys.insert(to);
        }
    }
    let keys: Vec<&str> = keys.into_iter().collect();

    let str_to_idx: HashMap<&str, usize> = keys.iter().enumerate().map(|(i, &k)| (k, i)).collect();

    let mut graph: AdjacencyList = Vec::with_capacity(keys.len());

    for from in keys {
        let edges = graph_str.get(from).map(|e| e.as_slice()).unwrap_or(&[]);

        let edge_idxs: Vec<usize> = edges.iter().map(|to| str_to_idx[to]).collect();

        graph.push(edge_idxs);
    }

    let you = str_to_idx["you"];
    let out = str_to_idx["out"];
    let svr = str_to_idx["svr"];
    let fft = str_to_idx["fft"];
    let dac = str_to_idx["dac"];

    (graph, you, out, svr, fft, dac)
}
