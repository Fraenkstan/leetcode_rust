use data_structure::list_node::ListNode;

pub fn has_cycle(root: Option<Box<ListNode<i32>>>) -> bool {
    if root.is_none() || root.unwrap().next.is_none() {
        return false;
    }
    let mut p0 = root.unwrap();
    let mut p1 = root.unwrap().next.unwrap().next.unwrap();
    while p0 != p1 {

    }
    true
}