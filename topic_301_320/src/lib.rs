mod topic_301;
mod topic_303;
mod topic_304;
mod topic_307;
mod topic_309;
mod topic_310;
mod topic_313;

#[cfg(test)]
mod tests {

    use crate::topic_301::remove_invalid_parentheses;
    use crate::topic_307::SegmentTree;
    use crate::topic_309::max_profit;
    use crate::topic_310::find_min_height_trees;
    use crate::topic_313::nth_super_ugly_number;

    #[test]
    fn solution_301() {
        println!("{:?}", remove_invalid_parentheses("()())()".to_string()));
        println!("{:?}", remove_invalid_parentheses("(a)())()".to_string()));
        println!("{:?}", remove_invalid_parentheses(")(".to_string()));
    }

    #[test]
    fn solution_307() {
        let mut num_array = SegmentTree::new(vec![1, 3, 5]);
        println!("{}", num_array.sum_range(0,2));
        num_array.update(1,2);
        println!("{}", num_array.sum_range(0, 2));
    }

    #[test]
    fn solution_309() {
        println!("{}", max_profit(vec![1,2,3,0,2]));
        println!("{}", max_profit(vec![1]));
    }

    #[test]
    fn solution_310() {
        println!("{:?}", find_min_height_trees(4, vec![vec![1,0], vec![1,2], vec![1,3]]));
        println!("{:?}", find_min_height_trees(6, vec![vec![3,0], vec![3,1], vec![3,2],
                                                       vec![3,4], vec![5,4]]));
    }

    #[test]
    fn solution_313() {
        println!("{}", nth_super_ugly_number(12, vec![2, 7, 13, 19]));
        println!("{}", nth_super_ugly_number(1, vec![2, 3, 5]));
    }
}
