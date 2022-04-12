mod topic_203;
mod topic_208;
mod topic_211;
mod topic_218;

#[cfg(test)]
mod tests {
    use crate::topic_203::remove_elements;
    use crate::topic_218::get_skyline;
    use data_structure::list_node::ListNode;

    #[test]
    fn topic_203() {
        let mut head = ListNode::<i32>::new(1);
        head.add_last_val(2);
        head.add_last_val(6);
        head.add_last_val(3);
        head.add_last_val(4);
        head.add_last_val(5);
        head.add_last_val(6);
        println!("{}", remove_elements(Some(Box::new(head)), 6).unwrap());
    }

    #[test]
    fn topic_218() {
        println!(
            "{:?}",
            get_skyline(vec![
                vec![2, 9, 10],
                vec![3, 7, 15],
                vec![5, 12, 12],
                vec![15, 20, 10],
                vec![19, 24, 8]
            ])
        );
        println!("{:?}", get_skyline(vec![vec![0, 2, 3], vec![2, 5, 3]]));
    }
}
