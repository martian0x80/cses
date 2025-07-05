pub fn find_possible_moves(n: i64) -> Vec<i64> {
    let mut total_moves = vec![0];
    for k in 2..=n {
        let moves = k.pow(2) * (k.pow(2) - 1) / 2;
        let attack_moves = 4 * (k - 1) * (k - 2);
        total_moves.push(moves - attack_moves);
    }
    total_moves
}
