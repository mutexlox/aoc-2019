use std::collections::HashMap;
use std::env;
use std::fs;
use std::sync::mpsc;

fn num_affected_in(ints: &HashMap<usize, i64>, x: i64, y: i64) -> usize {
    let mut out = 0;

    for i in 0..x {
        for j in 0..y {
            let (in_sender, in_receiver) = mpsc::channel();
            let (out_sender, out_receiver) = mpsc::channel();

            in_sender.send(i).unwrap();
            in_sender.send(j).unwrap();

            let mut mem = ints.to_owned();
            intcode::eval_with_input(&mut mem, in_receiver, out_sender.clone());

            let affected = out_receiver.recv().unwrap();
            if affected == 1 {
                out += 1;
            }
        }
    }

    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let input = fs::read_to_string(&args[1]).expect("couldn't read file");
    let ints = intcode::parse(&input);

    println!("{}", num_affected_in(&ints, 50, 50));
}
