pub fn print_matrix(mat: &Vec<Vec<i64>>) {
    for row in mat {
        for col in row {
            print!("{col:>3} ")
        }
        println!();
    }
}
