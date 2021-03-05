use std::collections::{HashMap, HashSet, VecDeque};
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

// Finds the top-left coordinates of a square of dim x dim. Returns x * 10_000 + y
fn start_of_square(ints: &HashMap<usize, i64>, dim: i64) -> i64 {
    let mut filled = HashSet::new();
    let mut checked = HashSet::new();

    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    queue.push_back((4, 3));
    while let Some((x, y)) = queue.pop_front() {
        // 1012, 0834 is too high (per failed guess), so don't need to explor past there
        if x > 1_112 && y > 934 {
            break;
        }
        if checked.contains(&(x, y)) {
            continue;
        }
        checked.insert((x, y));
        let (in_sender, in_receiver) = mpsc::channel();
        let (out_sender, out_receiver) = mpsc::channel();

        in_sender.send(x).unwrap();
        in_sender.send(y).unwrap();

        let mut mem = ints.to_owned();
        intcode::eval_with_input(&mut mem, in_receiver, out_sender.clone());

        let affected = out_receiver.recv().unwrap() == 1;
        if affected {
            filled.insert((x, y));
            queue.push_back((x + 1, y));
            queue.push_back((x, y + 1));
            queue.push_back((x + 1, y + 1));
        }
    }

    for x in 0..2_000 {
        for y in 0..1_000 {
            if !filled.contains(&(x, y)) {
                continue;
            }
            let mut works = true;
            for j in y..y + dim {
                if !filled.contains(&(x, j)) {
                    works = false;
                    break;
                }
            }
            if works {
                for i in x..x + dim {
                    if !filled.contains(&(i, y)) {
                        works = false;
                        break;
                    }
                }
            }
            if works {
                return x * 10_000 + y;
            }
        }
    }

    -1
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let input = fs::read_to_string(&args[1]).expect("couldn't read file");
    let ints = intcode::parse(&input);

    println!("{}", num_affected_in(&ints, 50, 50));
    println!("{}", start_of_square(&ints, 100));
}
