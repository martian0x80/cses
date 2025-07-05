pub fn find_longest_repetition(dna: &str) -> i64 {
    let mut longest: i64 = 0;

    let mut current: i64 = 0;
    let mut last_char = 'K';
    for i in dna.chars() {
        if i == last_char {
            current += 1;
        } else {
            current = 1;
        }
        if current > longest {
            longest = current;
        }
        last_char = i;
    }
    longest
}
