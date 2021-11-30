// Given an integer n, return an array ans of length n + 1 such that for each i (0 <= i <= n), ans[i] is the number of 1's in the binary representation of i.

pub fn count_bits(n: i32) -> Vec<i32> {
    (0..=n).into_iter().map(|i| i.count_ones() as i32).collect()
}

#[cfg(test)]
mod test {
    use super::count_bits;

    #[test]
    fn leetcode_tests() {
        count_bits(5);
    }
}
