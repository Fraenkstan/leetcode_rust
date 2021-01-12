#[derive(Debug, Clone, Eq, PartialEq)]
pub struct ListNode<T> {
    val: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> ListNode<T> {
        ListNode { val, next: None }
    }
}
