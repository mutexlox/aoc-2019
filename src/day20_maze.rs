use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

#[derive(PartialEq, Eq, Clone, Debug)]
struct Graph {
    adjacency: HashMap<(usize, usize), Vec<(usize, usize)>>,
    entrance: (usize, usize),
    exit: (usize, usize),
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Tile {
    Wall,
    Empty,
    Labeled(String),
}

fn get_label(i: usize, j: usize, lines: &Vec<Vec<char>>) -> Option<String> {
    if i > 0 && lines[i - 1][j].is_ascii_uppercase() {
        return Some(format!("{}{}", lines[i - 2][j], lines[i - 1][j]));
    } else if i < lines.len() && lines[i + 1][j].is_ascii_uppercase() {
        return Some(format!("{}{}", lines[i + 1][j], lines[i + 2][j]));
    } else if j > 0 && lines[i][j - 1].is_ascii_uppercase() {
        return Some(format!("{}{}", lines[i][j - 2], lines[i][j - 1]));
    } else if j < lines[i].len() && lines[i][j + 1].is_ascii_uppercase() {
        return Some(format!("{}{}", lines[i][j + 1], lines[i][j + 2]));
    }
    None
}

fn build_intermediate(lines: &Vec<Vec<char>>) -> Vec<Vec<Tile>> {
    let mut out = Vec::new();
    for (i, row) in lines.iter().enumerate() {
        let mut next = Vec::new();
        for (j, c) in row.iter().enumerate() {
            match c {
                '#' => next.push(Tile::Wall),
                '.' => {
                    if let Some(l) = get_label(i, j, lines) {
                        next.push(Tile::Labeled(l));
                    } else {
                        next.push(Tile::Empty);
                    }
                }
                // treat " " and labels as walls so we don't get misaligned
                _ => next.push(Tile::Wall),
            }
        }
        out.push(next);
    }

    out
}

// Get all immediate neighbors of a tile (i.e. not following portals)
fn get_immediate_neighbors(
    intermediate: &Vec<Vec<Tile>>,
    i: usize,
    j: usize,
) -> Vec<(usize, usize)> {
    let mut out_pos = Vec::new();
    if i > 0 {
        out_pos.push((i - 1, j));
    }
    if i < intermediate.len() - 1 {
        out_pos.push((i + 1, j));
    }
    if j < intermediate[i].len() - 1 {
        out_pos.push((i, j + 1));
    }
    if j > 0 {
        out_pos.push((i, j - 1));
    }
    out_pos
        .into_iter()
        .filter(|(i, j)| intermediate[*i][*j] != Tile::Wall)
        .collect()
}

fn find_portal_match(intermediate: &Vec<Vec<Tile>>, i: usize, j: usize) -> (usize, usize) {
    for (k, row) in intermediate.iter().enumerate() {
        for (l, t) in row.iter().enumerate() {
            if i == k && j == l {
                continue;
            }
            if intermediate[i][j] == *t {
                return (k, l);
            }
        }
    }

    panic!("no match for {}, {} ({:?})", i, j, intermediate[i][j]);
}

fn build_graph(lines: &Vec<Vec<char>>) -> Graph {
    let mut out = Graph {
        adjacency: HashMap::new(),
        entrance: (0, 0),
        exit: (0, 0),
    };
    let intermediate = build_intermediate(lines);

    for (i, row) in intermediate.iter().enumerate() {
        for (j, t) in row.iter().enumerate() {
            match t {
                Tile::Empty => {
                    out.adjacency
                        .insert((i, j), get_immediate_neighbors(&intermediate, i, j));
                }
                Tile::Labeled(s) => {
                    let mut neighbors = get_immediate_neighbors(&intermediate, i, j);
                    match s.as_str() {
                        "AA" => out.entrance = (i, j),
                        "ZZ" => out.exit = (i, j),
                        _ => neighbors.push(find_portal_match(&intermediate, i, j)),
                    }
                    out.adjacency.insert((i, j), neighbors);
                }
                Tile::Wall => {}
            }
        }
    }

    out
}

fn min_steps(g: &Graph) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((g.entrance, 0));
    while let Some((coord, steps)) = queue.pop_front() {
        if coord == g.exit {
            return steps;
        }
        if visited.contains(&coord) {
            continue;
        }
        visited.insert(coord);
        for n in &g.adjacency[&coord] {
            queue.push_back((*n, steps + 1));
        }
    }
    0
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let input = fs::read_to_string(&args[1]).expect("couldn't read file");
    let lines = input
        .trim_matches('\n')
        .split('\n')
        .map(|s| s.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let graph = build_graph(&lines);
    println!("{}", min_steps(&graph));
}
