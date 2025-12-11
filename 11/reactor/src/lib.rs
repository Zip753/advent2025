pub type AdjacencyList = Vec<Vec<usize>>;

fn dfs(from: usize, graph: &AdjacencyList, visited: &mut [bool], sorted: &mut Vec<usize>) {
    if visited[from] {
        return;
    }
    visited[from] = true;

    for &to in &graph[from] {
        dfs(to, graph, visited, sorted);
    }
    sorted.push(from);
}

/// Returns list of indexes in the order of start -> nodes without outgoing edges
fn toposort(graph: &AdjacencyList, start: usize) -> Vec<usize> {
    let mut visited: Vec<bool> = vec![false; graph.len()];
    let mut sorted: Vec<usize> = vec![];

    dfs(start, graph, &mut visited, &mut sorted);

    sorted.reverse();
    sorted
}

#[test]
fn toposort_simple() {
    let graph: AdjacencyList = vec![vec![2, 1], vec![2], vec![]];

    let result = toposort(&graph, 0);

    assert_eq!(result, vec![0, 1, 2]);
}

pub fn total_routes(graph: &AdjacencyList, start: usize, end: usize) -> Option<u64> {
    // run toposort
    let sorted_idx = toposort(graph, start);

    // initialise counts to 0
    let mut count: Vec<Option<u64>> = vec![None; graph.len()];
    // initialise start count to 1
    count[start] = Some(1);

    // walk the graph in toposort order
    for &from in &sorted_idx {
        let edges = &graph[from];
        let from_count = count[from].expect("already calculated");
        // for each edge
        for &to in edges {
            count[to] = Some(count[to].unwrap_or(0) + from_count);
        }
    }

    // return count for end
    count[end]
}

pub fn total_routes_through_nodes(
    graph: &AdjacencyList,
    start: usize,
    end: usize,
    through: &[usize],
) -> Option<u64> {
    // run toposort
    let sorted_idx = toposort(graph, start);

    let through_masks = 1 << through.len();

    // initialise counts to 0
    let mut count: Vec<Vec<Option<u64>>> = vec![vec![None; through_masks]; graph.len()];
    // initialise start count to 1
    count[start][0] = Some(1);

    // walk the graph in toposort order
    for &from in &sorted_idx {
        let edges = &graph[from];
        let t_idx = through.iter().position(|&t| t == from);

        eprintln!("t_idx: {:?} (from: {})", t_idx, from);

        for mask in 0..through_masks {
            if let Some(from_count) = count[from][mask] {
                eprintln!("we're iterating! {}/{} (count {})", from, mask, from_count);
                // for each edge
                for &to in edges {
                    *count[to][mask].get_or_insert(0) += from_count;
                    if let Some(t_idx) = t_idx {
                        let mask_t = mask | (1 << t_idx);
                        eprintln!("passing through {} (idx {}), to {}, mask {} -> {}", from, t_idx, to, mask, mask_t);
                        *count[to][mask_t].get_or_insert(0) += from_count;
                    }
                }
            }
        }
    }

    // return count for end
    count[end][through_masks - 1]
}
