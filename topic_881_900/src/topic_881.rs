pub fn num_rescue_boats(people: Vec<i32>, limit: i32) -> i32 {
    let mut people = people;
    people.sort_unstable();
    let (mut left, mut right, mut ans) = (0usize, people.len() - 1, 0);
    while left < right {
        if people[left] + people[right] <= limit {
            ans += 1;
            left += 1;
            right -= 1;
        } else {
            ans += 1;
            right -= 1;
        }
    }
    return if left == right { ans + 1 } else { ans };
}
