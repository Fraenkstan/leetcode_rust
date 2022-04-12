use std::collections::HashMap;

pub struct DetectSquares {
    cnt: HashMap<i32, HashMap<i32, i32>>,
}

impl DetectSquares {
    pub fn new() -> Self {
        DetectSquares {
            cnt: {
                let map = HashMap::new();
                map
            },
        }
    }

    pub fn add(&mut self, point: Vec<i32>) {
        let (x, y) = (point[0], point[1]);
        let y_cnt = self.cnt.entry(y).or_insert(HashMap::new());
        y_cnt.insert(x, *y_cnt.get(&x).unwrap_or(&0) + 1);
    }

    pub fn count(&self, point: Vec<i32>) -> i32 {
        let mut res = 0;
        let (x, y) = (point[0], point[1]);
        if !self.cnt.contains_key(&y) {
            return 0;
        }
        let y_cnt = self.cnt.get(&y).unwrap();
        self.cnt.iter().for_each(|(&col, col_cnt)| {
            if y != col {
                let d = col - y;
                res += col_cnt.get(&x).unwrap_or(&0)
                    * y_cnt.get(&(x + d)).unwrap_or(&0)
                    * col_cnt.get(&(x + d)).unwrap_or(&0);
                res += col_cnt.get(&x).unwrap_or(&0)
                    * y_cnt.get(&(x - d)).unwrap_or(&0)
                    * col_cnt.get(&(x - d)).unwrap_or(&0);
            }
        });
        res
    }
}
