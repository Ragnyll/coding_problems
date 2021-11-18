// https://leetcode.com/problems/search-in-rotated-sorted-array/

#[allow(dead_code)]
fn binary_search(nums: &[i32], target: i32) -> i32 {
    if nums.len() == 0 {
        return -1;
    }
    let mut start_i = 0;
    let mut end_i = nums.len() - 1;

    while start_i <= end_i {
        let mid = start_i + (end_i - start_i) / 2;
        if nums[mid] == target {
            return mid as i32;
        } else if nums[mid] >= nums[start_i] {
            if target >= nums[start_i] && target < nums[mid] {
                end_i = mid - 1;
            } else {
                start_i = mid + 1;
            }
        } else {
            if target <= nums[end_i] && target > nums[mid] {
                start_i = mid + 1;
            } else {
                end_i = mid - 1;
            }
        }
    }
    -1
}

#[cfg(test)]
mod test {
    use super::binary_search;

    #[test]
    fn test_binary_search_target_exists() {
        assert_eq!(binary_search(&vec![4, 5, 6, 7, 0, 1, 2], 0), 4);
        assert_eq!(binary_search(&vec![4, 5, 6, 7, 8, 0, 1, 2], 0), 5);
    }

    #[test]
    fn test_binary_search_target_exists_small_array() {
        assert_eq!(binary_search(&vec![4, 5], 4), 0);
        assert_eq!(binary_search(&vec![4, 5], 5), 1);
    }

    #[test]
    fn test_binary_search_target_dne_small_array() {
        assert_eq!(binary_search(&vec![4, 5], 0), -1);
    }

    #[test]
    fn test_binary_search_target_exists_single_element_array() {
        assert_eq!(binary_search(&vec![4], 4), 0);
    }

    #[test]
    fn test_binary_search_target_dne_single_element_array() {
        assert_eq!(binary_search(&vec![4], 0), -1);
    }

    #[test]
    fn test_binary_search_empty_array() {
        assert_eq!(binary_search(&vec![], 0), -1);
    }
}
