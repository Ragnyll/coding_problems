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

#[cfg(test)]
mod test {
    use super::bf_print;
    use std::collections::HashMap;

    #[test]
    fn valid_bf_print() {
        let mut adjacency_list = HashMap::new();
        adjacency_list.insert(String::from("a"), vec![String::from("b"), String::from("c")]);
        adjacency_list.insert(String::from("b"), vec![String::from("d")]);
        adjacency_list.insert(String::from("c"), vec![String::from("e")]);
        adjacency_list.insert(String::from("d"), vec![String::from("f")]);
        adjacency_list.insert(String::from("e"), vec![]);
        adjacency_list.insert(String::from("f"), vec![]);

        bf_print(adjacency_list, &"a");
    }


}
