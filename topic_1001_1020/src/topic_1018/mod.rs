pub fn prefixes_div_by5(a: Vec<i32>) -> Vec<bool> {
    let mut result = Vec::with_capacity(a.len());
    let mut n = 0;

    for i in a {
        n <<= 1;
        n += i;
        result.push(n % 5 == 0);
        n %= 5;
    }
    result
}

fn prefixes_div_by5_1(a: Vec<i32>) -> Vec<bool> {
    a.iter()
        .scan(0, |res, &i| Some((2 * *res + i) % 5))
        .map(|x| x == 0)
        .collect()
}

pub fn prefixes_div_by5_2(a: Vec<i32>) -> Vec<bool> {
    let mut result: Vec<bool> = vec![];
    a.iter().fold(0, |res, i| {
        let res = (2 * res + i) % 5;
        result.push(res == 0);
        res
    });
    result
}
