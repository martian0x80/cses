use std::cmp::{max, min};

pub fn is_possible(a: i64, b: i64) -> bool {
    (a + b) % 3 == 0 && 2 * min(a, b) >= max(a, b)
}
