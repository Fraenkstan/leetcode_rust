#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

#[allow(unused)]
impl ListNode {
    #[inline]
    pub(crate) fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

mod topic_02_01;
mod topic_02_02;
mod topic_02_04;
mod topic_02_05;
mod topic_02_06;

#[cfg(test)]
mod tests {
    use crate::topic_02_01::remove_duplicate_nodes;
    use crate::topic_02_02::kth_to_last;
    use crate::topic_02_04::partition;
    use crate::topic_02_05::add_two_numbers;
    use crate::topic_02_06::is_palindrome;
    use crate::ListNode;

    #[test]
    fn solution_02_01() {
        let node0 = ListNode::new(4);
        let node1 = ListNode {
            val: 1,
            next: Some(Box::new(node0)),
        };
        let node2 = ListNode {
            val: 2,
            next: Some(Box::new(node1)),
        };
        let node3 = ListNode {
            val: 3,
            next: Some(Box::new(node2)),
        };
        let node4 = ListNode {
            val: 3,
            next: Some(Box::new(node3)),
        };
        let node5 = ListNode {
            val: 2,
            next: Some(Box::new(node4)),
        };
        let head = ListNode {
            val: 1,
            next: Some(Box::new(node5)),
        };
        println!("{:?}", remove_duplicate_nodes(Some(Box::new(head))));
    }

    #[test]
    fn solution_02_02() {
        let node5 = ListNode::new(5);
        let node4 = ListNode {
            val: 4,
            next: Some(Box::new(node5)),
        };
        let node3 = ListNode {
            val: 3,
            next: Some(Box::new(node4)),
        };
        let node2 = ListNode {
            val: 2,
            next: Some(Box::new(node3)),
        };
        let node1 = ListNode {
            val: 1,
            next: Some(Box::new(node2)),
        };
        println!("{}", kth_to_last(Some(Box::new(node1)), 2));
    }

    #[test]
    fn solution_02_04() {
        let node1 = ListNode::new(2);
        let node2 = ListNode {
            val: 5,
            next: Some(Box::new(node1)),
        };
        let node3 = ListNode {
            val: 2,
            next: Some(Box::new(node2)),
        };
        let node4 = ListNode {
            val: 3,
            next: Some(Box::new(node3)),
        };
        let node5 = ListNode {
            val: 4,
            next: Some(Box::new(node4)),
        };
        let head = ListNode {
            val: 1,
            next: Some(Box::new(node5)),
        };
        println!("{:?}", partition(Some(Box::new(head)), 3));
    }

    #[test]
    fn solution_02_05() {
        let node1 = ListNode::new(6);
        let node2 = ListNode {
            val: 1,
            next: Some(Box::new(node1)),
        };
        let node3 = ListNode {
            val: 7,
            next: Some(Box::new(node2)),
        };

        let node4 = ListNode::new(2);
        let node5 = ListNode {
            val: 9,
            next: Some(Box::new(node4)),
        };
        let node6 = ListNode {
            val: 5,
            next: Some(Box::new(node5)),
        };
        println!(
            "{:?}",
            add_two_numbers(Some(Box::new(node3)), Some(Box::new(node6)))
        );
    }

    #[test]
    fn solution_02_06() {
        let node1 = ListNode::new(1);
        let node2 = ListNode {
            val: 2,
            next: Some(Box::new(node1)),
        };
        let node3 = ListNode {
            val: 2,
            next: Some(Box::new(node2)),
        };
        let node4 = ListNode {
            val: 1,
            next: Some(Box::new(node3)),
        };
        println!("{}", is_palindrome(Some(Box::new(node4))));
    }
}
