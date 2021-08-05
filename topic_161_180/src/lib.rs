
mod topic_162;
mod topic_165;
mod topic_166;
mod topic_167;
mod topic_168;
mod topic_169;
mod topic_171;
mod topic_172;
mod topic_173;
mod topic_179;

#[cfg(test)]
mod tests {

    use crate::topic_162::find_peak_element;
    use crate::topic_165::compare_version;
    use crate::topic_166::fraction_to_decimal;
    use crate::topic_167::two_sum;
    use crate::topic_168::convert_to_title;
    use crate::topic_169::majority_element;
    use crate::topic_171::title_to_number;
    use crate::topic_172::trailing_zeroes;
    use crate::topic_179::largest_number;

    #[test]
    fn solution_162() {
        println!("{}", find_peak_element(vec![1,2,3,1]));
        println!("{}", find_peak_element(vec![1,2,1,3,5,6,4]));
    }

    #[test]
    fn solution_165() {
        println!("{}", compare_version("1.01".to_string(), "1.001".to_string()));
        println!("{}", compare_version("1.0".to_string(), "1.0.0".to_string()));
        println!("{}", compare_version("0.1".to_string(), "1.1".to_string()));
        println!("{}", compare_version("1.0.1".to_string(), "1".to_string()));
        println!("{}", compare_version("7.5.2.4".to_string(), "7.5.3".to_string()));
    }

    #[test]
    fn solution_166() {
        println!("{}", fraction_to_decimal(1, 2));
        println!("{}", fraction_to_decimal(2, 1));
        println!("{}", fraction_to_decimal(2, 3));
        println!("{}", fraction_to_decimal(4, 333));
        println!("{}", fraction_to_decimal(1, 5));
    }

    #[test]
    fn solution_167() {
        println!("{:?}", two_sum(vec![2,7,11,15], 9));
        println!("{:?}", two_sum(vec![2,3,4], 6));
        println!("{:?}", two_sum(vec![-1,0], -1));
    }

    #[test]
    fn solution_168() {
        println!("{}", convert_to_title(1));
        println!("{}", convert_to_title(28));
        println!("{}", convert_to_title(701));
    }

    #[test]
    fn solution_169() {
        println!("{}", majority_element(vec![3,2,3]));
        println!("{}", majority_element(vec![2,2,1,1,1,2,2]));
    }

    #[test]
    fn solution_171() {
        println!("{}", title_to_number("A".to_string()));
        println!("{}", title_to_number("AB".to_string()));
        println!("{}", title_to_number("ZY".to_string()));
    }

    #[test]
    fn solution_172() {
        println!("{}", trailing_zeroes(3));
        println!("{}", trailing_zeroes(5));
    }

    #[test]
    fn solution_179() {
        println!("{}", largest_number(vec![10,2]));
        println!("{}", largest_number(vec![3,30,34,5,9]));
        println!("{}", largest_number(vec![1]));
        println!("{}", largest_number(vec![10]));
    }
}
