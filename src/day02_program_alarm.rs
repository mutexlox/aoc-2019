mod intcode;

use crate::intcode::eval;
use std::env;
use std::fs;

fn find_noun_and_verb(ints: &[i32]) -> (i32, i32) {
    for i in 0..100 {
        for j in 0..100 {
            let mut c = ints.to_owned();
            c[1] = i;
            c[2] = j;
            if eval(&mut c) == 19690720 {
                return (i, j);
            }
        }
    }
    panic!("no pair found");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input: String = fs::read_to_string(&args[1]).expect("couldn't read file");
    let ints = intcode::parse(&input);
    let mut mem = ints.clone();
    mem[1] = 12;
    mem[2] = 2;

    println!("{}", eval(&mut mem));
    let (noun, verb) = find_noun_and_verb(&ints);
    println!("{}", 100 * noun + verb);
}
