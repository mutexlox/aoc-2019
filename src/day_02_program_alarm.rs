mod intcode;

use crate::intcode::eval;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input: String = fs::read_to_string(&args[1]).expect("couldn't read file");
    let mut ints = input
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<usize>()
                .unwrap_or_else(|_| panic!("invalid int {}", s))
        })
        .collect::<Vec<_>>();
    ints[1] = 12;
    ints[2] = 2;
    println!("{}", eval(&mut ints));
}
