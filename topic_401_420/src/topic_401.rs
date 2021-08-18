

pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
    let mut ans = vec![];
    (0..12).into_iter().for_each(|h| {
        (0..60).into_iter().for_each(|m| {
            if turned_on == bit_count(h) + bit_count(m) {
                if m < 10 {
                    ans.push(format!("{}:0{}", h, m));
                } else { ans.push(format!("{}:{}", h, m)) }
            }
        })
    });
    ans
}

fn bit_count(mut i: i32) -> i32 {
    i = i - ((i >> 1) & 0x55555555);
    i = (i & 0x33333333) + ((i >> 2) & 0x33333333);
    i = (i + (i >> 4)) & 0x0f0f0f0f;
    i = i + (i >> 8);
    i = i + (i >> 16);
    i & 0x3f
}