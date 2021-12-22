// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.

// You can return the answer in any order.

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut num_to_ind: HashMap<i32, usize> = HashMap::new();
    for i in 0..nums.len() {
        let compliment = target - nums[i];
        if num_to_ind.contains_key(&compliment) {
            return vec![i as i32, *num_to_ind.get(&compliment).unwrap() as i32];
        }
        num_to_ind.insert(nums[i], i);
    }

    vec![]
}

#[cfg(test)]
pub mod test {
    use super::two_sum;

    #[test]
    fn test_two_sum_pair_exists() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 9), vec![1, 0]);
        assert_eq!(two_sum(vec![3, 2, 4], 6), vec![2, 1]);
        assert_eq!(two_sum(vec![3, 3], 6), vec![1, 0]);
    }

    #[test]
    fn test_two_sum_pair_dne() {
        assert_eq!(two_sum(vec![2, 7, 11, 15], 1), vec![]);
    }
}
