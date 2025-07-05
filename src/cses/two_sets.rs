use std::collections::HashSet;

pub fn find_sets(n: i64) -> Option<Vec<Vec<i64>>> {
    if (n * (n + 1) / 2) % 2 != 0 {
        return None;
    }
    let mut solutions = vec![];
    if n % 2 == 0 {
        let mut set1 = vec![];
        let mut set2 = vec![];
        for i in 0..n / 2 {
            if i % 2 == 0 {
                set1.push(i + 1);
                set1.push(n - i);
            } else {
                set2.push(i + 1);
                set2.push(n - i);
            }
        }
        solutions.push(set1);
        solutions.push(set2);
    } else {
        let mut set1 = vec![];
        let mut set2 = vec![];
        for i in 0..n / 2 {
            if i % 2 == 0 {
                set1.push(i + 1);
                set1.push(n - i - 1);
            } else {
                set2.push(i + 1);
                set2.push(n - i - 1);
            }
        }
        set2.push(n);
        solutions.push(set1);
        solutions.push(set2);
    }
    Some(solutions)
}

pub fn greedy_find_sets(n: i64) -> Option<Vec<Vec<i64>>> {
    let sum = n * (n + 1) / 2;
    if sum % 2 != 0 {
        return None;
    }
    let target = sum / 2;
    let mut set1 = HashSet::new();
    let mut set2 = vec![];
    let mut sum1 = 0;
    let mut current = n;
    while sum1 + current < target {
        set1.insert(current);
        sum1 += current;
        current -= 1;
    }
    // add remaining difference to set1
    let remaining = target - sum1;
    if remaining > 0 {
        set1.insert(remaining);
    }
    // fill set2 with remaining numbers
    for i in 1..=n {
        if !set1.contains(&i) {
            set2.push(i);
        }
    }
    Some(vec![set1.into_iter().collect(), set2])
}
