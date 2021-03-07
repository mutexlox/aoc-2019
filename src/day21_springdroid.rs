use std::collections::HashMap;
use std::env;
use std::fs;
use std::sync::mpsc;

fn run_springdroid_pgrm(ints: &HashMap<usize, i64>, program: &str) -> i64 {
    let (in_sender, in_receiver) = mpsc::channel();
    let (out_sender, out_receiver) = mpsc::channel();
    let input = program.chars().map(|c| c as i64).collect::<Vec<_>>();
    for i in input {
        in_sender.send(i).unwrap();
    }
    let mut mem = ints.clone();
    intcode::eval_with_input(&mut mem, in_receiver, out_sender);

    let mut out = out_receiver.recv().unwrap();
    while out < (i8::MAX as i64) {
        print!("{}", out as u8 as char);
        out = out_receiver.recv().unwrap();
    }

    out
}

fn amount_hull_damage(ints: &HashMap<usize, i64>) -> i64 {
    // logic copied from reddit:
    // Either we have to jump early or we jump when A is hole. We jump early when B or C is
    // hole and D (where we land if we jump) is ground.
    // so logic is: J := !A | ((!B | !C) & D)
    let program = r#"NOT B J
NOT C T
OR T J
AND D J
NOT A T
OR T J
WALK
"#;
    run_springdroid_pgrm(ints, &program)
}

fn amount_hull_damage_2(ints: &HashMap<usize, i64>) -> i64 {
    // add "AND H J" to jump early part; if we can't double-jump don't jump early
    let program = r#"NOT B J
NOT C T
OR T J
AND D J
AND H J
NOT A T
OR T J
RUN
"#;
    run_springdroid_pgrm(ints, &program)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let input = fs::read_to_string(&args[1]).expect("couldn't read file");
    let ints = intcode::parse(&input);

    println!("{}", amount_hull_damage(&ints));
    println!("{}", amount_hull_damage_2(&ints));
}
