pub fn find_trailing_zeroes(n: i64) -> i64 {
    let mut count = 0;
    let mut divisor = 5;

    while n >= divisor {
        count += n / divisor;
        divisor *= 5;
    }
    count
}
