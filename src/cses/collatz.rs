fn collatz_recursive(n: i64, sequence: &mut Vec<i64>) -> i64 {
    sequence.push(n);
    if n == 1 {
        1
    } else if n % 2 == 0 {
        collatz_recursive(n / 2, sequence)
    } else {
        collatz_recursive(3 * n + 1, sequence)
    }
}

#[allow(dead_code)]
pub fn collatz(n: i64) -> Vec<i64> {
    let mut sequence: Vec<i64> = Vec::new();
    collatz_recursive(n, &mut sequence);
    sequence
}
