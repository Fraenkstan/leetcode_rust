

pub fn unhappy_friends(n: i32, preferences: Vec<Vec<i32>>, pairs: Vec<Vec<i32>>) -> i32 {
    let n = n as usize;
    let mut order = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..(n - 1) {
            order[i][preferences[i][j] as usize] = j;
        }
    }
    let mut match_vec = vec![0; n];
    for pair in pairs {
        let person0 = pair[0];
        let person1 = pair[1];
        match_vec[person0 as usize] = person1;
        match_vec[person1 as usize] = person0;
    }
    let mut count = 0;
    for x in 0..n {
        let y = match_vec[x];
        let index = order[x][y as usize];
        for i in 0..index as usize {
            let u = preferences[x][i];
            let v = match_vec[u as usize];
            if order[u as usize][x] < order[u as usize][v as usize] {
                count += 1;
                break
            }
        }
    }
    count
}