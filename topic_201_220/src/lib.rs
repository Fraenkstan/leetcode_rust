
mod topic_203;
mod topic_208;
mod topic_211;

#[cfg(test)]
mod tests {
    use data_structure::list_node::ListNode;
    use crate::topic_203::remove_elements;

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
}
