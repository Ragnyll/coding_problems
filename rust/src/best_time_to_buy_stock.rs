#[allow(dead_code)]
fn best_time_to_buy_stock(stock_prices: &[usize]) -> usize {
    let mut max_profit = 0;
    let mut min_buy_price = usize::MAX;

    for price in stock_prices.into_iter() {
        if price < &min_buy_price {
            min_buy_price = *price;
        } else if max_profit < price - min_buy_price {
            max_profit = *price - min_buy_price;
        }
    }

    max_profit
}

#[cfg(test)]
pub mod test {
    use super::{best_time_to_buy_stock};

    #[test]
    fn max_profit_0() {
        assert_eq!(best_time_to_buy_stock(&vec![]), 0);
        assert_eq!(best_time_to_buy_stock(&vec![5, 4, 3, 2, 1]), 0);
    }

    #[test]
    fn max_profit_not_0() {
        assert_eq!(best_time_to_buy_stock(&vec![1, 2, 3, 4]), 3);
    }
}
