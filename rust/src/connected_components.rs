use std::collections::HashMap;
use std::collections::HashSet;

/// given a map of adjacent_nodes (adj_list) finds the number of distinct graphs
#[allow(dead_code)]
fn count_connected_components(adj_list: &HashMap<String, Vec<String>>) -> u32 {
    let mut visited: HashSet<String> = HashSet::new();
    // the number of distinct components
    let mut connected_component_count = 0;
    for node in adj_list {
        if explore(node.0, adj_list, &mut visited) {
            connected_component_count += 1;
        }
    }
    connected_component_count
}

#[allow(dead_code)]
fn explore(
    start_node: &str,
    adjacency_list: &HashMap<String, Vec<String>>,
    visited: &mut HashSet<String>,
) -> bool {
    if visited.contains(start_node) {
        return false;
    }

    visited.insert(String::from(start_node));

    let neighbors = adjacency_list.get(start_node).unwrap();
    for neighbor in neighbors {
        explore(neighbor, adjacency_list, visited);
    }

    true
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::count_connected_components;

    #[test]
    fn test_count_connected_components_no_islands() {
        assert_eq!(count_connected_components(&HashMap::new()), 0);
    }

    #[test]
    fn test_count_connected_components_one_island() {
        let mut graph = HashMap::new();
        graph.insert(
            String::from("0"),
            vec![String::from("1"), String::from("2")],
        );
        graph.insert(
            String::from("1"),
            vec![String::from("0"), String::from("2")],
        );
        graph.insert(
            String::from("2"),
            vec![String::from("0"), String::from("1")],
        );
        assert_eq!(count_connected_components(&graph), 1);
    }

    #[test]
    fn test_count_connected_components_many_islands() {
        let mut graph = HashMap::new();
        graph.insert(
            String::from("0"),
            vec![String::from("8"), String::from("1"), String::from("5")],
        );
        graph.insert(String::from("1"), vec![String::from("0")]);
        graph.insert(
            String::from("5"),
            vec![String::from("0"), String::from("8")],
        );
        graph.insert(
            String::from("8"),
            vec![String::from("0"), String::from("5")],
        );
        graph.insert(
            String::from("2"),
            vec![String::from("4"), String::from("3")],
        );
        graph.insert(
            String::from("3"),
            vec![String::from("4"), String::from("2")],
        );
        graph.insert(
            String::from("4"),
            vec![String::from("3"), String::from("2")],
        );

        assert_eq!(count_connected_components(&graph), 2);
    }
}
