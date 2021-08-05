
mod topic_1736;

#[cfg(test)]
mod tests {

    use crate::topic_1736::maximum_time;

    #[test]
    fn solution_1736() {
        println!("{}", maximum_time("2?:?0".to_string()));
        println!("{}", maximum_time("0?:3?".to_string()));
        println!("{}", maximum_time("1?:22".to_string()));
    }
}
