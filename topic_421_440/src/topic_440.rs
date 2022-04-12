pub fn find_kth_number(n: i32, mut k: i32) -> i32 {
    let mut curr: i32 = 1;
    while k > 1 {
        let (mut step, mut p, mut q): (i64, i64, i64) = (0, curr as i64, curr as i64 + 1);
        // 计算节点数
        while p <= n as i64 {
            step += q.min(n as i64 + 1) - p;
            p *= 10;
            q *= 10;
        }
        // 根据结果在下一棵树还是当前树做不同的处理
        if k - 1 >= step as i32 {
            k -= step as i32;
            curr += 1;
        } else {
            k -= 1;
            curr *= 10;
        }
    }
    curr as i32
}
