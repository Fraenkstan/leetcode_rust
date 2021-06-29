
pub fn max_profit(prices: Vec<i32>) -> i32 {
    if prices.len() <= 1 {
        return 0;
    }
    let mut min = prices[0];
    let mut max = 0;
    for v in prices {
        max = max.max(v - min);
        min = min.min(v);
    }
    max
}