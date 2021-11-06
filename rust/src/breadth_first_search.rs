use std::collections::HashMap;
use std::collections::VecDeque;

#[allow(dead_code)]
fn bf_print(adjacency_list: HashMap<String, Vec<String>>, start: &str) {
    let mut queue = VecDeque::new();
    queue.push_back(String::from(start));

    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        println!("{}", current);

        // get the current's neighbors
        for neighbor in &adjacency_list[&current] {
            queue.push_back(neighbor.clone());
        }
    }
}

#[allow(dead_code)]
fn bf_has_path(adjacency_list: &HashMap<String, Vec<String>>, src: &str, dest: &str) -> bool {
    if src == dest {
        return true;
    }

    let mut queue = VecDeque::new();
    queue.push_back(String::from(src));

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

// TODO: see if i can do this with a stack allocated array type
/// give a list of edge connections finds if there is a path from src to dest
#[allow(dead_code)]
fn bf_has_path_from_edges(edges: Vec<Vec<String>>, src: String, dest: String) -> bool {
    fn build_adjacency_list_from_edges(edges: Vec<Vec<String>>) -> HashMap<String, Vec<String>> {
        let mut adjacency_list: HashMap::<String, Vec::<String>> = HashMap::new();
        for edge in edges {
            // There will only ever be two nodes in an edge
            for node in edge {
                if !adjacency_list.contains_key(&node) {
                    adjacency_list.insert(node.clone(), vec![]);
                }
            }
            // put the neighbor in the list of adjacent nodes
            adjacency_list[&edge[0]].push(edge[1].clone());
            adjacency_list[&edge[1]].push(edge[0].clone());
        }
        adjacency_list
    }

    fn bf_has_path_undirected(
        adjacency_list: &HashMap<String, Vec<String>>,
        src: String,
        dest: String,
    ) -> bool {
        false
    }

    let adjacency_list = build_adjacency_list_from_edges(edges);
    bf_has_path_undirected(&adjacency_list, src, dest)
}

#[cfg(test)]
mod test {
    use super::{bf_print, bf_has_path};
    use std::collections::HashMap;

    #[test]
    fn valid_bf_print() {
        let mut adjacency_list = HashMap::new();
        adjacency_list.insert(
            String::from("a"),
            vec![String::from("b"), String::from("c")],
        );
        adjacency_list.insert(String::from("b"), vec![String::from("d")]);
        adjacency_list.insert(String::from("c"), vec![String::from("e")]);
        adjacency_list.insert(String::from("d"), vec![String::from("f")]);
        adjacency_list.insert(String::from("e"), vec![]);
        adjacency_list.insert(String::from("f"), vec![]);

        bf_print(adjacency_list, &"a");
    }

    #[test]
    fn test_df_has_path() {
        let mut adjacency_list = HashMap::new();
        adjacency_list.insert(
            String::from("a"),
            vec![String::from("b"), String::from("c")],
        );
        adjacency_list.insert(String::from("b"), vec![String::from("d")]);
        adjacency_list.insert(String::from("c"), vec![String::from("e")]);
        adjacency_list.insert(String::from("d"), vec![String::from("f")]);
        adjacency_list.insert(String::from("e"), vec![]);
        adjacency_list.insert(String::from("f"), vec![]);
        adjacency_list.insert(String::from("g"), vec![]);

        assert_eq!(bf_has_path(&adjacency_list, &"a", &"b"), true);
        assert_eq!(bf_has_path(&adjacency_list, &"a", &"d"), true);
        assert_eq!(bf_has_path(&adjacency_list, &"a", &"g"), false);
    }
}
