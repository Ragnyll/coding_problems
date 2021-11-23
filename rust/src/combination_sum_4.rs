// Given an array of distinct integers nums and a target integer target, return the number of possible combinations that add up to target.

// The answer is guaranteed to fit in a 32-bit integer.

pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
    let mut possible_sums = vec![0; target as usize + 1];
    // set initial state
    for num in &nums {
        if (*num as usize) < possible_sums.len() {
            possible_sums[*num as usize] = 1;
        }
    }

    for possible_sum_index in 0..(possible_sums.len() - 1) {
        // look ahead by each number in the nums vector
        for num in &nums {
            if possible_sum_index + (*num as usize) < possible_sums.len() {
                possible_sums[possible_sum_index + (*num as usize)] +=
                    possible_sums[possible_sum_index];
            }
        }
    }

    possible_sums[possible_sums.len() - 1]
}

#[cfg(test)]
pub mod test {
    use super::combination_sum4;

    #[test]
    fn test_combination_sum() {
        assert_eq!(combination_sum4(vec![1, 2, 3], 4), 7);
    }

    #[test]
    fn test_combination_sum_target_smaller_than_elements_in_list() {
        assert_eq!(combination_sum4(vec![9], 3), 0);
    }
}
