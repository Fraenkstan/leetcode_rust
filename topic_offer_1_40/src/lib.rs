#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

mod topic_offer_10;
mod topic_offer_22;
mod topic_offer_38;

#[allow(unused)]
#[cfg(test)]
mod tests {
    use crate::topic_offer_10::fib;
    use crate::topic_offer_22::get_kth_from_end;
    use crate::topic_offer_38::{permutation, permutation_1};
    use crate::ListNode;

    #[test]
    fn solution_offer_10() {
        println!("{}", fib(2));
        println!("{}", fib(3));
    }

    #[test]
    fn solution_offer_22() {
        let mut node1 = ListNode::new(1);
        let mut node2 = ListNode::new(2);
        let mut node3 = ListNode::new(3);
        let mut node4 = ListNode::new(4);
        let mut node5 = ListNode::new(5);
        let node6 = ListNode::new(6);
        node5.next = Some(Box::from(node6));
        node4.next = Some(Box::from(node5));
        node3.next = Some(Box::from(node4));
        node2.next = Some(Box::from(node3));
        node1.next = Some(Box::from(node2));
        println!("{:?}", get_kth_from_end(Some(Box::from(node1)), 2));
    }

    #[test]
    fn solution_offer_38() {
        println!("{:?}", permutation("abc".to_string()));
        println!("{:?}", permutation_1("abc".to_string()));
        println!("{:?}", permutation_1("dcz".to_string()));
        println!("{:?}", permutation("dcz".to_string()));
    }
}
