mod topic_1202;
mod topic_1203;
mod topic_1219;

#[cfg(test)]
mod tests {

    use crate::topic_1202::smallest_string_with_swaps;
    use crate::topic_1203::sort_items;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn solution_1202() {
        let mut s = String::from("dcab");
        let pairs = vec![vec![0, 3], vec![1, 2]];
        s = smallest_string_with_swaps(s, pairs);
        println!("{}", s);
    }

    #[test]
    fn solution_1203() {
        println!("Hello, world!");
        let group1 = vec![-1,-1,1,0,0,1,0,-1];
        let before_items1 =
            vec![vec![], vec![6], vec![5], vec![6], vec![3,6],
                 vec![], vec![], vec![]];
        println!("{:?}", sort_items(8, 2, group1, before_items1));

        let group2 = vec![-1,-1,1,0,0,1,0,-1];
        let before_items2 =
            vec![vec![], vec![6], vec![5], vec![6], vec![3],
                 vec![], vec![4], vec![]];
        println!("{:?}", sort_items(8, 2, group2, before_items2));
    }
}
