use std::collections::HashMap;
use trace::trace;

trace::init_depth_var!();

#[trace(prefix_enter = "ENTER", prefix_exit = "EXIT")]
pub fn find_shortest_sub_array(nums: Vec<i32>) -> i32 {
    let mut map = HashMap::with_capacity(50000);
    let mut max = 0;
    let mut len = 0;
    for i in 0..nums.len() {
        // 数据第一次出现则插入数据。
        if let None = map.get(&nums[i]) {
            map.insert(nums[i], (1, i));
        }
        // 对已插入hashmap的数据进行处理。
        if let Some(&(num, index)) = map.get(&nums[i]) {
            // 若出现次数相同则取最小的len.
            if num + 1 == max {
                len = len.min((i + 1 - index) as i32);
            }
            // 若出现次数更多则更新len和max.
            if num + 1 > max {
                max = num + 1;
                len = (i + 1 - index) as i32;
            }
            map.insert(nums[i], (num + 1, index));
        }
    }
    len
}
