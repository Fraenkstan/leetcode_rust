use std::cmp::max;

pub fn max_profit(prices: Vec<i32>) -> i32 {
    match prices.len()
    {
        0 | 1 => 0,
        2 => max(0, prices[1] - prices[0]),
        _ =>
            {
                let mut buy = max(-prices[0], -prices[1]);
                let mut sell0 = 0;
                let mut sell1 = max(sell0, prices[1] + buy);

                for v in prices.into_iter().skip(2)
                {
                    buy = max(buy, sell0 - v);
                    sell0 = sell1;
                    sell1 = max(sell0, buy + v);
                }

                sell1
            },
    }
}