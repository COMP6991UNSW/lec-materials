fn first_word(s: &str) -> &str {
    s.split_once(' ')
        .map(|(before, after)| before)
        .unwrap_or(s)
}
