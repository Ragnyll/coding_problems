// You are climbing a staircase. It takes n steps to reach the top.
// Each time you can either climb 1 or 2 steps. In how many distinct ways can you climb to the top?

#[allow(dead_code)]
fn climbing_stairs(n: usize) -> usize {
    // base case 1
    let mut stairs = vec![0; n];
    stairs[0] = 1;
    if n > 1 {
        stairs[1] = 2;
        let mut current_step = 2;

        while current_step < stairs.len() {
            stairs[current_step] = stairs[current_step - 1] + stairs[current_step - 2];
            current_step +=1;
        }
    }

    return stairs[n - 1];
}

#[cfg(test)]
mod test {
    use super::climbing_stairs;

    #[test]
    fn test1() {
        assert_eq!(1, climbing_stairs(1));
    }

    #[test]
    fn test2() {
        assert_eq!(2, climbing_stairs(2));
    }

    #[test]
    fn test3() {
        assert_eq!(3, climbing_stairs(3));
    }

    #[test]
    fn test4() {
        assert_eq!(5, climbing_stairs(4));
    }
}
