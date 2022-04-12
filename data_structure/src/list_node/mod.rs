use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListNode<T> {
    val: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> Display for ListNode<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        return match &self.next {
            None => {
                write!(f, "{} -> {}", self.val, "none")
            }
            Some(next) => {
                write!(f, "{} -> {}", self.val, next)
            }
        };
    }
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> ListNode<T> {
        ListNode { val, next: None }
    }

    pub fn add_last(&mut self, node: ListNode<T>) {
        match &self.next {
            None => {
                self.next = Some(Box::new(node));
            }
            Some(..) => {
                self.next.as_mut().unwrap().add_last(node);
            }
        }
    }

    pub fn add_last_val(&mut self, val: T) {
        match &self.next {
            None => {
                self.next = Some(Box::new(ListNode::new(val)));
            }
            Some(..) => {
                self.next.as_mut().unwrap().add_last_val(val);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::list_node::ListNode;

    #[test]
    fn test() {
        let mut node0 = ListNode::new(0);
        let mut node1 = ListNode::new(1);
        let mut node2 = ListNode::new(2);
        node2.add_last_val(3);
        node2.add_last(ListNode::new(4));
        node1.next = Some(Box::new(node2));
        node0.next = Some(Box::new(node1));
        println!("{}", node0);
    }
}
