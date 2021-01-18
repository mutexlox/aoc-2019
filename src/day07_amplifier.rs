use std::env;
use std::fs;

use itertools::Itertools;

fn max_without_loop(ints: &[i32]) -> i32 {
    let inputs = (0..5).permutations(5);
    let mut max = None;
    for inp in inputs {
        let mut extra_in = 0;
        let mut out = Vec::new();
        for x in inp {
            let in_arr = vec![x, extra_in];
            out = Vec::new();
            let mut mem = ints.to_owned();
            intcode::eval_with_input(&mut mem, &in_arr, &mut out);
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
    max.unwrap()
}

fn max_with_loop(ints: &[i32]) -> i32 {
    let inputs = (5..10).permutations(5);
    // Create a memory and pc for each amp
    let mut mems = Vec::new();
    for _ in 0..5 {
        mems.push((None, ints.to_owned()));
    }
    let mut max = None;
    for inp in inputs {
        mems = Vec::new();
        for _ in 0..5 {
            mems.push((None, ints.to_owned()));
        }
        let mut out: Vec<i32>;
        // start each one; run until they need more input than they have, or halt...
        let mut in_arr = vec![0];
        for (i, x) in inp.iter().enumerate() {
            in_arr.insert(0, *x);
            out = Vec::new();
            let mut pc = mems[i].0;
            pc = intcode::eval_with_input_and_pc(&mut mems[i].1, &in_arr, &mut out, pc);
            mems[i].0 = pc;
            in_arr = out;
        }
        // Then keep running without the additional input until they're done

        let mut finished = false;
        while !finished {
            for item in mems.iter_mut() {
                out = Vec::new();
                let mut pc = item.0;
                pc = intcode::eval_with_input_and_pc(&mut item.1, &in_arr, &mut out, pc);
                if pc.is_none() {
                    // No more loops after this one
                    finished = true;
                }
                item.0 = pc;
                in_arr = out;
            }
        }
        // Finally, check to see if we're larger
        if let Some(x) = max {
            if x < in_arr[0] {
                max = Some(in_arr[0]);
            }
        } else {
            max = Some(in_arr[0]);
        }
    }
    max.unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input: String = fs::read_to_string(&args[1]).expect("couldn't read file");
    let ints = intcode::parse(&input);

    println!("{}", max_without_loop(&ints));
    println!("{}", max_with_loop(&ints));
}
