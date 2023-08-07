struct Solution;
impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {

        let mut profit = 0;
        let mut buy_in = i32::MAX;
        for price in prices {
            profit = std::cmp::max(profit, price - buy_in);
            buy_in = std::cmp::min(buy_in, price);
        }

        profit
    }
}

#[cfg(test)]
mod _121 {
    use crate::_121_best_time_to_buy_and_sell_stock::Solution;
    #[test]
    fn case1() {
        assert_eq!(Solution::max_profit(vec![7,1,5,3,6,4]), 5);
    }
    #[test]
    fn case2() {
        assert_eq!(Solution::max_profit(vec![7,6,4,3,2,1]), 0);
    }
}