

pub fn maximum_time(time: String) -> String {
    let mut time = time.chars().into_iter().collect::<Vec<char>>();
    match (time[0], time[1]) {
        ('?', '?') => {
            time[0] = '2';
            time[1] = '3';
        }
        ('?', t1) if t1 < '4' => time[0] = '2',
        ('?', _) => time[0] = '1',
        ('2', '?') => time[1] = '3',
        (_, '?') => time[1] = '9',
        (_, _) => ()
    }
    if time[3] == '?' { time[3] = '5' }
    if time[4] == '?' { time[4] = '9' }
    time.into_iter().collect::<String>()
}

