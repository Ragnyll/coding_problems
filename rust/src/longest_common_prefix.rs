#[allow(dead_code)]
pub fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::from("");
    }

    // git the shortest str
    let shortest_str: usize = strs.iter().map(|x| x.len()).min().unwrap();

    let mut longest_pre = String::from("");
    for i in 0..shortest_str + 1 {
        let pre = &strs[0][0..i];
        if strs.iter().all(|s| s.starts_with(pre)) {
            longest_pre = String::from(pre);
        }
    }

    longest_pre
}

#[cfg(test)]
mod test {
    use super::longest_common_prefix;

    #[test]
    fn test_empty_string() {
        assert_eq!(
            longest_common_prefix(vec![String::from(""), String::from("")]),
            String::from("")
        )
    }

    #[test]
    fn test_no_common_prefix() {
        assert_eq!(
            longest_common_prefix(vec![String::from("ab"), String::from("dc")]),
            longest_common_prefix(vec![
                String::from("c"),
                String::from("acc"),
                String::from("ccc")
            ]),
        )
    }

    #[test]
    fn test_common_prefix_1() {
        assert_eq!(
            longest_common_prefix(vec![String::from("ab"), String::from("ac")]),
            String::from("a")
        )
    }

    #[test]
    fn test_common_prefix_n() {
        assert_eq!(
            longest_common_prefix(vec![
                String::from("flower"),
                String::from("flow"),
                String::from("flight")
            ]),
            String::from("fl")
        )
    }
}
