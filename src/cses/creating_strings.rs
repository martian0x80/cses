fn permutate(
    input: &Vec<char>,
    permutations: &mut Vec<String>,
    used: &mut Vec<bool>,
    current: &mut String,
) {
    if current.len() == input.len() {
        permutations.push(current.clone());
        return;
    }
    for i in 0..input.len() {
        if used[i] {
            continue;
        }
        if i > 0 && input[i] == input[i - 1] && !used[i - 1] {
            continue;
        }
        used[i] = true;
        current.push(input[i]);
        permutate(input, permutations, used, current);
        current.pop();
        used[i] = false;
    }
}

pub fn generate_permutations(inp: &str) -> Vec<String> {
    if inp.is_empty() {
        return vec![];
    }
    let mut input_chars: Vec<char> = inp.chars().collect();
    input_chars.sort_unstable();
    let mut permutations = Vec::new();
    let mut used = vec![false; inp.len()];
    permutate(
        &input_chars,
        &mut permutations,
        &mut used,
        &mut String::new(),
    );
    permutations
}
