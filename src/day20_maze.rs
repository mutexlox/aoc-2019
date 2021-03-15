use std::collections::{HashMap, HashSet, VecDeque};
use std::env;
use std::fs;

#[derive(PartialEq, Eq, Clone, Debug)]
struct GraphPt1 {
    adjacency: HashMap<(usize, usize), Vec<(usize, usize)>>,
    entrance: (usize, usize),
    exit: (usize, usize),
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Tile {
    Wall,
    Empty,
    LabeledInner(String),
    LabeledOuter(String),
}

fn get_label(i: usize, j: usize, lines: &[Vec<char>]) -> Option<String> {
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

fn build_intermediate(lines: &[Vec<char>]) -> Vec<Vec<Tile>> {
    let mut out = Vec::new();
    for (i, row) in lines.iter().enumerate() {
        let mut next = Vec::new();
        for (j, c) in row.iter().enumerate() {
            match c {
                '#' => next.push(Tile::Wall),
                '.' => {
                    if let Some(l) = get_label(i, j, lines) {
                        if i <= 2 || j <= 2 || i >= lines.len() - 3 || j >= lines[i].len() - 3 {
                            next.push(Tile::LabeledOuter(l));
                        } else {
                            next.push(Tile::LabeledInner(l));
                        }
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
fn get_immediate_neighbors(intermediate: &[Vec<Tile>], i: usize, j: usize) -> Vec<(usize, usize)> {
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

fn find_portal_match(intermediate: &[Vec<Tile>], i: usize, j: usize) -> (usize, usize) {
    let label = match &intermediate[i][j] {
        Tile::LabeledOuter(s) | Tile::LabeledInner(s) => s.clone(),
        _ => panic!("can't match tile {:?} at {}, {}", intermediate[i][j], i, j),
    };
    for (k, row) in intermediate.iter().enumerate() {
        for (l, t) in row.iter().enumerate() {
            if i == k && j == l {
                continue;
            }
            match &t {
                Tile::LabeledOuter(s) | Tile::LabeledInner(s) => {
                    if *s == label {
                        return (k, l);
                    }
                }
                _ => {}
            };
        }
    }

    panic!("no match for {}, {} ({:?})", i, j, intermediate[i][j]);
}

fn build_graph_pt1(lines: &[Vec<char>]) -> GraphPt1 {
    let mut out = GraphPt1 {
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
                Tile::LabeledInner(s) | Tile::LabeledOuter(s) => {
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

fn min_steps_pt1(g: &GraphPt1) -> usize {
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

#[derive(PartialEq, Eq, Clone, Debug)]
struct Pt2Adjacency {
    nodes: Vec<(usize, usize)>,
    in_portal: Option<(usize, usize)>,
    out_portal: Option<(usize, usize)>,
}

#[derive(PartialEq, Eq, Clone, Debug)]
struct GraphPt2 {
    adjacency: HashMap<(usize, usize), Pt2Adjacency>,
    entrance: (usize, usize),
    exit: (usize, usize),
}

fn build_graph_pt2(lines: &[Vec<char>]) -> GraphPt2 {
    let mut out = GraphPt2 {
        adjacency: HashMap::new(),
        entrance: (0, 0),
        exit: (0, 0),
    };
    let intermediate = build_intermediate(lines);

    for (i, row) in intermediate.iter().enumerate() {
        for (j, t) in row.iter().enumerate() {
            match t {
                Tile::Empty => {
                    let neighbors = get_immediate_neighbors(&intermediate, i, j);
                    let adjacency = Pt2Adjacency {
                        nodes: neighbors,
                        in_portal: None,
                        out_portal: None,
                    };
                    out.adjacency.insert((i, j), adjacency);
                }
                Tile::LabeledOuter(s) => {
                    let neighbors = get_immediate_neighbors(&intermediate, i, j);
                    let mut adjacency = Pt2Adjacency {
                        nodes: neighbors,
                        in_portal: None,
                        out_portal: None,
                    };
                    match s.as_str() {
                        "AA" => out.entrance = (i, j),
                        "ZZ" => out.exit = (i, j),
                        _ => adjacency.out_portal = Some(find_portal_match(&intermediate, i, j)),
                    }
                    out.adjacency.insert((i, j), adjacency);
                }
                Tile::LabeledInner(_) => {
                    let neighbors = get_immediate_neighbors(&intermediate, i, j);
                    let adjacency = Pt2Adjacency {
                        nodes: neighbors,
                        in_portal: Some(find_portal_match(&intermediate, i, j)),
                        out_portal: None,
                    };
                    out.adjacency.insert((i, j), adjacency);
                }
                Tile::Wall => {}
            }
        }
    }

    out
}

fn min_steps_pt2(g: &GraphPt2) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back((g.entrance, 0, 0));
    while let Some((coord, steps, level)) = queue.pop_front() {
        if coord == g.exit && level == 0 {
            return steps;
        }
        if visited.contains(&(coord, level)) {
            continue;
        }
        visited.insert((coord, level));
        for n in &g.adjacency[&coord].nodes {
            queue.push_back((*n, steps + 1, level));
        }
        if let Some(n) = g.adjacency[&coord].in_portal {
            queue.push_back((n, steps + 1, level + 1));
        }
        if let Some(n) = g.adjacency[&coord].out_portal {
            if level != 0 {
                queue.push_back((n, steps + 1, level - 1));
            }
        }
    }
    usize::MAX
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

    let graph = build_graph_pt1(&lines);
    println!("{}", min_steps_pt1(&graph));

    let graph2 = build_graph_pt2(&lines);
    println!("{}", min_steps_pt2(&graph2));
}
