use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input: String = fs::read_to_string(&args[1]).expect("couldn't read file");
    let mut ints = intcode::parse(&input);
    let mut mem = ints.clone();

    let mut inputs = vec![1];
    let mut outputs = Vec::new();

    intcode::eval_with_input(&mut mem, &inputs, &mut outputs);
    for (i, out) in outputs.iter().enumerate() {
        if *out != 0 {
            if i != outputs.len() - 1 {
                panic!("failed at index {}; code {}", i, out);
            } else {
                println!("{}", out)
            }
        }
    }
    inputs = vec![5];
    outputs = Vec::new();
    intcode::eval_with_input(&mut ints, &inputs, &mut outputs);
    println!("{}", outputs[0]);
}
