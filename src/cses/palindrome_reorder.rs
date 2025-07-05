pub fn reorder(s: &str) -> Option<String> {
    let chars = s.chars();
    let mut freq = [0; 26];
    for c in chars {
        freq[c as usize - 'A' as usize] += 1;
    }
    let mut odd_count = freq.iter().filter(|&x| x % 2 == 1).count();
    // if odd_count > 1 {
    //     return "NO SOLUTION".to_string();
    // }
    if s.len() % 2 != 0 {
        if odd_count > 1 {
            return None;
        }
    } else {
        if odd_count > 0 {
            return None;
        }
    }

    let mut middle: Option<char> = None;
    let mut half = String::new();
    for (i, f) in freq.iter().enumerate() {
        let c = (b'A' + i as u8) as char;
        if f % 2 != 0 {
            middle = Some(c);
        }
        half.push_str(&c.to_string().repeat((f / 2) as usize));
    }
    Some(format!(
        "{}{}{}",
        half,
        middle.unwrap_or('\0'),
        half.chars().rev().collect::<String>()
    ))
    .map(|s| s.replace('\0', ""))
}
