use std::cmp::Ordering;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn count_blocks_at_end(ints: &HashMap<usize, i64>) -> usize {
    let (_, in_receiver) = mpsc::channel();
    let (out_sender, out_reciever) = mpsc::channel();

    // Run the program
    let mut mem = ints.to_owned();
    intcode::eval_with_input(&mut mem, in_receiver, out_sender);

    let mut grid = HashMap::new();
    let outs = out_reciever.try_iter().collect::<Vec<_>>();

    let mut i = 0;
    while i < outs.len() {
        let x = outs[i];
        let y = outs[i + 1];
        let val = outs[i + 2];
        grid.insert((x, y), val);
        i += 3;
    }

    grid.values().filter(|x| **x == 2).count()
}

fn display_grid(grid: &HashMap<(i64, i64), i64>) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for (i, (x, y)) in grid.keys().enumerate() {
        if i == 0 {
            min_x = *x;
            max_x = *x;
            min_y = *y;
            max_y = *y;
        }
        if min_x > *x {
            min_x = *x;
        }
        if max_x < *x {
            max_x = *x;
        }
        if min_y > *y {
            min_y = *y;
        }
        if max_y < *y {
            max_y = *y;
        }
    }

    for y in min_y..max_y + 1 {
        for x in min_x..max_x + 1 {
            match grid.get(&(x, y)).unwrap_or(&0) {
                0 => print!(" "),
                1 => print!("W"),
                2 => print!("#"),
                3 => print!("_"),
                4 => print!("0"),
                _ => panic!("invalid value"),
            };
        }
        println!();
    }
}

fn play_game(ints: &HashMap<usize, i64>) -> i64 {
    let mut mem = ints.to_owned();

    // insert quarters
    mem.insert(0, 2);

    let (in_sender, in_receiver) = mpsc::channel();
    let (out_sender, out_reciever) = mpsc::channel();

    let done_sentinel = -100;
    // Run the program
    let child = thread::spawn(move || {
        intcode::eval_with_input(&mut mem, in_receiver, out_sender.clone());
        out_sender.send(done_sentinel).unwrap();
    });

    let mut grid = HashMap::new();

    let mut score = 0;
    let mut ball_x = -1;
    let mut paddle_x = -1;
    loop {
        let x_or = out_reciever.recv_timeout(Duration::from_millis(1));
        let x = match x_or {
            Ok(v) => v,
            Err(_) => {
                match paddle_x.cmp(&ball_x) {
                    Ordering::Less => in_sender.send(1).unwrap(), // tilt right
                    Ordering::Greater => in_sender.send(-1).unwrap(), // tilt left
                    Ordering::Equal => in_sender.send(0).unwrap(), // tilt right
                }
                continue;
            }
        };
        if x == done_sentinel {
            // done
            break;
        }
        let y = out_reciever.recv().unwrap();
        let val = out_reciever.recv().unwrap();
        if x == -1 && y == 0 {
            score = val;
        } else {
            grid.insert((x, y), val);
            if val == 4 {
                ball_x = x;
            } else if val == 3 {
                paddle_x = x;
            }
        }
    }
    child.join().unwrap();
    display_grid(&grid);
    score
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let input = fs::read_to_string(&args[1]).expect("couldn't read file");
    let ints = intcode::parse(&input);

    println!("{}", count_blocks_at_end(&ints));
    println!("{}", play_game(&ints));
}
