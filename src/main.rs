mod cses;
mod utils;

use cses::gray_code;
use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let n = line.parse::<u32>().unwrap();
    for i in gray_code::get_codes_list(n) {
        println!("{i}");
    }
}
