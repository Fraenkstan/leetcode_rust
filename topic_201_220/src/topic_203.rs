use data_structure::list_node::ListNode;

pub fn remove_elements(
    head: Option<Box<ListNode<i32>>>,
    target: i32,
) -> Option<Box<ListNode<i32>>> {
    if head.is_none() {
        return None;
    }
    let mut p = head.unwrap();
    if target == p.val {
        return remove_elements(p.next, target);
    } else {
        p.next = remove_elements(p.next, target)
    }
    Some(p)
}
