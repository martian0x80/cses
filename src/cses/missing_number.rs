use std::collections::HashSet;

pub fn find_missing(seq: &[i64], n: i64) -> i64 {
    let nums: HashSet<i64> = seq.iter().cloned().collect();

    for i in 1..=n {
        if !nums.contains(&i) {
            return i;
        }
    }
    -1
}
