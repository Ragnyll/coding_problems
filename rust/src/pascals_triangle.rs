#[allow(dead_code)]
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    if num_rows == 0 {
        return vec![vec![]];
    }

    let mut triangle: Vec<Vec<i32>> = vec![];
    for i in 0..num_rows {
        // first and last element of row are 1. just make everything 1 then loop between the
        // first and last elms
        let mut row = vec![1; i as usize + 1];
        for j in 1..(row.len() - 1) {
            row[j] = triangle[(i as usize) - 1][j - 1 as usize]
                + triangle[(i as usize) - 1][(j as usize)];
        }

        triangle.push(row);
    }

    triangle
}

/// Given an integer rowIndex, return the rowIndexth (0-indexed) row of the Pascal's triangle.
#[allow(dead_code)]
pub fn get_row(row_index: i32) -> Vec<i32> {
    generate(row_index + 1).last().unwrap().to_vec()
}

#[cfg(test)]
mod test {
    use super::{generate, get_row};

    #[test]
    fn test_pascals_triangle() {
        assert_eq!(
            generate(5),
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        )
    }

    #[test]
    fn test_pascals_triangle_1() {
        assert_eq!(generate(1), vec![vec![1],])
    }

    #[test]
    fn test_pascals_triangle_2() {
        assert_eq!(generate(2), vec![vec![1], vec![1, 1]])
    }

    #[test]
    fn test_get_row() {
        assert_eq!(get_row(3), vec![1, 3, 3, 1])
    }
}
