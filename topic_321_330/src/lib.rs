mod topic_322;
mod topic_326;

#[cfg(test)]
mod tests {
    use crate::topic_322::coin_change;
    use crate::topic_326::is_power_of_three;

    #[test]
    fn solution_322() {
        println!("{}", coin_change(vec![1, 2, 5], 11));
        println!("{}", coin_change(vec![1], 0));
    }

    #[test]
    fn solution_326() {
        println!("{}", is_power_of_three(27));
        println!("{}", is_power_of_three(0));
        println!("{}", is_power_of_three(9));
        println!("{}", is_power_of_three(45));
    }
}
