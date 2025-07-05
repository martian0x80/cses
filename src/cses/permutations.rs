pub fn find_beautiful_sequence(n: i64) -> String {
    if n == 1 {
        return "1".to_string();
    }
    if n <= 3 {
        return String::from("NO SOLUTION");
    }
    let mut solution = String::new();
    let mut init = 2;
    while init <= n {
        solution.push_str(init.to_string().as_str());
        solution.push_str(" ");
        init += 2;
    }
    init = 1;
    while init <= n {
        solution.push_str(init.to_string().as_str());
        solution.push_str(" ");
        init += 2;
    }
    solution.pop();
    solution
}
