use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::sync::mpsc;
use std::thread;

fn count_panels_painted(ints: &HashMap<usize, i64>) -> usize {
    let (in_sender, in_receiver) = mpsc::channel();
    let (out_sender, out_reciever) = mpsc::channel();

    // Start up the program
    let mut mem = ints.to_owned();
    let child = thread::spawn(move || {
        intcode::eval_with_input(&mut mem, in_receiver, out_sender.clone());
        out_sender.send(-1).unwrap();
    });

    let mut tiles = HashSet::new();
    let mut loc_x = 0;
    let mut loc_y = 0;
    let mut face = (0, 1);

    let mut painted = HashSet::new();
    loop {
        // Send current color
        in_sender
            .send(if tiles.contains(&(loc_x, loc_y)) {
                1
            } else {
                0
            })
            .unwrap();
        // recieve new color
        let color = out_reciever.recv().unwrap();
        if color == -1 {
            // special sentinel value to indicate end
            break;
        } else if color == 1 {
            tiles.insert((loc_x, loc_y));
        } else {
            tiles.remove(&(loc_x, loc_y));
        }
        // count number of tiles painted at least once.
        painted.insert((loc_x, loc_y));

        let turn = out_reciever.recv().unwrap();
        if turn == 0 {
            // (0, 1) -> (-1, 0); (-1, 0) -> (0, -1); (0, -1) -> (1, 0); (1, 0) -> (0, 1)
            let face_x = -face.1;
            let face_y = face.0;
            face = (face_x, face_y);
        } else {
            assert_eq!(turn, 1);
            // (0, 1) -> (1, 0); (1, 0) -> (0, -1); (0, -1) -> (-1, 0); (-1, 0) -> (0, 1)
            let face_x = face.1;
            let face_y = -face.0;
            face = (face_x, face_y);
        }
        loc_x += face.0;
        loc_y += face.1;
    }

    child.join().unwrap();
    painted.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let input = fs::read_to_string(&args[1]).expect("couldn't read file");
    let ints = intcode::parse(&input);

    println!("{}", count_panels_painted(&ints));
}
