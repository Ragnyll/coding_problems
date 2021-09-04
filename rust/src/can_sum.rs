/// given a target number n and an array a determine if it is possible to sum any number of elements
/// from a to obtain target n
/// elements are not eliminated from the array a
use std::collections::HashMap;

#[allow(dead_code)]
fn can_sum_r(target: i32, a: &[i32]) -> bool {
    if target < 0 {
        return false;
    }
    match target {
        0 => true,
        _ => {
            for n in a {
                let remainder = target - n;
                if can_sum_r(remainder, a) {
                    return true;
                }
            }
            false
        }
    }
}

#[allow(dead_code)]
fn can_sum(target: i32, a: &[i32]) -> bool {
    let mut memo = HashMap::new();

    fn can_sum_memo(target: i32, a: &[i32], memo: &mut HashMap<i32, bool>) -> bool {
        if memo.contains_key(&target) {
            return *memo.get(&target).unwrap();
        }

        if target < 0 {
            return false;
        }

        match target {
            0 => true,
            _ => {
                for n in a {
                    let remainder = target - n;
                    if can_sum_memo(remainder, a, memo) {
                        memo.insert(target, true);
                        return true;
                    }
                }
                memo.insert(target, false);
                false
            }
        }
    }

    can_sum_memo(target, a, &mut memo)
}

#[cfg(test)]
pub mod test {
    use super::{can_sum_r, can_sum};

    #[test]
    fn can_sum_true() {
        assert_eq!(can_sum_r(7, &[4, 3, 5, 7]), true);
        assert_eq!(can_sum_r(7, &[7]), true);

        assert_eq!(can_sum(7, &[4, 3, 5, 7]), true);
        assert_eq!(can_sum(7, &[7]), true);
    }

    #[test]
    fn can_sum_false() {
        assert_eq!(can_sum_r(7, &[]), false);
        assert_eq!(can_sum_r(7, &[2]), false);

        assert_eq!(can_sum(7, &[]), false);
        assert_eq!(can_sum(7, &[2]), false);
        assert_eq!(can_sum(300, &[7, 14]), false);
    }
}
