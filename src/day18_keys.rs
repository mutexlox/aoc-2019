use std::cmp::{Ord, Ordering, Reverse};
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Tile {
    Empty,
    Wall,
    Door(char),
    Key(char),
    Entrance,
}

fn get_neighbors(map: &[Vec<Tile>], i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    if i > 0 {
        out.push((i - 1, j));
    }
    if j > 0 {
        out.push((i, j - 1));
    }
    if i < map.len() - 1 {
        out.push((i + 1, j));
    }
    if j < map[i].len() - 1 {
        out.push((i, j + 1));
    }

    out
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct HeapElem {
    loc: (usize, usize),
    steps: usize,
}

impl Ord for HeapElem {
    fn cmp(&self, other: &Self) -> Ordering {
        self.steps.cmp(&other.steps)
    }
}

impl PartialOrd for HeapElem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// returns list of ((i, j), nsteps) representing all possible moves from the starting |loc|
fn get_all_moves(
    map: &[Vec<Tile>],
    loc: (usize, usize),
    keys: &HashSet<char>,
) -> BinaryHeap<Reverse<HeapElem>> {
    let mut out = BinaryHeap::new();

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((loc, 0));
    while let Some(next) = queue.pop_front() {
        if visited.contains(&next.0) {
            continue;
        }
        visited.insert(next.0);
        for n in get_neighbors(map, next.0 .0, next.0 .1) {
            let st = HeapElem {
                loc: n,
                steps: next.1 + 1,
            };
            match map[n.0][n.1] {
                // Keep exploring
                Tile::Empty | Tile::Entrance => queue.push_back((n, next.1 + 1)),
                // Stop exploring
                Tile::Door(c) => {
                    // Only can open a door if we have the key!
                    if keys.contains(&c) {
                        queue.push_back((n, next.1 + 1));
                    }
                }
                // Stop exploring
                Tile::Key(_) => out.push(Reverse(st)),
                // Nothing to do
                Tile::Wall => {}
            }
        }
    }

    out
}

type StateEntry = (Vec<Vec<Tile>>, (usize, usize));
fn find_minimum_moves_with_keys(
    map: &mut Vec<Vec<Tile>>,
    loc: (usize, usize),
    nkeys: usize,
    keys: &mut HashSet<char>,
    mut min_so_far: usize,
    // previously-seen states mapped to # steps
    states: &mut HashMap<StateEntry, usize>,
    steps: usize,
) -> usize {
    let key = (map.to_owned(), loc);
    if let Some(n) = states.get(&key) {
        // don't bother -- can get here more quickly
        if *n <= steps {
            return min_so_far + 1;
        }
    }
    if steps >= min_so_far {
        // don't bother
        return min_so_far + 1;
    }
    states.insert(key, steps);
    if keys.len() == nkeys {
        return steps;
    }
    // Get all moves
    let moves = get_all_moves(map, loc, keys);
    // then, of those, try each (recursively) and find which is best
    for entry in moves {
        let Reverse(m) = entry;
        let old = map[m.loc.0][m.loc.1];
        if let Tile::Key(c) = old {
            keys.insert(c);
        }
        map[m.loc.0][m.loc.1] = Tile::Empty;
        let steps = find_minimum_moves_with_keys(
            map,
            m.loc,
            nkeys,
            keys,
            min_so_far,
            states,
            steps + m.steps,
        );
        if steps < min_so_far {
            min_so_far = steps;
        }
        if let Tile::Key(c) = old {
            keys.remove(&c);
        }
        map[m.loc.0][m.loc.1] = old;
    }

    assert_ne!(min_so_far, 0);
    min_so_far
}

// Find the minimum number of moves to get all keys starting from |loc|.
fn find_minimum_moves(map: &mut Vec<Vec<Tile>>, loc: (usize, usize), nkeys: usize) -> usize {
    let mut keys = HashSet::new();
    let mut states = HashMap::new();
    find_minimum_moves_with_keys(map, loc, nkeys, &mut keys, usize::MAX, &mut states, 0)
}

type FourStateEntry = (Vec<Vec<Tile>>, Vec<(usize, usize)>);
fn find_minimum_four_with_keys(
    map: &mut Vec<Vec<Tile>>,
    locs: &mut Vec<(usize, usize)>,
    nkeys: usize,
    keys: &mut HashSet<char>,
    mut min_so_far: usize,
    // previously-seen states mapped to # steps
    states: &mut HashMap<FourStateEntry, usize>,
    steps: usize,
) -> usize {
    let key = (map.to_owned(), locs.to_owned());
    if let Some(n) = states.get(&key) {
        // don't bother -- can get here more quickly
        if *n <= steps {
            return min_so_far + 1;
        }
    }
    if steps >= min_so_far {
        // don't bother
        return min_so_far + 1;
    }
    states.insert(key, steps);
    if keys.len() == nkeys {
        return steps;
    }
    // Get all moves
    let mut moves = Vec::new();
    for loc in locs.iter() {
        moves.push(get_all_moves(map, *loc, keys));
    }
    // Repeatedly pick one move from each robot, apply move, then recurse and find which is best
    for (i, robot) in moves.iter().enumerate() {
        for entry in robot {
            let Reverse(m) = entry;
            let old = map[m.loc.0][m.loc.1];
            if let Tile::Key(c) = old {
                keys.insert(c);
            }
            map[m.loc.0][m.loc.1] = Tile::Empty;
            let old_loc = locs[i];
            locs[i] = m.loc;
            let steps = find_minimum_four_with_keys(
                map,
                locs,
                nkeys,
                keys,
                min_so_far,
                states,
                steps + m.steps,
            );
            locs[i] = old_loc;
            if steps < min_so_far {
                min_so_far = steps;
            }
            if let Tile::Key(c) = old {
                keys.remove(&c);
            }
            map[m.loc.0][m.loc.1] = old;
        }
    }

    assert_ne!(min_so_far, 0);
    min_so_far
}

// Find the minimum number of moves from each of the four |locs|.
fn find_minimum_with_four(
    map: &mut Vec<Vec<Tile>>,
    locs: &mut Vec<(usize, usize)>,
    nkeys: usize,
) -> usize {
    let mut keys = HashSet::new();
    let mut states = HashMap::new();
    find_minimum_four_with_keys(map, locs, nkeys, &mut keys, usize::MAX, &mut states, 0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let input = fs::read_to_string(&args[1]).expect("couldn't read file");
    let lines = input.trim().split('\n').map(|s| s.trim());

    let mut map = Vec::new();
    let mut loc = (0, 0);
    let mut nkeys = 0;
    for (i, l) in lines.enumerate() {
        let mut cur = Vec::new();
        for (j, c) in l.chars().enumerate() {
            cur.push(match c {
                '.' => Tile::Empty,
                '#' => Tile::Wall,
                'A'..='Z' => Tile::Door(c),
                'a'..='z' => {
                    nkeys += 1;
                    Tile::Key(c.to_ascii_uppercase())
                }
                '@' => {
                    loc = (i, j);
                    Tile::Entrance
                }
                _ => panic!("invalid char {}", c),
            });
        }
        map.push(cur);
    }

    let mut clone = map.clone();
    println!("{}", find_minimum_moves(&mut clone, loc, nkeys));

    // update map
    map[loc.0 - 1][loc.1 - 1] = Tile::Entrance;
    map[loc.0 - 1][loc.1] = Tile::Wall;
    map[loc.0 - 1][loc.1 + 1] = Tile::Entrance;
    map[loc.0][loc.1 - 1] = Tile::Wall;
    map[loc.0][loc.1] = Tile::Wall;
    map[loc.0][loc.1 + 1] = Tile::Wall;
    map[loc.0 + 1][loc.1 - 1] = Tile::Entrance;
    map[loc.0 + 1][loc.1] = Tile::Wall;
    map[loc.0 + 1][loc.1 + 1] = Tile::Entrance;

    let mut locs = vec![
        (loc.0 - 1, loc.1 - 1),
        (loc.0 - 1, loc.1 + 1),
        (loc.0 + 1, loc.1 - 1),
        (loc.0 + 1, loc.1 + 1),
    ];
    println!("{}", find_minimum_with_four(&mut map, &mut locs, nkeys));
}
