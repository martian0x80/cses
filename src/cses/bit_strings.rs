use std::ops::BitAnd;

pub fn get_count(n: i64) -> i64 {
    if n == 0 {
        1
    } else {
        let mut result = 1;
        let mut a = 2;
        let mut b = n;
        while n > 0 {
            if b.bitand(1) == 1 {
                result = (result * a) % 1_000_000_007;
            }
            a = (a * a) % 1_000_000_007;
            b >>= 1;
        }
        result
    }
}
