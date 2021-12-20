// In this problem, a tree is an undirected graph that is connected and has no cycles.
//
// You are given a graph that started as a tree with n nodes labeled from 1 to n, with one additional edge added. The added edge has two different vertices chosen from 1 to n, and was not an edge that already existed. The graph is represented as an array edges of length n where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the graph.
//
// Return an edge that can be removed so that the resulting graph is a tree of n nodes. If there are multiple answers, return the answer that occurs last in the input.
use std::collections::HashMap;
use std::collections::HashSet;

/// Returns if a path exists from src to destination given an adjacency_list
#[allow(dead_code)]
fn df_has_path(
    adjacency_list: &HashMap<i32, HashSet<i32>>,
    src: i32,
    dest: i32,
    mut visited: HashSet<i32>,
) -> bool {
    visited.insert(src);
    if src == dest {
        return true;
    }

    for neighbor in &adjacency_list[&src] {
        if visited.contains(neighbor)
            && df_has_path(&adjacency_list, *neighbor, dest, visited.clone())
        {
            return true;
        }
    }
    false
}

fn insert_edge_into_graph(
    edge: Vec<i32>,
    graph: HashMap<i32, HashSet<i32>>,
) -> HashMap<i32, HashSet<i32>> {
    let mut new_graph = graph.clone();
    match new_graph.get_mut(&edge[0]) {
        Some(ns) => {
            let mut new_neighbors = ns.clone();
            new_neighbors.insert(edge[1]);
            new_graph.insert(edge[0], new_neighbors);
        }
        None => {
            let mut new_neighbors = HashSet::new();
            new_neighbors.insert(edge[1]);
            new_graph.insert(edge[0], new_neighbors);
        }
    };
    match new_graph.get_mut(&edge[1]) {
        Some(ns) => {
            let mut new_neighbors = ns.clone();
            new_neighbors.insert(edge[0]);
            new_graph.insert(edge[1], new_neighbors);
        }
        None => {
            let mut new_neighbors = HashSet::new();
            new_neighbors.insert(edge[0]);
            new_graph.insert(edge[1], new_neighbors);
        }
    };

    new_graph
}

fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::with_capacity(2 * edges.len());

    for edge in edges.clone() {
        graph.insert(edge[0], HashSet::new());
        graph.insert(edge[1], HashSet::new());
    }

    for edge in edges {
        let visited = HashSet::new();
        if graph.get(&edge[0]).is_some()
            && graph.get(&edge[1]).is_some()
            && df_has_path(&graph, edge[0], edge[1], visited)
        {
            return vec![edge[0], edge[1]];
        }
        // the edge is not redudnant
        graph = insert_edge_into_graph(edge, graph);
    }

    vec![]
}
