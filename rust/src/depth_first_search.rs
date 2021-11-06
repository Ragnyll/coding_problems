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

#[cfg(test)]
mod test {
    use super::{df_print, df_print_recursive};
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
}
