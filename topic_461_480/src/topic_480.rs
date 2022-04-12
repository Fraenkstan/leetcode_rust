use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};

pub fn median_sliding_window(nums: Vec<i32>, k: i32) -> Vec<f64> {
    let mut ret: Vec<f64> = vec![];
    let mut left_heap_max = BinaryHeap::new();
    let mut right_heap_min: BinaryHeap<Reverse<i32>> = BinaryHeap::new();
    let flag = if k % 2 == 0 { true } else { false }; // if flag is false, left has 1 more
    let mut balance = k % 2; // maintain 2 binary heap is to keep balance 0
    let mut map: HashMap<i32, i32> = HashMap::new();

    // init heaps
    for i in 0..k as usize {
        left_heap_max.push(nums[i]);
        balance -= 1;
    }
    balance_heaps(
        &mut left_heap_max,
        &mut right_heap_min,
        &mut balance,
        &mut map,
    );
    ret.push(get_median(&left_heap_max, &right_heap_min, flag));

    // iter
    for i in 1..nums.len() - k as usize + 1 {
        proc_outdate(
            nums[i - 1],
            &mut left_heap_max,
            &mut right_heap_min,
            &mut map,
            &mut balance,
        );
        push_new(
            nums[i + k as usize - 1],
            &mut left_heap_max,
            &mut right_heap_min,
            &mut balance,
        );
        balance_heaps(
            &mut left_heap_max,
            &mut right_heap_min,
            &mut balance,
            &mut map,
        );
        ret.push(get_median(&left_heap_max, &right_heap_min, flag));
    }
    ret
}

fn balance_heaps(
    left: &mut BinaryHeap<i32>,
    right: &mut BinaryHeap<Reverse<i32>>,
    balance: &mut i32,
    map: &mut HashMap<i32, i32>,
) {
    if *balance == 0 {
        return;
    }
    if *balance < 0 {
        while *balance != 0 {
            right.push(Reverse(left.pop().unwrap()));
            check_hashmap_left(left, map);
            *balance += 2;
        }
    } else {
        while *balance != 0 {
            left.push(right.pop().unwrap().0);
            check_hashmap_right(right, map);
            *balance -= 2;
        }
    }
}

fn get_median(left: &BinaryHeap<i32>, right: &BinaryHeap<Reverse<i32>>, flag: bool) -> f64 {
    if flag {
        (*left.peek().unwrap_or(&0) as f64 + right.peek().unwrap_or(&Reverse(0)).0 as f64) / 2.0
    } else {
        *left.peek().unwrap() as f64
    }
}

fn proc_outdate(
    num: i32,
    left: &mut BinaryHeap<i32>,
    right: &mut BinaryHeap<Reverse<i32>>,
    map: &mut HashMap<i32, i32>,
    balance: &mut i32,
) {
    if let Some(x) = left.peek() {
        if num == *x {
            left.pop();
            *balance += 1;
            check_hashmap_left(left, map);
            return;
        } else if num < *x {
            let entry = map.entry(num).or_insert(0);
            *entry += 1;
            *balance += 1;
            return;
        }
    }

    if let Some(y) = right.peek() {
        if num == y.0 {
            right.pop();
            *balance -= 1;
            check_hashmap_right(right, map);
        } else if num > y.0 {
            let entry = map.entry(num).or_insert(0);
            *entry += 1;
            *balance -= 1;
        }
    }
}

fn check_hashmap_left(heap: &mut BinaryHeap<i32>, map: &mut HashMap<i32, i32>) {
    while let Some(x) = heap.peek() {
        let key = *x;
        if decrease_record(key, map) {
            heap.pop();
        } else {
            break;
        }
    }
}

fn check_hashmap_right(heap: &mut BinaryHeap<Reverse<i32>>, map: &mut HashMap<i32, i32>) {
    while let Some(x) = heap.peek() {
        let key = x.0;
        if decrease_record(key, map) {
            heap.pop();
        } else {
            break;
        }
    }
}

fn decrease_record(key: i32, map: &mut HashMap<i32, i32>) -> bool {
    if map.contains_key(&key) {
        if map[&key] > 1 {
            *map.entry(key).or_insert(0) -= 1;
        } else if map[&key] == 1 {
            map.remove(&key);
        }
        return true;
    }
    return false;
}

fn push_new(
    num: i32,
    left: &mut BinaryHeap<i32>,
    right: &mut BinaryHeap<Reverse<i32>>,
    balance: &mut i32,
) {
    if let Some(x) = left.peek() {
        if num <= *x {
            left.push(num);
            *balance -= 1;
            return;
        }
    }
    right.push(Reverse(num));
    *balance += 1;
}
