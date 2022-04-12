use crate::ListNode;

pub fn kth_to_last(head: Option<Box<ListNode>>, k: i32) -> i32 {
    let mut p1 = head.as_ref().unwrap();
    let mut p2 = head.as_ref().unwrap();
    let mut k = k;
    while k > 1 {
        p2 = p2.next.as_ref().unwrap();
        k -= 1;
    }
    while p2.next.is_some() {
        p2 = p2.next.as_ref().unwrap();
        p1 = p1.next.as_ref().unwrap();
    }
    p1.val
}
