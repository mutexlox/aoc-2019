use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;
use std::sync::mpsc;
use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Tile {
    Wall = 0,
    Empty = 1,
    Oxygen = 2,
}

fn get_inverse(dir: i64) -> i64 {
    match dir {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        _ => panic!("invalid dir {}", dir),
    }
}

fn get_neighbors(loc: (i32, i32)) -> Vec<(i32, i32)> {
    vec![
        (loc.0 - 1, loc.1),
        (loc.0 + 1, loc.1),
        (loc.0, loc.1 - 1),
        (loc.0, loc.1 + 1),
    ]
}

// build up the map with a dfs using the program, because bfs using the program-provided interface
// is cumbersome and complex
fn build_map(
    loc: (i32, i32),
    sender: &mpsc::Sender<i64>,
    receiver: &mpsc::Receiver<i64>,
    map: &mut HashMap<(i32, i32), Tile>,
) {
    let neighbors = get_neighbors(loc);
    for next in &neighbors {
        if map.contains_key(&next) {
            continue;
        }
        let delta = (next.0 - loc.0, next.1 - loc.1);
        let dir = match delta {
            (-1, 0) => 1, // north
            (1, 0) => 2,  // south
            (0, -1) => 3, // west
            (0, 1) => 4,  // east
            _ => panic!("attempt to move by more than 1: {:?}", delta),
        };
        // move in that direction
        sender.send(dir).unwrap();
        let status = receiver.recv().unwrap();
        match status {
            0 => {
                // hit a wall -- nowhere to go.
                map.insert(*next, Tile::Wall);
            }
            1 => {
                map.insert(*next, Tile::Empty);
            }
            2 => {
                map.insert(*next, Tile::Oxygen);
            }
            _ => panic!("invalid status {}", status),
        };
        if status != 0 {
            // only recurse if we moved
            // recurse
            build_map(*next, sender, receiver, map);
            // move back
            sender.send(get_inverse(dir)).unwrap();
            // we were here before, so shouldn't be at a wall
            assert_ne!(receiver.recv().unwrap(), 0);
        }
    }
}

fn fewest_movements(map: &HashMap<(i32, i32), Tile>) -> (i32, (i32, i32)) {
    // now we have a map. BFS from (0, 0) to find the oxygen.
    let mut queue = VecDeque::new();
    queue.push_back(((0, 0), 0));
    let mut visited = HashSet::new();
    let mut total_steps = -1;
    let mut oxygen_loc = (0, 0);
    while let Some((next, steps)) = queue.pop_front() {
        if visited.contains(&next) {
            continue;
        }
        visited.insert(next);
        let tile = match map.get(&next) {
            Some(t) => t,
            None => continue,
        };
        if *tile == Tile::Oxygen {
            total_steps = steps;
            oxygen_loc = next;
        } else if *tile == Tile::Empty {
            // add neighbors
            let neighbors = get_neighbors(next);
            for n in &neighbors {
                queue.push_back((*n, steps + 1));
            }
        }
    }
    (total_steps, oxygen_loc)
}

fn max_depth_from_oxygen(map: &HashMap<(i32, i32), Tile>, oxygen_loc: (i32, i32)) -> i32 {
    // do another BFS from here to find max depth.
    let mut queue = VecDeque::new();
    queue.push_back((oxygen_loc, 0));
    let mut visited = HashSet::new();
    let mut max_depth = -1;
    while let Some((next, steps)) = queue.pop_front() {
        if visited.contains(&next) {
            continue;
        }
        visited.insert(next);
        match map.get(&next) {
            Some(Tile::Empty) | Some(Tile::Oxygen) => {
                // add neighbors
                let neighbors = get_neighbors(next);
                for n in &neighbors {
                    queue.push_back((*n, steps + 1));
                }
                if steps > max_depth {
                    max_depth = steps;
                }
            }
            _ => continue,
        }
    }
    max_depth
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let input = fs::read_to_string(&args[1]).expect("couldn't read file");
    let mut ints = intcode::parse(&input);

    let (in_sender, in_receiver) = mpsc::channel();
    let (out_sender, out_receiver) = mpsc::channel();

    thread::spawn(move || {
        intcode::eval_with_input(&mut ints, in_receiver, out_sender.clone());
    });

    let loc = (0, 0);
    let mut map = HashMap::new();
    map.insert(loc, Tile::Empty);
    build_map(loc, &in_sender, &out_receiver, &mut map);

    let (steps, oxygen_loc) = fewest_movements(&map);
    println!("{}", steps);
    println!("{}", max_depth_from_oxygen(&map, oxygen_loc));
}
