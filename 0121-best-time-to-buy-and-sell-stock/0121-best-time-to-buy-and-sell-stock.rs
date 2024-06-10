impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
         if prices.is_empty() {
            return 0;
        }

        let mut min_price = prices[0];
        let mut max_profit = 0;

        for &i in prices.iter() {
            if i < min_price {
                min_price = i;
            } else {
                max_profit = std::cmp::max(max_profit, i - min_price);
            }
        }

        max_profit
    }
        
    
}