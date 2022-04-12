pub fn max_ice_cream(costs: Vec<i32>, coins: i32) -> i32 {
    let mut costs = costs.clone();
    let mut coins = coins;
    costs.sort();
    costs
        .iter()
        .take_while(|&cost| {
            coins -= cost;
            coins >= 0
        })
        .count() as i32
}
