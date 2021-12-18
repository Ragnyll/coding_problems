// In this problem, a tree is an undirected graph that is connected and has no cycles.
//
// You are given a graph that started as a tree with n nodes labeled from 1 to n, with one additional edge added. The added edge has two different vertices chosen from 1 to n, and was not an edge that already existed. The graph is represented as an array edges of length n where edges[i] = [ai, bi] indicates that there is an edge between nodes ai and bi in the graph.
//
// Return an edge that can be removed so that the resulting graph is a tree of n nodes. If there are multiple answers, return the answer that occurs last in the input.
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

/// Returns if a path exists from src to destination given an adjacency_list
#[allow(dead_code)]
fn df_has_path(adjacency_list: &HashMap<i32, HashSet<i32>>, src: i32, dest: i32) -> bool {
    if src == dest {
        return true;
    }

    for neighbor in &adjacency_list[&src] {
        if df_has_path(&adjacency_list, *neighbor, dest) {
            return true;
        }
    }
    false
}

#[allow(dead_code)]
fn bf_has_path(adjacency_list: &HashMap<i32, HashSet<i32>>, src: i32, dest: i32) -> bool {
    if src == dest {
        return true;
    }

    let mut queue = VecDeque::new();
    queue.push_back(src);

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        if current == dest {
            return true;
        }

        // get the current's neighbors
        for neighbor in &adjacency_list[&current] {
            queue.push_back(neighbor.clone());
        }
    }

    false
}

fn build_adjacency_list(edges: Vec<Vec<i32>>) -> HashMap<i32, HashSet<i32>> {
    // adj list will be a hash map mapping a course to the prerequisite
    let mut adj_list: HashMap<i32, HashSet<i32>> = HashMap::with_capacity(edges.len());

    for pair in edges {
        // insert both nodes connected by edge
        match adj_list.get(&pair[0]) {
            Some(connected_nodes) => {
                let mut new_node_set = connected_nodes.clone();
                new_node_set.insert(pair[1]);
                adj_list.insert(pair[0], new_node_set);
            }
            None => {
                adj_list.insert(pair[0], HashSet::from([pair[1]]));
            }
        };

        match adj_list.get(&pair[1]) {
            Some(connected_nodes) => {
                let mut new_node_set = connected_nodes.clone();
                new_node_set.insert(pair[0]);
                adj_list.insert(pair[1], new_node_set);
            }
            None => {
                adj_list.insert(pair[1], HashSet::from([pair[0]]));
            }
        };
    }

    adj_list
}

fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
    let mut graph: HashMap<i32, HashSet<i32>> = HashMap::new();
    for edge in edges {
        if graph.contains_key(&edge[0]) && graph.contains_key(&edge[1]) && df_has_path(&graph, edge[0], edge[1]) {
            return vec![edge[0], edge[1]]
        }

        match graph.get(&edge[0]) {
            Some(connected_nodes) => {
                let mut new_node_set = connected_nodes.clone();
                new_node_set.insert(edge[1]);
                graph.insert(edge[0], new_node_set);
            }
            None => {
                // the rust 2021 syntax of hashset from is just better
                let mut new_hs = HashSet::new();
                new_hs.insert(edge[1]);
                graph.insert(edge[0], HashSet::from([edge[1]]));
            }
        };

        match graph.get(&edge[1]) {
            Some(connected_nodes) => {
                let mut new_node_set = connected_nodes.clone();
                new_node_set.insert(edge[0]);
                graph.insert(edge[1], new_node_set);
            }
            None => {
                graph.insert(edge[1], HashSet::from([edge[0]]));
            }
        };

    }

    vec![]
}

#[cfg(test)]
mod test {
    use std::collections::{HashMap, HashSet};
    use super::build_adjacency_list;

    #[test]
    fn test_build_adjacency_list() {
        let edges = vec![vec![1, 2], vec![2,3]];
        let mut expected = HashMap::new();
        expected.insert(1, HashSet::from([2]));
        expected.insert(2, HashSet::from([1, 3]));
        expected.insert(3, HashSet::from([2]));

        assert_eq!(build_adjacency_list(edges), expected);
    }

}
