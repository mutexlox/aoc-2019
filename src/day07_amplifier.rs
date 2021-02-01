use std::env;
use std::fs;
use std::sync::mpsc;
use std::sync::{Arc, Barrier};
use std::thread;

use itertools::Itertools;

fn max_without_loop(ints: &[i32]) -> i32 {
    let inputs = (0..5).permutations(5);
    let mut max = None;
    for inp in inputs {
        let mut extra_in = 0;

        for x in inp {
            let (in_sender, in_reciever) = mpsc::channel();
            in_sender.send(x).unwrap();
            in_sender.send(extra_in).unwrap();
            let (out_sender, out_reciever) = mpsc::channel();

            let mut mem = ints.to_owned();
            intcode::eval_with_input(&mut mem, in_reciever, out_sender);
            extra_in = out_reciever.recv().unwrap();
        }
        if let Some(x) = max {
            if x < extra_in {
                max = Some(extra_in);
            }
        } else {
            max = Some(extra_in);
        }
    }
    max.unwrap()
}

fn max_with_loop(ints: &[i32]) -> i32 {
    let inputs = (5..10).permutations(5);

    let mut max = None;
    for inp in inputs {
        let barrier = Arc::new(Barrier::new(5));

        // start each one; hook previous up to next
        let (mut sender, mut receiver) = mpsc::channel();
        let sender_4 = sender.clone();
        let mut children = Vec::new();
        for (i, x) in inp.iter().enumerate() {
            // Send ID
            sender.send(*x).unwrap();
            if i == 0 {
                // Send 0 to first amp...
                sender.send(0).unwrap();
            }
            let (next_sender, next_receiver) = mpsc::channel();
            let mut mem = ints.to_owned();
            let inner_sender = if i == 4 {
                sender_4.clone()
            } else {
                next_sender.clone()
            };

            let c = Arc::clone(&barrier);

            children.push(
                thread::Builder::new()
                    .name(format!("{}", i))
                    .spawn(move || {
                        // Don't start executing (and sending inputs) until all threads have received
                        // their first input
                        c.wait();
                        let reader = intcode::eval_with_input(&mut mem, receiver, inner_sender);
                        if i == 0 {
                            Some(reader.recv().unwrap())
                        } else {
                            None
                        }
                    })
                    .unwrap(),
            );
            receiver = next_receiver;
            sender = next_sender.clone();
        }
        let mut out = 0;
        for c in children {
            let res = c.join().unwrap();
            if let Some(x) = res {
                out = x;
            }
        }
        // Finally, check to see if we're larger
        if let Some(x) = max {
            if x < out {
                max = Some(out);
            }
        } else {
            max = Some(out);
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
