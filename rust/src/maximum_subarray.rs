// Given an integer array nums, find the contiguous subarray (containing at least one number) which has the largest sum and return its sum.

// A subarray is a contiguous part of an array.

#[allow(dead_code)]
fn maximum_subarray(arr: &[i32]) -> i32 {
    let mut res = i32::MIN;
    let mut cur_sum = 0;

    for num in arr {
        cur_sum = std::cmp::max(cur_sum + num, *num);
        res = std::cmp::max(res, cur_sum);
    }

    res
}

#[cfg(test)]
pub mod test {
    use super::maximum_subarray;

    #[test]
    fn ex1() {
        //Input: nums = [-2,1,-3,4,-1,2,1,-5,4]
        //Output: 6
        //Explanation: [4,-1,2,1] has the largest sum = 6.
        assert_eq!(6, maximum_subarray(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]));
    }

    #[test]
    fn ex2() {
        // Input: nums = [1]
        // Output: 1
        assert_eq!(1, maximum_subarray(&[1]));
    }

    #[test]
    fn ex3() {
        // Input: nums = [5,4,-1,7,8]
        // Output: 23
        assert_eq!(23, maximum_subarray(&[5, 4, -1, 7, 8]));
    }
}
