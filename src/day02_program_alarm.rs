mod intcode;

use crate::intcode::eval;
use std::collections::HashMap;
use std::env;
use std::fs;

fn find_noun_and_verb(ints: &HashMap<usize, i64>) -> (i64, i64) {
    for i in 0..100 {
        for j in 0..100 {
            let mut c = ints.to_owned();
            c.insert(1, i);
            c.insert(2, j);
            eval(&mut c);
            if c[&0] == 19690720 {
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
    mem.insert(1, 12);
    mem.insert(2, 2);

    eval(&mut mem);
    println!("{}", mem[&0]);
    let (noun, verb) = find_noun_and_verb(&ints);
    println!("{}", 100 * noun + verb);
}
