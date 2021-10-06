// Given an integer array nums of length n where all the integers of nums are in the range [1, n] and each integer appears once or twice, return an array of all the integers that appears twice.
//
// You must write an algorithm that runs in O(n) time and uses only constant extra space.

#[allow(dead_code)]
fn find_all_duplicates(arr: &[u16]) -> Vec<u16> {
    // reserve a vector with max size of the input value (the len of arr) to fit the constant space
    // requiment
    let mut register = vec![0; arr.len()];

    for num in arr {
        register[(*num as usize) - 1] += 1;
    }

    let mut duplicates = vec![];

    for i in 0..register.len() {
        if register[i] > 1 {
            duplicates.push((i + 1) as u16);
        }
    }

    duplicates
}

#[cfg(test)]
pub mod test {
    use super::find_all_duplicates;

    #[test]
    fn ex_1() {
        // Input: nums = [4,3,2,7,8,2,3,1]
        // Output: [2,3]
        assert_eq!(vec![2, 3], find_all_duplicates(&[4, 3, 2, 7, 8, 2, 3, 1]));
    }

    #[test]
    fn ex_2() {
        // Input: nums = [1,1,2]
        // Output: [1]
        assert_eq!(vec![1], find_all_duplicates(&[1, 1, 2]));
    }

    #[test]
    fn ex_3() {
        // Input: nums = [1]
        // Output: []
        let expected: Vec<u16> = vec![];
        assert_eq!(expected, find_all_duplicates(&[1]));
    }

    #[test]
    fn ex_4() {
        // Input: nums = [1]
        // Output: []
        let expected: Vec<u16> = vec![];
        assert_eq!(expected, find_all_duplicates(&[]));
    }
}
