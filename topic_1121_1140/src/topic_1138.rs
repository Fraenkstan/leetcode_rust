
pub fn find_num_of_valid_words(words: Vec<String>, puzzles: Vec<String>) -> Vec<i32> {
    match_bytes(
        &words.iter().map(String::as_str).collect::<Vec<_>>(),
        &puzzles.iter().map(String::as_str).collect::<Vec<_>>(),
    )
        .into_iter()
        .map(|e| e as i32)
        .collect()
}

fn match_bytes(words: &[&str], puzzles: &[&str]) -> Vec<usize> {
    fn into_mask(a: u8) -> u32 {
        1 << (a - b'a')
    }
    fn into_set(s: &str) -> u32 {
        s.bytes().fold(0, |s, c| s | into_mask(c))
    }
    let words: Vec<u32> = words.iter().copied().map(into_set).collect();
    puzzles
        .iter()
        .copied()
        .map(|puzzle| {
            let mask = into_mask(puzzle.bytes().next().unwrap());
            let puzzle = into_set(puzzle);
            words
                .iter()
                .copied()
                .filter(|&word| word & mask != 0 && word & puzzle == word)
                .count()
        })
        .collect()
}