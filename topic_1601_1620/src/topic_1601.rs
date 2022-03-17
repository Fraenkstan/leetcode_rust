

pub fn maximum_requests(_n: i32, req: Vec<Vec<i32>>) -> i32 {
    let p = [
        2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
    ];
    let states: i32 = 1 << req.len() as i32;
    let mut ans = 0;
    for i in 1..states {
        let mut ind: i32 = 1;
        let mut oud: i32 = 1;
        for j in 0..req.len() {
            if i & (1 << j as i32) > 0 {
                ind = ind.wrapping_mul(p[req[j][0] as usize]);
                oud = oud.wrapping_mul(p[req[j][1] as usize]);
            }
        }
        if ind == oud && ind != 1 {
            ans = ans.max(i.count_ones());
        }
    }
    ans as i32
}