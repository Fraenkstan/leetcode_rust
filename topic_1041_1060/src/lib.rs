mod topic_1047;
mod topic_1049;
mod topic_1052;

#[cfg(test)]
mod tests {
    use crate::topic_1047::remove_duplicates;
    use crate::topic_1049::{last_stone_weight_ii, solulast_stone_weight_ii_2};
    use crate::topic_1052::max_satisfied;

    #[test]
    fn solution_1047() {
        let s = "addadssd".to_string();
        println!("{}", remove_duplicates(s));
    }

    #[test]
    fn solution_1049() {
        println!("{}", last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]));
        println!("{}", last_stone_weight_ii(vec![31, 26, 33, 21, 40]));
        println!("{}", last_stone_weight_ii(vec![1, 2]));

        println!("{}", solulast_stone_weight_ii_2(vec![2, 7, 4, 1, 8, 1]));
        println!("{}", solulast_stone_weight_ii_2(vec![31, 26, 33, 21, 40]));
        println!("{}", solulast_stone_weight_ii_2(vec![1, 2]));
    }

    #[test]
    fn solution_1052() {
        let customers = vec![1, 0, 1, 2, 1, 1, 7, 5];
        let grumpy = vec![0, 1, 0, 1, 0, 1, 0, 1];
        customers
            .iter()
            .zip(grumpy.iter())
            .for_each(|(customer, grumpy)| println!("{}, {}", customer, grumpy));
        println!("{}", max_satisfied(customers, grumpy, 3));
    }
}
