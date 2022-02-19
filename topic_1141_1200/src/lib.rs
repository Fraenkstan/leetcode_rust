
mod topic_1189;

#[cfg(test)]
mod tests {
    use crate::topic_1189::max_number_of_balloons;

    #[test]
    fn solution_1189() {
        println!("{}", max_number_of_balloons("nlaebolko".to_string()));
        println!("{}", max_number_of_balloons("loonbalxballpoon".to_string()));
        println!("{}", max_number_of_balloons("leetcode".to_string()));
    }
}
