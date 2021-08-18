

pub fn check_record(s: String) -> bool {
    !s.chars().scan((0, 0), |(late, absent), c| {
        match c {
            'P' => *late = 0,
            'A' => {
                *late = 0;
                *absent += 1;
            },
            'L' => *late += 1,
             _ => {}
        }
        Some((*late, *absent))
    }).any(|(late, absent)| {
        late > 2 || absent > 1
    })
}