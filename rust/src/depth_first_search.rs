use std::collections::HashMap;

#[allow(dead_code)]
fn df_print(adjacency_list: HashMap<String, Vec<String>>, start: &str) {
    let mut stack = vec![String::from(start)];

    while !stack.is_empty() {
        let current = stack.pop().unwrap();
        println!("{}", current);

        // get the current's neighbors
        for neighbor in &adjacency_list[&current] {
            stack.push(neighbor.clone());
        }
    }
}

#[allow(dead_code)]
fn df_print_recursive(adjacency_list: HashMap<String, Vec<String>>, start: &str) {
    println!("{}", start);

    for neighbor in &adjacency_list[start] {
        df_print_recursive(adjacency_list.clone(), &neighbor);
    }
}

/// Returns if a path exists from src to destination given an adjacency_list
#[allow(dead_code)]
fn df_has_path(adjacency_list: &HashMap<String, Vec<String>>, src: &str, dest: &str) -> bool {
    if src == dest {
        return true;
    }

    for neighbor in &adjacency_list[src] {
        if df_has_path(&adjacency_list, neighbor, dest) {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod test {
    use super::{df_print, df_print_recursive, df_has_path};
    use std::collections::HashMap;

    #[test]
    fn valid_df_print() {
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

        df_print(adjacency_list, &"a");
    }

    #[test]
    fn valid_df_print_recursive() {
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

        df_print_recursive(adjacency_list, &"a");
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

        assert_eq!(df_has_path(&adjacency_list, &"a", &"b"), true);
        assert_eq!(df_has_path(&adjacency_list, &"a", &"d"), true);
        assert_eq!(df_has_path(&adjacency_list, &"a", &"g"), false);
    }
}
