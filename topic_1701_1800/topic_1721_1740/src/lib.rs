
mod topic_1725;
mod topic_1736;

#[cfg(test)]
mod tests {
    use crate::topic_1725::count_good_rectangles;
    use crate::topic_1736::maximum_time;

    #[test]
    fn solution_1725() {
        println!("{}", count_good_rectangles(vec![vec![5,8], vec![3,9], vec![5,12], vec![16,5]]));
        println!("{}", count_good_rectangles(vec![vec![2,3], vec![3,7], vec![4,3], vec![3,7]]));
    }

    #[test]
    fn solution_1736() {
        println!("{}", maximum_time("2?:?0".to_string()));
        println!("{}", maximum_time("0?:3?".to_string()));
        println!("{}", maximum_time("1?:22".to_string()));
    }
}
