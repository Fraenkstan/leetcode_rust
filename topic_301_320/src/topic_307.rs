// zkw线段树；单点修改、区间查询
pub struct SegmentTree {
    data: Vec<i32>,
    base: usize,
}
impl SegmentTree {
    pub fn new(nums: Vec<i32>) -> Self {
        let mut base = 1;
        while base <= nums.len() {
            base <<= 1;
        }
        let mut data = vec![0; base << 1];
        for i in 0..nums.len() {
            data[base + 1 + i] = nums[i]; // 此处是从 base + 1 开始
        }
        for i in (1..base).rev() { // 父节点下标是 [1, base - 1]；因此base这个位置是空的
            data[i] = data[i << 1] + data[i << 1 | 1];
        }
        Self {
            data,
            base,
        }
    }
    pub fn update(&mut self, index: i32, val: i32) {
        // 线段树从下标1开始，与题目不一致，因此需要处理一下
        let mut i = self.base + 1 + index as usize;
        self.data[i] = val;
        i >>= 1;
        while i > 0 {
            self.data[i] = self.data[i << 1] + self.data[i << 1 | 1];
            i >>= 1;
        }
    }
    pub fn sum_range(&self, left: i32, right: i32) -> i32 {
        // 将[left, right]的查询改为(left - 1, right + 1)，再经过下标处理得到以下结果
        let (mut l, mut r) = (self.base + left as usize, self.base + right as usize + 2);
        let mut ret = 0;
        // 当两个节点的父节点相同时才结束循环
        while l ^ r ^ 1 != 0 {
            // 当l是左儿子时
            if l & 1 == 0 {
                ret += self.data[l ^ 1];
            }
            // 当r是右儿子时
            if r & 1 == 1 {
                ret += self.data[r ^ 1];
            }
            l >>= 1;
            r >>= 1;
        }
        ret
    }
}