// You are a professional robber planning to rob houses along a street. Each house has a certain amount of money stashed, the only constraint stopping you from robbing each of them is that adjacent houses have security systems connected and it will automatically contact the police if two adjacent houses were broken into on the same night.

// Given an integer array nums representing the amount of money of each house, return the maximum amount of money you can rob tonight without alerting the police

pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.is_empty() {
        return 0;
    }

    let max_size = nums.len();
    let mut best_loot = vec![0; max_size + 1];

    // initial state
    best_loot[max_size] = 0;
    best_loot[max_size - 1] = nums[max_size - 1];

    for i in (0..(max_size - 1)).rev() {
        best_loot[i] = std::cmp::max(best_loot[i + 1], best_loot[i + 2] + nums[i]);
    }

    best_loot[0]
}

#[cfg(test)]
mod test {
    use super::rob;

    #[test]
    fn test_rob() {
        assert_eq!(rob(vec![1, 2, 3, 4]), 6);
    }

    #[test]
    fn test_rob_2() {
        assert_eq!(rob(vec![1, 2, 3, 1]), 4);
    }
}
