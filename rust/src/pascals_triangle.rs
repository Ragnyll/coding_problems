#[allow(dead_code)]
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    vec![vec![]]
}

#[cfg(test)]
mod test {
    use super::generate;

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
}
