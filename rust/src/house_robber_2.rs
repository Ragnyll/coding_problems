pub fn rob(nums: Vec<i32>) -> i32 {
    if nums.len() == 0 {
        return 0;
    }

    if nums.len() == 1 {
        return nums[0];
    }

    let max_from_0 = rob_linear(&nums[0..(nums.len() - 1)]);
    let max_from_1 = rob_linear(&nums[1..(nums.len())]);

    return std::cmp::max(max_from_0, max_from_1);
}

fn rob_linear(nums: &[i32]) -> i32 {
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
        rob(vec![1, 2, 3, 4, 5]);
        assert_eq!(1, 2);
    }
}
