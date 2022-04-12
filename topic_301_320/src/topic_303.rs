#[allow(unused)]
struct NumArray {
    nums: Vec<i32>,
    subscript: Vec<i32>, // 代表 以 i 为 右侧索引的范围和
}

/*
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
#[allow(unused)]
impl NumArray {
    fn new(nums: Vec<i32>) -> Self {
        let mut subscript = vec![0; nums.len()];
        let mut sum = 0;
        for i in 0..nums.len() {
            sum += nums[i];
            subscript[i] = sum;
        }
        NumArray { subscript, nums }
    }

    fn sum_range(&self, i: i32, j: i32) -> i32 {
        if i == 0 {
            self.subscript[j as usize]
        } else {
            self.subscript[j as usize] - self.subscript[i as usize - 1]
        }
    }
}

/*
 * Your NumArray object will be instantiated and called as such:
 * let obj = NumArray::new(nums);
 * let ret_1: i32 = obj.sum_range(i, j);
 */
