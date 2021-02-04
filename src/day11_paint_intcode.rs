use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::sync::mpsc;
use std::thread;

fn count_panels_painted(ints: &HashMap<usize, i64>, tiles: &mut HashSet<(i64, i64)>) -> usize {
    let (in_sender, in_receiver) = mpsc::channel();
    let (out_sender, out_reciever) = mpsc::channel();

    // Start up the program
    let mut mem = ints.to_owned();
    let child = thread::spawn(move || {
        intcode::eval_with_input(&mut mem, in_receiver, out_sender.clone());
        out_sender.send(-1).unwrap();
    });

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

fn print_tiles(tiles: &HashSet<(i64, i64)>) {
    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;
    for (i, (x, y)) in tiles.iter().enumerate() {
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
    for x in min_x..max_x + 1 {
        for y in min_y..max_y + 1 {
            if tiles.contains(&(x, y)) {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let input = fs::read_to_string(&args[1]).expect("couldn't read file");
    let ints = intcode::parse(&input);

    let mut tiles = HashSet::new();
    println!("{}", count_panels_painted(&ints, &mut tiles));

    tiles = HashSet::new();
    tiles.insert((0, 0));
    count_panels_painted(&ints, &mut tiles);
    print_tiles(&tiles);
}
