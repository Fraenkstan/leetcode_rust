use std::collections::{HashMap, HashSet};
use std::ptr::NonNull;

/// 双向链表 + 两个 HashMap
///
/// 本实现使用 `NonNull` 封装裸指针，需要注意在删除节点或者 Drop 整个结构的时候需要用
/// `Box::from_raw()` 来释放内存。
///
/// * `keys: HashMap<String, u32>` 记录该数据结构内所有的 key 对应的值，以便用 O(1) 的时间查找 key
/// * `index: HashMap<u32, NonNull<Node>>` 记录该数据结构内所有 key 的值（作为 key）在链表结构里对应的节点的指针
#[derive(Default, Debug)]
pub struct AllOne {
    head: Option<NonNull<Node>>,
    tail: Option<NonNull<Node>>,
    keys: HashMap<String, u32>,
    index: HashMap<u32, NonNull<Node>>,
}

// Guaranteed Sync and Send for AllOne
unsafe impl Send for AllOne {}
unsafe impl Sync for AllOne {}

/// 链表节点，节点的值为该节点记录的 key 对应的值，同时记录所有等于该值的 key 的 set。
#[derive(Default, Debug)]
struct Node {
    next: Option<NonNull<Node>>,
    prev: Option<NonNull<Node>>,
    value: u32,
    keys: HashSet<String>,
}

enum Offset {
    Inc,
    Dec,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl AllOne {
    /** Initialize your data structure here. */
    pub fn new() -> Self {
        Default::default()
    }

    /** Inserts a new key <Key> with value 1. Or increments an existing key by 1. */
    pub fn inc(&mut self, key: String) {
        unsafe {
            self.update_node(&key, Offset::Inc);
        }
    }

    /** Decrements an existing key by 1. If Key's value is 1, remove it from the data structure. */
    pub fn dec(&mut self, key: String) {
        unsafe {
            self.update_node(&key, Offset::Dec);
        }
    }

    /** Returns one of the keys with maximal value. */
    pub fn get_max_key(&self) -> String {
        self.tail
            .as_ref()
            .and_then(|tail| unsafe { (*tail.as_ptr()).keys.iter().next().map(Clone::clone) })
            .unwrap_or_else(|| "".to_string())
    }

    /** Returns one of the keys with Minimal value. */
    pub fn get_min_key(&self) -> String {
        self.head
            .as_ref()
            .and_then(|head| unsafe { (*head.as_ptr()).keys.iter().next().map(Clone::clone) })
            .unwrap_or_else(|| "".to_string())
    }

    // `pop_head()` will constructs a node from its raw point, which will be destructured
    // and freed allocated memory correctly.
    fn pop_head(&mut self) -> Option<Box<Node>> {
        self.head.map(|node| unsafe {
            let node = Box::from_raw(node.as_ptr());
            self.head = node.next;
            match self.head {
                Some(head) => (*head.as_ptr()).prev = None,
                None => self.tail = None,
            }
            node
        })
    }

    // TODO simplify codes
    unsafe fn update_node(&mut self, key: &String, offset: Offset) {
        let (old_v, new_v) = if let Some(&v) = self.keys.get(key) {
            let new_v = match offset {
                Offset::Inc => v + 1,
                Offset::Dec => v - 1,
            };
            (self.keys.insert(key.clone(), new_v), new_v)
        } else {
            let new_v = match offset {
                Offset::Inc => 1,
                Offset::Dec => return,
            };
            (self.keys.insert(key.clone(), new_v), new_v)
        };

        // update index
        let old_index = if let Some(index) = self.index.get_mut(&new_v) {
            (*index.as_ptr()).keys.insert(key.clone());
            old_v
                .and_then(|old_v| self.index.get_mut(&old_v))
                .map(|v| *v)
        } else if new_v > 0 {
            // insert a new node
            let mut node = Box::new(Node::default());
            node.value = new_v;
            node.keys.insert(key.clone());
            // find old and attach new node
            let (old_index, node) =
                if let Some(old_index) = old_v.and_then(|old_v| self.index.get_mut(&old_v)) {
                    let old_index = old_index.clone();
                    let node = match offset {
                        Offset::Inc => {
                            // insert after
                            node.prev = Some(old_index);
                            node.next = (*old_index.as_ptr()).next.take();
                            let node = Box::leak(node).into();
                            (*old_index.as_ptr()).next = Some(node);
                            (*node.as_ptr())
                                .next
                                .as_mut()
                                .map(|n| (*n.as_ptr()).prev = Some(node));
                            node
                        }
                        Offset::Dec => {
                            // insert before
                            node.next = Some(old_index);
                            node.prev = (*old_index.as_ptr()).prev.take();
                            let node = Box::leak(node).into();
                            (*old_index.as_ptr()).prev = Some(node);
                            (*node.as_ptr())
                                .prev
                                .as_mut()
                                .map(|n| (*n.as_ptr()).next = Some(node));
                            node
                        }
                    };

                    // if new node is tail, update `tail` ptr
                    if (*node.as_ptr()).next.is_none() {
                        self.tail = Some(node);
                    }
                    // if new node is head, update `head` ptr
                    if (*node.as_ptr()).prev.is_none() {
                        self.head = Some(node);
                    }
                    (Some(old_index), node)
                } else {
                    node.next = self.head;
                    let node = Box::leak(node).into();
                    match self.head {
                        Some(head) => (*head.as_ptr()).prev = Some(node),
                        None => self.tail = Some(node),
                    }
                    self.head = Some(node);
                    (None, node)
                };
            self.index.insert(new_v, node);
            old_index
        } else {
            // if new_v == 0, no need to add a new node, but remove instead
            old_v
                .and_then(|old_v| self.index.get_mut(&old_v))
                .map(|v| *v)
        };

        if let Some(old_index) = old_index {
            (*old_index.as_ptr()).keys.remove(key);
            if (*old_index.as_ptr()).keys.is_empty() {
                // Should drop node
                let mut node = Box::from_raw(old_index.as_ptr());
                self.index.remove(&node.value);

                let (mut next, mut prev) = (node.next.take(), node.prev.take());
                if let Some(ref mut next) = next {
                    (*next.as_ptr()).prev = prev;
                } else {
                    self.tail = prev;
                }

                if let Some(ref mut prev) = prev {
                    (*prev.as_ptr()).next = next;
                } else {
                    // has no prev, update head
                    self.head = next;
                }
            }
        }
    }
}

impl Drop for AllOne {
    /// Must pop every node to drop allocated memory of nodes
    fn drop(&mut self) {
        while let Some(node) = self.pop_head() {
            drop(node);
        }
    }
}
