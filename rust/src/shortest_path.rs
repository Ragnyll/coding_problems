use std::collections::HashMap;
use std::collections::VecDeque;
use std::collections::HashSet;

/// given an adjacency list of nodes finds the distance (num edges travesersed) from src to target
#[allow(dead_code)]
fn find_shortest_path(
    adj_list: &HashMap<String, Vec<String>>,
    src: &str,
    target: &str,
) -> Option<u16> {
    let mut visited: HashSet<String> = HashSet::new();
    // a queue of the nodes to traverse and their distance from source
    let mut queue = VecDeque::new();
    queue.push_back((src.clone(), 0));
    visited.insert(String::from(src));

    while !queue.is_empty() {
        // unwrap is safe as loop condition prevents unwrapping a None
        let node = queue.pop_front().unwrap();

        if node.0 == target {
            // return the distance (second item in pair)
            return Some(node.1);
        }

        // the target was not found. search the neighbors
        for neighbor in adj_list.get(node.0).unwrap() {
            if visited.insert(String::from(neighbor)) {
                queue.push_back((neighbor, node.1 + 1));
            }
        }
    }
    None
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::find_shortest_path;

    #[test]
    fn test_find_shortest_path_no_path() {
        let mut adj_list = HashMap::new();
        adj_list.insert(String::from("a"), vec![String::from("b")]);
        adj_list.insert(String::from("b"), vec![String::from("a")]);
        adj_list.insert(String::from("c"), vec![]);

        assert_eq!(find_shortest_path(&adj_list, &"a", &"c"), None);
    }

    #[test]
    fn test_find_shortest_path_has_path() {
        // not a true adj_list as the path is only expressed through 1 direction
        let mut adj_list = HashMap::new();
        adj_list.insert(
            String::from("a"),
            vec![String::from("b"), String::from("d")],
        );
        adj_list.insert(String::from("b"), vec![String::from("c")]);
        adj_list.insert(String::from("c"), vec![String::from("e")]);
        adj_list.insert(String::from("d"), vec![String::from("e")]);

        // shortest path a -> d -> e
        assert_eq!(find_shortest_path(&adj_list, &"a", &"e").unwrap(), 2);
        // src == target, dist = 0
        assert_eq!(find_shortest_path(&adj_list, &"a", &"a").unwrap(), 0);
    }
}
