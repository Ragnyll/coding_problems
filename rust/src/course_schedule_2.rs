// There are a total of numCourses courses you have to take, labeled from 0 to numCourses - 1. You are given an array prerequisites where prerequisites[i] = [ai, bi] indicates that you must take course bi first if you want to take course ai.

//     For example, the pair [0, 1], indicates that to take course 0 you have to first take course 1.

// Return the ordering of courses you should take to finish all courses. If there are many valid answers, return any of them. If it is impossible to finish all courses, return an empty array.
use std::collections::{HashMap, HashSet, VecDeque};

fn build_adjacency_list(num_items: i32, course_pairs: Vec<Vec<i32>>) -> HashMap<i32, Vec<i32>> {
    // adj list will be a hash map mapping a course to the prerequisite
    let mut adj_list: HashMap<i32, Vec<i32>> = HashMap::with_capacity(num_items as usize);

    for pair in course_pairs {
        if adj_list.contains_key(&pair[0]) {
            let mut course_prereqs: Vec<i32> = adj_list.get(&pair[0]).unwrap().to_vec();
            course_prereqs.push(pair[1]);
            adj_list.insert(pair[0], course_prereqs);
        } else {
            adj_list.insert(pair[0], vec![pair[1]]);
        }
    }

    adj_list
}


#[cfg(test)]
mod test {
    use std::collections::HashMap;
    use super::{build_adjacency_list};

    #[test]
    fn test_build_adjacency_list() {
        let expected = HashMap::from([(1, vec![2, 3]), (2, vec![3])]);
        assert_eq!(
            build_adjacency_list(3, vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
            expected
        );
    }

    #[test]
    fn test_find_order_no_courses() {}

    #[test]
    fn test_find_order_impossible_completion() {}

    #[test]
    fn test_find_order_possible_completion() {}
}
