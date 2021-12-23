// A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).

// The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).

// How many possible unique paths are there?

#[allow(dead_code)]
pub fn unique_paths(n: i32, m: i32) -> i32 {
    if n < 1 || m < 1 {
        return 0
    }

    // preallocate array
    let mut paths = vec![vec![0; (m + 1) as usize]; (n + 1) as usize];
    // set base case
    paths[1][1] = 1;
    for i in 0..=n as usize {
        for j in 0..=m as usize {
            // add to the right
            if i + 1 <= n as usize {
                paths[i + 1][j] = paths[i][j] + paths[i+1][j];
            }
            // add down
            if j + 1 <= m as usize{
                paths[i][j + 1] = paths[i][j] + paths[i][j + 1];
            }

        }
    }

    paths[n as usize][m as usize]
}

#[cfg(test)]
pub mod test {
    use super::unique_paths;

    #[test]
    fn test_unique_paths() {
        assert_eq!(unique_paths(3, 7), 28);
        assert_eq!(unique_paths(3, 4), 10);
    }
}
