// A message containing letters from A-Z can be encoded into numbers using the following mapping:
//
// 'A' -> "1"
// 'B' -> "2"
// ...
// 'Z' -> "26"
//
// To decode an encoded message, all the digits must be grouped then mapped back into letters using the reverse of the mapping above (there may be multiple ways). For example, "11106" can be mapped into:
//
//     "AAJF" with the grouping (1 1 10 6)
//     "KJF" with the grouping (11 10 6)
//
// Note that the grouping (1 11 06) is invalid because "06" cannot be mapped into 'F' since "6" is different from "06".
//
// Given a string s containing only digits, return the number of ways to decode it.
//
// The answer is guaranteed to fit in a 32-bit integer.

#[allow(dead_code)]
fn decode_ways(str_to_decode: &str) -> i32 {
    let mut decode_ways = 0;

    let mut start_char = 0;
    while start_char < str_to_decode.len() {
        let mut valid = true;
        let mut lookahead = 1;
        while valid {
            if start_char + lookahead > str_to_decode.len() {
                break;
            }

            let substr = String::from(&str_to_decode[start_char..(start_char + lookahead)]);
            let digit = substr.parse::<u16>().unwrap_or(0);

            if digit > 0 && digit <= 26 {
                decode_ways += 1;
                lookahead += 1;
            } else {
                valid = false;
                lookahead = 1;
            }
        }
        start_char += 1;
    }
    decode_ways
}

#[cfg(test)]
pub mod test {
    use super::decode_ways;

    #[test]
    fn test_decode_ways1() {
        assert_eq!(0, decode_ways("0"));
    }

    #[test]
    fn test_decode_ways2() {
        assert_eq!(1, decode_ways("06"));
    }

    #[test]
    fn test_decode_ways3() {
        assert_eq!(3, decode_ways("12"));
    }

    #[test]
    fn test_decode_ways4() {
        assert_eq!(5, decode_ways("226"));
    }

    #[test]
    fn test_decode_ways5() {
        assert_eq!(0, decode_ways("0"));
    }
}
