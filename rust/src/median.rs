pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
    // bad on memory but i was passed an owned type and not a ref so thats on them
    // Also this would be faster if implemnted with a min heap then using that to merge
    let mut my_nums: Vec<i32> = nums1.clone();
    my_nums.append(&mut nums2.clone());
    my_nums.sort_unstable();

    let n = my_nums.len();
    if n % 2 == 0 {
        return (my_nums[n / 2] + my_nums[n / 2 - 1]) as f64 / 2.0;
    }
    my_nums[n / 2] as f64
}

#[cfg(test)]
mod test {
    #[test]
    fn test_median_sorted_arrays() {}
}
