use std::collections::{HashSet, VecDeque};
use std::env;
use std::fs;

#[derive(Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Door(char),
    Key(char),
    Me,
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

// returns list of ((i, j), nsteps) representing all possible moves from the starting |loc|
fn get_all_moves(map: &Vec<Vec<Tile>>, loc: (usize, usize)) -> Vec<((usize, usize), usize)> {
    let mut out = Vec::new();

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    queue.push_back((loc, 0));
    while let Some(next) = queue.pop_front() {
        if visited.contains(&next.0) {
            continue;
        }
        visited.insert(next.0);
        for n in get_neighbors(map, next.0.0, next.0.1) {
            match map[n.0][n.1] {
                // Keep exploring
                Tile::Empty | Tile::Me => queue.push_back((n, next.1 + 1)),
                // Stop exploring - for now
                Tile::Door(_) | Tile::Key(_) => out.push((n, next.1 + 1)),
                // Nothing to do
                Tile::Wall => {}
            }
        }
    }

    out
}

// TODO: make this faster
// ideas:
//   1) cache BFSs across |get_all_moves| calls -- may be doing repeated work there
//   2) cut off search when steps > min so far
fn find_minimum_moves_with_keys(map: &mut Vec<Vec<Tile>>, loc: (usize, usize), nkeys: usize,
                                keys: &mut HashSet<char>) -> usize {
    let mut min_steps = 0;
    if keys.len() == nkeys {
        return 0;
    }
    // Get all moves, except those where we don't have the key.
    let moves = get_all_moves(map, loc).iter().filter(|((i, j), _)| {
        if let Tile::Door(c) = map[*i][*j] {
            return keys.contains(&c);
        }
        return true;
    }).cloned().collect::<Vec<_>>();
    // then, of those, try each (recursively) and find which is best
    for m in moves {
        let old = map[m.0.0][m.0.1];
        if let Tile::Key(c) = old {
            keys.insert(c);
        }
        map[m.0.0][m.0.1] = Tile::Empty;
        let steps = find_minimum_moves_with_keys(map, m.0, nkeys, keys) + m.1;
        if min_steps == 0  || steps < min_steps {
            min_steps = steps;
        }
        if let Tile::Key(c) = old {
            keys.remove(&c);
        }
        map[m.0.0][m.0.1] = old;
    }

    assert_ne!(min_steps, 0);
    min_steps
}

// Find the minimum number of moves to get all keys starting from |loc|.
fn find_minimum_moves(map: &mut Vec<Vec<Tile>>, loc: (usize, usize), nkeys: usize) -> usize {
    let mut keys = HashSet::new();
    find_minimum_moves_with_keys(map, loc, nkeys, &mut keys)
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
                    Tile::Me
                }
                _ => panic!("invalid char {}", c),
            });
        }
        map.push(cur);
    }
    println!("{:?}", find_minimum_moves(&mut map, loc, nkeys));
}