use crate::ListNode;

pub fn get_kth_from_end(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
    let mut p0 = head.as_ref().unwrap();
    let mut p1 = head.as_ref().unwrap();
    for _i in 1..k as usize {
        if p1.next.is_none() {
            return None;
        }
        p1 = p1.next.as_ref().unwrap();
    }
    while p1.next.is_some() {
        p1 = p1.next.as_ref().unwrap();
        p0 = p0.next.as_ref().unwrap();
    }
    Some(p0.clone())
}