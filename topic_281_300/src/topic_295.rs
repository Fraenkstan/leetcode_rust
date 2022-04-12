use std::cmp::Ordering;

#[derive(Debug)]
pub struct MedianFinder {
    data: Vec<i32>,
}

#[allow(unused)]
impl MedianFinder {
    /** initialize your data structure here. */
    pub fn new() -> Self {
        MedianFinder { data: vec![] }
    }

    pub fn add_num(&mut self, num: i32) {
        let mut l = 0;
        let mut r = self.data.len();
        let mut mid = 0;
        while l < r {
            mid = l + (r - l) / 2;
            match self.data[mid].cmp(&&num) {
                Ordering::Less => {
                    l = mid + 1;
                }
                Ordering::Equal => {
                    self.data.insert(mid, num);
                    return;
                }
                Ordering::Greater => {
                    r = mid;
                }
            }
        }
        self.data.insert(l, num);
    }

    pub fn find_median(&self) -> f64 {
        if self.data.len() == 0 {
            return 0.0;
        }
        if self.data.len() % 2 == 1 {
            self.data[self.data.len() / 2] as f64
        } else {
            (self.data[1.max(self.data.len() / 2) - 1] as f64
                + self.data[self.data.len() / 2] as f64)
                / 2.0
        }
    }
}
