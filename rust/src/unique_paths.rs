// A robot is located at the top-left corner of a m x n grid (marked 'Start' in the diagram below).

// The robot can only move either down or right at any point in time. The robot is trying to reach the bottom-right corner of the grid (marked 'Finish' in the diagram below).

// How many possible unique paths are there?

#[allow(dead_code)]
fn unique_paths(m: i32, n: i32) -> i32 {
    // initialzie array
    let mut paths = vec![vec![0; (n + 1) as usize]; (m + 1) as usize];
    paths[1][1] = 1;

    for i in 0..((m + 1) as usize) {
        for j in 0..((n + 1) as usize) {
            // add the val in the square to the right and below
            if i + 1 <= m as usize {
                paths[i + 1][j] = paths[i + 1][j] + paths[i][j];
            }
            if j + 1 <= n as usize {
                paths[i][j + 1] = paths[i][j + 1] + paths[i][j];
            }
        }
    }

    return paths[m as usize][n as usize];
}

#[cfg(test)]
pub mod test {
    use super::unique_paths;

    #[test]
    fn test_unique_paths() {
        assert_eq!(unique_paths(3, 7), 28);
    }
}
