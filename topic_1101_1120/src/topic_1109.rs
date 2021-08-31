

pub fn corp_flight_bookings(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
    bookings.iter().fold(vec![0; n as usize], |mut ans, book| {
        for i in book[0] as usize..=book[1] as usize {
            ans[i - 1] += book[2];
        }
        ans
    })
}

// 查分数组
pub fn corp_flight_bookings1(bookings: Vec<Vec<i32>>, n: i32) -> Vec<i32> {
    let mut ans = vec![0; n as usize];
    bookings.iter().for_each(|book| {
        ans[book[0] as usize - 1] += book[2];
        if book[1] < n {
            ans[book[1] as usize] -= book[2];
        }
    });
    for i in 1..n as usize {
        ans[i] += ans[i - 1];
    }
    ans
}