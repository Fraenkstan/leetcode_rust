pub fn circular_array_loop(mut nums: Vec<i32>) -> bool {
    let n = nums.len();
    for i in 0..n {
        if nums[i] == 0 {
            continue;
        }
        let next = |i| ((i as i32 + nums[i]) % n as i32 + n as i32) as usize % n;
        let (mut slow, mut fast) = (i, next(i));
        while nums[slow] * nums[fast] > 0 && nums[slow] * nums[next(fast)] > 0 {
            if slow == fast {
                if slow != next(slow) {
                    return true;
                }
                break;
            }
            slow = next(slow);
            fast = next(next(fast));
        }
        let mut add = i;
        loop {
            let next = |i| ((i as i32 + nums[i]) % n as i32 + n as i32) as usize % n;
            if nums[add] * nums[next(add)] <= 0 {
                break;
            }
            let t = add;
            add = next(add);
            nums[t] = 0;
        }
    }
    false
}
