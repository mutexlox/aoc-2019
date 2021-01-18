use std::env;
use std::fs;

use itertools::Itertools;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input: String = fs::read_to_string(&args[1]).expect("couldn't read file");
    let ints = intcode::parse(&input);
    let mut mem = ints.clone();

    let inputs = (0..5).permutations(5);
    let mut max = None;
    for inp in inputs {
        let mut extra_in = 0;
        let mut out = Vec::new();
        for x in inp {
            let in_arr = vec![x, extra_in];
            out = Vec::new();
            intcode::eval_with_input(&mut mem, &in_arr, &mut out);
            mem = ints.clone();
            extra_in = out[0];
        }
        if let Some(x) = max {
            if x < out[0] {
                max = Some(out[0]);
            }
        } else {
            max = Some(out[0]);
        }
    }

    println!("{}", max.unwrap());
}
