/// given a target number n and an array a determine how it is possible to sum any number of elements
/// from a to obtain target n
/// elements are not eliminated from the array a
use std::collections::HashMap;

#[allow(dead_code)]
fn how_sum_r(target: i32, a: &[i32]) -> Option<Vec<i32>> {
    if target < 0 {
        return None;
    }

    match target {
        0 => Some(vec![]),
        _ => {
            for n in a {
                let remainder = target - n;
                let remainder_result = how_sum_r(remainder, a);
                match remainder_result {
                    Some(_) => {
                        // This is def a case where i would want to use Box. Otherwise my space
                        // complexity is bad
                        let mut new_remainder_result = remainder_result.unwrap().clone();
                        new_remainder_result.push(*n);
                        return Some(new_remainder_result.to_vec());
                    }
                    None => (),
                }
            }
            None
        }
    }
}

#[cfg(test)]
pub mod test {
    use super::{how_sum_r};

    #[test]
    fn how_sum_true() {
        assert_eq!(how_sum_r(7, &[4, 3, 5, 7]).unwrap(), vec![3, 4]);
        assert_eq!(how_sum_r(7, &[7]).unwrap(), vec![7]);
        assert_eq!(how_sum_r(0, &[1, 2, 3]).unwrap(), vec![]);
    }

    #[test]
    fn how_sum_false() {
        assert_eq!(how_sum_r(7, &[]), None);
        assert_eq!(how_sum_r(7, &[2]), None);
    }
}
