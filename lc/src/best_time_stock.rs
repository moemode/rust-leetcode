pub fn max_profit(prices: Vec<i32>) -> i32 {
    let (profit, _) = max_profit_indices(prices);
    profit
}

pub fn max_profit_indices(prices: Vec<i32>) -> (i32, Option<(usize, usize)>) {
    if prices.len() < 2 {
        return (0, None);
    }
    let mut cheapest_buy_idx = 0;
    let mut buy_price = prices[0];
    let mut max_prof = 0;
    let mut max_prof_buy = 0;
    let mut max_prof_sell = 0;
    for (idx, &price) in prices.iter().enumerate() {
        if price < buy_price {
            buy_price = price;
            cheapest_buy_idx = idx;
        }
        if price - buy_price > max_prof {
            max_prof = price - buy_price;
            max_prof_buy = cheapest_buy_idx;
            max_prof_sell = idx;
        }
    }
    if max_prof > 0 {
        (max_prof, Some((max_prof_buy, max_prof_sell)))
    } else {
        (0, None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let prices = vec![7, 1, 5, 3, 6, 4];
        assert_eq!(max_profit_indices(prices), (5, Some((1, 4))));
    }

    #[test]
    fn test_example_2() {
        let prices = vec![7, 6, 4, 3, 1];
        assert_eq!(max_profit_indices(prices), (0, None));
    }

    #[test]
    fn test_empty() {
        let prices = vec![0];
        assert_eq!(max_profit_indices(prices), (0, None));
    }

    #[test]
    fn test_one_element() {
        let prices = vec![7];
        assert_eq!(max_profit_indices(prices), (0, None));
    }

    #[test]
    fn test_two_no_profit() {
        let prices = vec![7, 0];
        assert_eq!(max_profit_indices(prices), (0, None));
    }

    #[test]
    fn test_tw_profit() {
        let prices = vec![4, 9];
        assert_eq!(max_profit_indices(prices), (5, Some((0, 1))));
    }
}
