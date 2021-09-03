use std::collections::HashMap;

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



// This wont compile because you could be doing multiple inserts. so multiple borrows will occur
// #[allow(dead_code)]
// fn fib_memo(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    // match memo.get(&n) {
        // Some(val) => return *val,
        // None => {
            // match n {
                // 0 | 1 | 2 => return 1,
                // _ => {
                    // memo.insert(n, fib_memo(n -1, memo) + fib_memo(n -2, memo));
                    // return *memo.get(&n).unwrap();
                // }
            // }
        // }

    // }
// }



#[allow(dead_code)]
fn favorite_fib(n: usize) -> usize {
    fn fib_memoization(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
        if let Some(v) = memo.get(&n) {
            return *v;
        }

        let v = match n {
            0 | 1 | 2 => 1,
            _ => fib_memoization(n-2, memo) + fib_memoization(n-1, memo),
        };

        memo.insert(n, v);
        v
    }

    let mut memo = HashMap::new();
    fib_memoization(n, &mut memo)
}

#[cfg(test)]
pub mod test {
    use super::{fib_recursive, fib, favorite_fib};

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
    fn fib_memoization_base_case() {
        assert_eq!(favorite_fib(1), 1);
        assert_eq!(favorite_fib(2), 1);
    }

    #[test]
    fn fib_memoizatoin_non_base_case() {
        assert_eq!(favorite_fib(5), 5);
        assert_eq!(favorite_fib(50), 12586269025);
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
