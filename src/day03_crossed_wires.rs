use std::collections::HashMap;

use std::env;
use std::fs;

fn find_closest_intersection(lines: &[Vec<&str>]) -> (i32, i32) {
    let mut map = HashMap::<(i32, i32), (usize, i32)>::new();
    let mut closest = -1;
    let mut fewest_steps = -1;
    for (i, wire) in lines.iter().enumerate() {
        let mut x: i32 = 0;
        let mut y: i32 = 0;
        let mut steps: i32 = 0;
        for op in wire {
            let d = op.chars().next().unwrap();
            let count = op[1..].parse::<i32>().unwrap();
            for _j in 0..count {
                if map.contains_key(&(x, y)) && map[&(x, y)].0 != i && !(x == 0 && y == 0) {
                    if fewest_steps == -1 || (map[&(x, y)].1) + steps < fewest_steps {
                        fewest_steps = (map[&(x, y)].1) + steps;
                    }
                    if closest == -1 || (x.abs() + y.abs()) < closest {
                        closest = x.abs() + y.abs();
                    }
                } else {
                    map.insert((x, y), (i, steps));
                }

                steps += 1;
                match d {
                    'R' => y += 1,
                    'L' => y -= 1,
                    'U' => x -= 1,
                    'D' => x += 1,
                    _ => panic!("unexpected code {}", d),
                }
            }
        }
    }
    (closest, fewest_steps)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input: String = fs::read_to_string(&args[1]).expect("couldn't read file");
    let ops = input
        .split_whitespace()
        .map(|s| s.split(',').collect::<Vec<_>>())
        .collect::<Vec<Vec<&str>>>();
    let (closest, shortest) = find_closest_intersection(&ops);
    println!("{}, {}", closest, shortest);
}
