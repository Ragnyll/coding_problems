// You are given an integer array coins representing coins of different denominations and an integer amount representing a total amount of money.
// Return the fewest number of coins that you need to make up that amount. If that amount of money cannot be made up by any combination of the coins, return -1.
// You may assume that you have an infinite number of each kind of coin.
use num_traits::PrimInt;
use std::fmt::Debug;

#[allow(dead_code)]
fn coin_change<T: PrimInt + Debug>(target: T, coins: &[T]) -> usize {
    // some shorthand utilitieis
    // use the coin value as a pointer for lookups
    let coin_as_pointer = |coin: &T| coin.to_usize().unwrap();

    let mut coin_combos: Vec<Vec<Vec<T>>> = vec![vec![]; target.to_usize().unwrap() + 1];

    // seed the  initial values
    coin_combos[0].push(vec![]); // base case
    for coin in coins {
        if coin_as_pointer(coin) < coin_combos.len() {
            coin_combos[coin_as_pointer(coin)].push(vec![*coin]);
        }
    }

    let mut i = 0;
    while i < coin_combos.len() {
        if !coin_combos[i].is_empty() {
            for coin in coins {
                let coin_val = coin_as_pointer(coin);

                if i + coin_val < coin_combos.len() {
                    for possible_coin_set in &coin_combos[i].clone() {
                        let mut append_me = possible_coin_set.clone();
                        append_me.push(*coin);

                        &coin_combos[i + coin_val].push(append_me);
                    }

                }
            }
        }
        i += 1;
    }

    // The head will always be the shortest
    return coin_combos[coin_as_pointer(&target)][0].len();
}

#[cfg(test)]
mod test {
    use super::coin_change;
    use num_traits::ToPrimitive;

    #[test]
    fn test_coin_change() {
        assert_eq!(3, coin_change(12, &[1, 5, 10]).to_i32().unwrap());
        assert_eq!(1, coin_change(5, &[1, 5, 10]).to_i32().unwrap());
        assert_eq!(0, coin_change(0, &[1, 5, 10]).to_i32().unwrap());
    }
}
