#[allow(dead_code)]
fn fib_recursive(n: usize) -> usize {
    if n < 3 {
        return 1;
    }
    fib_recursive(n - 1) + fib_recursive(n - 2)
}

#[allow(dead_code)]
fn fib(number: usize) -> usize {
    fn fib_memo(n: usize, memo: &mut [Option<usize>]) -> usize {
        memo[n].unwrap_or_else(|| {
            let result = {
                if n > 2 {
                    fib_memo(n - 1, memo) + fib_memo(n - 2, memo)
                } else {
                    1
                }
            };
            memo[n] = Some(result);
            result
        })
    }

    fib_memo(number, &mut vec![None; number + 1])
}

#[cfg(test)]
pub mod test {
    use super::{fib_recursive, fib};

    #[test]
    fn fib_recursive_base_case() {
        assert_eq!(fib_recursive(1), 1);
        assert_eq!(fib_recursive(2), 1);
    }

    #[test]
    fn fib_recursive_non_base_case() {
        assert_eq!(fib_recursive(5), 5);
        // This case will take a long time with a pure recursive solution
        // assert_eq!(fib_recursive(50), 12586269025);
    }

    #[test]
    fn fib_memo_base_case() {
        assert_eq!(fib(1), 1);
        assert_eq!(fib(2), 1);
    }

    #[test]
    fn fib_memo_non_base_case() {
        assert_eq!(fib(5), 5);
        assert_eq!(fib(50), 12586269025);
    }
}
