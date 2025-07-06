mod cses;
mod utils;

use cses::creating_strings;
use std::io::{stdin, BufRead};

fn main() {
    let stdin = stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    // let n = line.parse::<u32>().unwrap();
    let perms = creating_strings::generate_permutations(&line);
    println!("{}", perms.len());
    for i in perms {
        println!("{i}");
    }
}
