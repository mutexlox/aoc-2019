use std::env;
use std::fs;
use std::sync::mpsc;

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input: String = fs::read_to_string(&args[1]).expect("couldn't read file");
    let mut ints = intcode::parse(&input);
    let mut mem = ints.clone();

    let (in_sender, in_reciever) = mpsc::channel();
    in_sender.send(1).unwrap();
    let (out_sender, out_reciever) = mpsc::channel();

    intcode::eval_with_input(&mut mem, in_reciever, out_sender);
    for out in out_reciever.iter() {
        if out != 0 {
            println!("{}", out)
        }
    }
    let (in_sender, in_reciever) = mpsc::channel();
    let (out_sender, out_reciever) = mpsc::channel();
    in_sender.send(5).unwrap();
    intcode::eval_with_input(&mut ints, in_reciever, out_sender);
    println!("{}", out_reciever.recv().unwrap());
}
