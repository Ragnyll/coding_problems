fn gcd(x: i32, y: i32) -> i32 {
    if y == 0 {
        return x
    }
    return gcd(y, x % y)
}


#[cfg(test)]
mod test {
    use super::gcd;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(127, (127 as i32).pow(2)), 127);
    }
}
