mod topic_141;
mod topic_149;
mod topic_160;

#[cfg(test)]
mod tests {
    use crate::topic_149::max_points;
    use crate::topic_160::get_intersection_node;
    use data_structure::list_node::ListNode;

    #[test]
    fn solution_149() {
        println!("{}", max_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]));
        println!(
            "{}",
            max_points(vec![
                vec![1, 1],
                vec![3, 2],
                vec![5, 3],
                vec![4, 1],
                vec![2, 3],
                vec![1, 4]
            ])
        );
        println!("{}", max_points(vec![vec![4, 5], vec![4, -1], vec![4, 0]]));
        println!("{}", max_points(vec![vec![0, 0], vec![1, -1], vec![1, 1]]));
    }

    #[test]
    fn solution_160() {
        let mut head_a = ListNode::new(0);
        head_a.add_last(ListNode::new(9));
        head_a.add_last(ListNode::new(1));
        head_a.add_last(ListNode::new(2));
        head_a.add_last(ListNode::new(4));

        let mut head_b = ListNode::new(3);
        head_b.add_last(ListNode::new(2));
        head_b.add_last(ListNode::new(4));
        println!(
            "{:?}",
            get_intersection_node(Some(Box::new(head_a)), Some(Box::new(head_b)))
        );
    }
}
