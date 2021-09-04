/// Given a start grid of size x,y how many ways can the bottom right corner be reached
use std::collections::HashMap;

#[allow(dead_code)]
fn r_travel_recursive(x: usize, y: usize) -> usize {
    if x == 1 && y == 1 {
        return 1;
    }
    if x == 0 || y == 0 {
        return 0;
    }

    r_travel_recursive(x - 1, y) + r_travel_recursive(x, y - 1)
}

#[allow(dead_code)]
fn r_travel(x: usize, y: usize) -> usize {
    let mut memo = HashMap::new();

    fn r_travel_memo(x: usize, y: usize, memo: &mut HashMap<(usize, usize), usize>) -> usize {
        if x == 1 && y == 1 {
            return 1;
        }
        if x == 0 || y == 0 {
            return 0;
        }
        // Could make this more efficient by checking the inverse as well
        let v = match memo.get(&(x, y)) {
            Some(n) => return *n,
            None => r_travel_memo(x - 1, y, memo) + r_travel_memo(x, y - 1, memo),
        };
        memo.insert((x, y), v);

        v
    }

    r_travel_memo(x, y, &mut memo)
}

#[cfg(test)]
pub mod test {
    use super::{r_travel_recursive, r_travel};

    #[test]
    fn r_travel_invalid_size() {
        assert_eq!(r_travel_recursive(0, 0), 0);
        assert_eq!(r_travel_recursive(0, 1), 0);
        assert_eq!(r_travel_recursive(1, 0), 0);

        assert_eq!(r_travel(0, 0), 0);
        assert_eq!(r_travel(0, 1), 0);
        assert_eq!(r_travel(1, 0), 0);
    }

    #[test]
    fn r_travel_size_1_1() {
        assert_eq!(r_travel_recursive(1, 1), 1);

        assert_eq!(r_travel(1, 1), 1);
    }

    #[test]
    fn r_travel_size_3_2() {
        assert_eq!(r_travel_recursive(3, 2), 3);
        assert_eq!(r_travel_recursive(2, 3), 3);

        assert_eq!(r_travel(3, 2), 3);
        assert_eq!(r_travel(2, 3), 3);
        assert_eq!(r_travel(18, 18), 2333606220);
    }
}
