// Given an m x n binary matrix filled with 0's and 1's, find the largest square containing only 1's and return its area.

#[allow(dead_code)]
pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    let dp_h = matrix.len() + 1;
    let dp_w = matrix[0].len() + 1;
    let mut dp: Vec<Vec<i32>> = vec![vec![0; dp_w]; dp_h];

    let mut largest = 0;
    for i in 1..dp_h {
        for j in 1..dp_w {
            // if the matrix has a 1 check all the neighbors on dp
            if matrix[i - 1][j - 1] == '1' {
                //               left          left          diagonal
                dp[i][j] = *vec![dp[i - 1][j], dp[i][j - 1], dp[i - 1][j - 1]]
                    .iter()
                    .min()
                    .unwrap() + 1;

                // save the largest square
                largest = std::cmp::max(dp[i][j], largest);
            }
        }
    }

    largest * largest
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
            4
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
            9
        );

        assert_eq!(
            maximal_square(vec![
                vec!['1', '1', '1', '0'],
                vec!['1', '1', '1', '0'],
                vec!['1', '1', '1', '0'],
                vec!['0', '0', '0', '0'],
            ]),
            9
        );
    }
}
