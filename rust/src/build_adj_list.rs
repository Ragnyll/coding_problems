use std::collections::HashMap;

fn get_val_or_new(map: &HashMap<i32, Vec<i32>>, get: i32) -> Vec<i32> {
    if let Some(val) = map.get(&get) {
        return val.clone()
    }
    vec![]
}

fn build_adj_list(n: i32, edges: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    let mut adj_list = HashMap::with_capacity(n as usize);

    for edge in edges {
        // insert edge[0] => { .. + edge[1] }
        let mut new = get_val_or_new(&adj_list, edge[0]);
        new.push(edge[1]);
        adj_list.insert(edge[0], new);
        // insert edge[1] => { .. + edge[0] }
        let mut new = get_val_or_new(&adj_list, edge[1]);
        new.push(edge[0]);
        adj_list.insert(edge[1], new);
    }

    adj_list
}


#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::build_adj_list;

    #[test]
    fn test_build_adjacency_list() {
        let expected = HashMap::from([(1, vec![2, 3]), (2, vec![1, 3]), (3, vec![1, 2])]);
        assert_eq!(
            build_adj_list(3, vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            expected
        );
    }
}
