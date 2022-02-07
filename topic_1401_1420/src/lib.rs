
mod topic_1405;
mod topic_1414;
mod topic_1418;

#[cfg(test)]
mod tests {
    use crate::topic_1405::longest_diverse_string;
    use crate::topic_1414::find_min_fibonacci_numbers;
    use crate::topic_1418::display_table;

    #[test]
    fn solution_1405() {
        println!("{}", longest_diverse_string(1,1,7));
        println!("{}", longest_diverse_string(2,2,1));
        println!("{}", longest_diverse_string(7,1,0));
    }

    #[test]
    fn solution_1414() {
        println!("{}", find_min_fibonacci_numbers(7));
        println!("{}", find_min_fibonacci_numbers(10));
        println!("{}", find_min_fibonacci_numbers(19));
    }

    #[test]
    fn solution_1418() {
        println!("{:?}", display_table(vec![
            vec!["David".to_string(), "3".to_string(), "Ceviche".to_string()],
            vec!["Corina".to_string(), "10".to_string(), "Beef Burrito".to_string()],
            vec!["David".to_string(), "3".to_string(), "Fried Chicken".to_string()],
            vec!["Carla".to_string(), "5".to_string(), "Water".to_string()],
            vec!["Carla".to_string(), "5".to_string(), "Ceviche".to_string()],
            vec!["Rous".to_string(), "3".to_string(), "Ceviche".to_string()]]));

        println!("{:?}", display_table(vec![
            vec!["James".to_string(), "12".to_string(), "Fried Chicken".to_string()],
            vec!["Ratesh".to_string(), "12".to_string(), "Fried Chicken".to_string()],
            vec!["Amadeus".to_string(), "12".to_string(), "Fried Chicken".to_string()],
            vec!["Adam".to_string(), "1".to_string(), "Canadian Waffles".to_string()],
            vec!["Brianna".to_string(), "1".to_string(), "Canadian Waffles".to_string()]]));

        println!("{:?}", display_table(vec![
            vec!["Laura".to_string(), "2".to_string(), "Bean Burrito".to_string()],
            vec!["Jhon".to_string(), "2".to_string(), "Beef Burrito".to_string()],
            vec!["Melissa".to_string(), "2".to_string(), "Soda".to_string()]]));
    }
}
