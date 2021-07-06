
mod topic_1418;

#[cfg(test)]
mod tests {

    use crate::topic_1418::display_table;

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
