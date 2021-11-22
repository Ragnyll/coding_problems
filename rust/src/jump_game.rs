// You are given an integer array nums. You are initially positioned at the array's first index, and each element in the array represents your maximum jump length at that position. Return true if you can reach the last index, or false otherwise.

#[allow(dead_code)]
fn jump_game(nums: Vec<i32>) -> bool {
    let mut last_i = (nums.len() - 1) as i32;
    for i in (0..(nums.len() - 1)).rev() {
        if i as i32 + nums[i] >= last_i {
            last_i = i as i32;
        }
    }
    return last_i == 0;
}

#[cfg(test)]
mod test {
    use super::jump_game;

    #[test]
    fn test_jump_game() {
        assert_eq!(jump_game(vec![2, 3, 1, 1, 4]), true);
    }
}
