impl Solution {
    pub fn max_profit(prices: Vec<i32>) -> i32 {
         if prices.is_empty() {
            return 0;
        }

        let mut min_price = prices[0];
        let mut max_profit = 0;

        for price in prices.iter() {
            if *price < min_price {
                min_price = *price;
            } else {
                max_profit = std::cmp::max(max_profit, price - min_price);
            }
        }

        max_profit
    }
        
    
}