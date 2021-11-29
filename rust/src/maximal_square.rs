// Given an m x n binary matrix filled with 0's and 1's, find the largest square containing only 1's and return its area.

#[allow(dead_code)]
pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let h = matrix.len();
    let w = matrix[0].len();
    let mut dp: Vec<Vec<i32>> = vec![vec![0; w]; h];
    0
}

#[cfg(test)]
mod test {
    use super::maximal_square;

    #[test]
    fn max_size_0() {
        assert_eq!(maximal_square(vec![vec![]]), 0);
        assert_eq!(maximal_square(vec![vec!['0']]), 0);
        assert_eq!(
            maximal_square(vec![
                vec!['0', '0', '0'],
                vec!['0', '0', '0'],
                vec!['0', '0', '0'],
            ]),
            0
        );
    }

    #[test]
    fn max_size_1() {
        assert_eq!(maximal_square(vec![vec!['1']]), 1);
        assert_eq!(
            maximal_square(vec![
                vec!['1', '0', '0'],
                vec!['0', '1', '1'],
                vec!['0', '1', '0'],
            ]),
            1
        );
    }

    #[test]
    fn max_size_2() {
        assert_eq!(
            maximal_square(vec![
                vec!['1', '1', '0'],
                vec!['1', '1', '1'],
                vec!['0', '1', '1'],
            ]),
            2
        );
    }

    #[test]
    fn max_size_3() {
        assert_eq!(
            maximal_square(vec![
                vec!['1', '1', '1'],
                vec!['1', '1', '1'],
                vec!['1', '1', '1'],
            ]),
            3
        );

        assert_eq!(
            maximal_square(vec![
                vec!['1', '1', '1', '0'],
                vec!['1', '1', '1', '0'],
                vec!['1', '1', '1', '0'],
                vec!['0', '0', '0', '0'],
            ]),
            3
        );
    }
}
