mod topic_1818;
mod topic_1833;
mod topic_1838;
mod topic_1846;
mod topic_1877;
mod topic_1893;

#[cfg(test)]
mod tests {

    use crate::topic_1818::min_absolute_sum_diff;
    use crate::topic_1833::max_ice_cream;
    use crate::topic_1838::max_frequency;
    use crate::topic_1846::maximum_element_after_decrementing_and_rearranging;
    use crate::topic_1877::min_pair_sum;
    use crate::topic_1893::is_covered;

    #[test]
    fn solution_1818() {
        println!("{}", min_absolute_sum_diff(vec![1, 7, 5], vec![2, 3, 5]));
        println!(
            "{}",
            min_absolute_sum_diff(vec![2, 4, 6, 8, 10], vec![2, 4, 6, 8, 10])
        );
        println!(
            "{}",
            min_absolute_sum_diff(vec![1, 10, 4, 4, 2, 7], vec![9, 3, 5, 1, 7, 4])
        );
    }

    #[test]
    fn solution_1833() {
        println!("{}", max_ice_cream(vec![1, 3, 2, 4, 1], 7));
        println!("{}", max_ice_cream(vec![10, 6, 8, 7, 7, 8], 5));
        println!("{}", max_ice_cream(vec![1, 6, 3, 1, 2, 5], 20));
    }

    #[test]
    fn solution_1838() {
        println!("{}", max_frequency(vec![1, 2, 4], 5));
        println!("{}", max_frequency(vec![1, 4, 8, 13], 3));
        println!("{}", max_frequency(vec![3, 9, 6], 2));
    }

    #[test]
    fn solution_1846() {
        println!(
            "{}",
            maximum_element_after_decrementing_and_rearranging(vec![2, 2, 1, 2, 1])
        );
        println!(
            "{}",
            maximum_element_after_decrementing_and_rearranging(vec![100, 1, 1000])
        );
    }

    #[test]
    fn solution_1877() {
        println!("{}", min_pair_sum(vec![3, 5, 2, 3]));
        println!("{}", min_pair_sum(vec![3, 5, 4, 2, 4, 6]));
    }

    #[test]
    fn solution_1893() {
        println!(
            "{}",
            is_covered(vec![vec![1, 2], vec![3, 4], vec![5, 6]], 2, 5)
        );
        println!("{}", is_covered(vec![vec![1, 10], vec![10, 20]], 21, 21));
    }
}
