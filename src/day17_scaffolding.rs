use std::collections::HashMap;
use std::env;
use std::fs;
use std::sync::mpsc;
use std::thread;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Tile {
    Empty,
    Scaffolding,
    Robot(Dir),
    FallenRobot,
}

fn get_neighbors(map: &[Vec<Tile>], i: usize, j: usize) -> Vec<Tile> {
    let mut out = Vec::new();
    if i > 0 {
        out.push(map[i - 1][j]);
    }
    if j > 0 {
        out.push(map[i][j - 1]);
    }
    if i < map.len() - 1 {
        out.push(map[i + 1][j]);
    }
    if j < map[i].len() - 1 {
        out.push(map[i][j + 1]);
    }

    out
}

fn sum_alignment_params(ints: &HashMap<usize, i64>) -> usize {
    let (_, in_receiver) = mpsc::channel();
    let (out_sender, out_reciever) = mpsc::channel();

    // Run the program
    let mut mem = ints.to_owned();
    intcode::eval_with_input(&mut mem, in_receiver, out_sender);

    let out_str = out_reciever
        .iter()
        .map(|i| std::char::from_u32(i as u32).unwrap())
        .collect::<String>();

    let mut map = Vec::new();
    for line in out_str.trim().split('\n') {
        let mut inner = Vec::new();
        for c in line.chars() {
            inner.push(match c {
                '.' => Tile::Empty,
                '#' => Tile::Scaffolding,
                '>' => Tile::Robot(Dir::Right),
                '^' => Tile::Robot(Dir::Up),
                '<' => Tile::Robot(Dir::Left),
                'v' | 'V' => Tile::Robot(Dir::Down),
                'X' => Tile::FallenRobot,
                _ => panic!("invalid character {}", c),
            });
        }
        map.push(inner);
    }

    let mut sum = 0;
    for i in 0..map.len() {
        for j in 0..map[0].len() {
            match map[i][j] {
                Tile::Scaffolding | Tile::Robot(_) => {
                    let mut all_scaffold = true;
                    for n in get_neighbors(&map, i, j) {
                        if n == Tile::Empty || n == Tile::FallenRobot {
                            all_scaffold = false;
                            break;
                        }
                    }
                    if all_scaffold {
                        sum += i * j;
                    }
                }
                _ => continue,
            }
        }
    }

    sum
}

fn explore_all_scaffolding(ints: &HashMap<usize, i64>) -> i64 {
    // manually-computed path
    // L,12,R,8,L,6,R,8,L,6,R,8,L,12,L,12,R,8,L,12,R,8,L,6,R,8,L,6,L,12,R,8,L,6,R,8,L,6,R,8,L,12,L,12,R,8,L,6,R,6,L,12,R,8,L,12,L,12,R,8,L,6,R,6,L,12,L,6,R,6,L,12,R,8,L,12,L,12,R,8
    let main = "A,B,A,A,B,C,B,C,C,B";
    let func_a = "L,12,R,8,L,6,R,8,L,6";
    let func_b = "R,8,L,12,L,12,R,8";
    let func_c = "L,6,R,6,L,12";

    let input_str = main.to_owned() + "\n" + func_a + "\n" + func_b + "\n" + func_c + "\n" + "n\n";
    let input_ints = input_str.chars().map(|c| c as i64);

    let mut mem = ints.to_owned();
    mem.insert(0, 2); // set to prompt for movement rules

    let (in_sender, in_receiver) = mpsc::channel();
    let (out_sender, out_receiver) = mpsc::channel();
    thread::spawn(move || {
        intcode::eval_with_input(&mut mem, in_receiver, out_sender.clone());
    });

    for i in input_ints {
        in_sender.send(i).unwrap();
    }

    let mut out = out_receiver.recv().unwrap();
    while out < 128 {
        out = out_receiver.recv().unwrap();
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let input = fs::read_to_string(&args[1]).expect("couldn't read file");
    let ints = intcode::parse(&input);

    println!("{}", sum_alignment_params(&ints));
    println!("{}", explore_all_scaffolding(&ints));
}
