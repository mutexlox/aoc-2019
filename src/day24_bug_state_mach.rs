use std::collections::HashSet;
use std::env;
use std::fs;

const WIDTH: usize = 5;
const HEIGHT: usize = 5;

fn get_neighbors(idx: usize) -> Vec<usize> {
    let mut out = Vec::new();
    if idx >= WIDTH {
        // past first row
        out.push(idx - WIDTH);
    }
    if idx < WIDTH * HEIGHT - WIDTH {
        // before last row
        out.push(idx + WIDTH);
    }
    if idx % WIDTH != 0 {
        // out of first column
        out.push(idx - 1);
    }
    if idx % WIDTH != WIDTH - 1 {
        // out of last column
        out.push(idx + 1);
    }

    out
}

fn step_pt1(board: usize) -> usize {
    let mut out = 0;
    for i in 0..WIDTH * HEIGHT {
        let mut count = 0;
        for n in get_neighbors(i) {
            if board & (1 << n) != 0 {
                count += 1;
            }
        }
        if count == 1 || (board & 1 << i) == 0 && count == 2 {
            out |= 1 << i;
        }
    }

    out
}

fn iterate_til_seen(start: usize) -> usize {
    let mut seen = HashSet::new();
    let mut state = start;
    while !seen.contains(&state) {
        seen.insert(state);
        state = step_pt1(state);
    }

    state
}

// part 2

fn get_neighbors_pt2(x: usize, y: usize, layer: i64) -> Vec<(usize, usize, i64)> {
    let mut out = Vec::new();
    // Above
    if x == 0 {
        // no matter what, above is (1, 2) in containing layer TODO: maybe make this generic?
        out.push((1, 2, layer - 1));
    } else if x == 3 && y == 2 {
        // push all elements of last row of inner layer
        for i in 0..WIDTH {
            out.push((HEIGHT - 1, i, layer + 1));
        }
    } else {
        out.push((x - 1, y, layer));
    }

    // Left
    if y == 0 {
        // no matter what, left is (2, 1) in containing layer. TODO: maybe make this generic?
        out.push((2, 1, layer - 1));
    } else if x == 2 && y == 3 {
        // Push all elements of right column of inner layer
        for i in 0..HEIGHT {
            out.push((i, WIDTH - 1, layer + 1));
        }
    } else {
        out.push((x, y - 1, layer));
    }

    // Below
    if x == HEIGHT - 1 {
        // no matter what, below is (3, 2) in containing layer.
        out.push((3, 2, layer - 1));
    } else if x == 1 && y == 2 {
        // push all elements of first row of inner layer
        for i in 0..WIDTH {
            out.push((0, i, layer + 1));
        }
    } else {
        out.push((x + 1, y, layer));
    }

    // Right
    if y == WIDTH - 1 {
        // below is (2, 3) in containing layer
        out.push((2, 3, layer - 1));
    } else if x == 2 && y == 1 {
        // push all elements of left column of inner layer
        for i in 0..HEIGHT {
            out.push((i, 0, layer + 1));
        }
    } else {
        out.push((x, y + 1, layer));
    }

    out
}

fn step_pt2(state: &HashSet<(usize, usize, i64)>) -> HashSet<(usize, usize, i64)> {
    let mut out = HashSet::new();
    let mut also_check = HashSet::new();
    for (x, y, layer) in state {
        let mut count = 0;
        for n in get_neighbors_pt2(*x, *y, *layer) {
            if state.contains(&n) {
                count += 1;
            } else {
                also_check.insert(n);
            }
        }
        if count == 1 {
            out.insert((*x, *y, *layer));
        }
    }

    for (x, y, layer) in also_check {
        let mut count = 0;
        for n in get_neighbors_pt2(x, y, layer) {
            if state.contains(&n) {
                count += 1;
            }
        }
        if count == 1 || count == 2 {
            out.insert((x, y, layer));
        }
    }

    out
}

fn iter_part2(mut state: HashSet<(usize, usize, i64)>, iters: usize) -> usize {
    for _ in 0..iters {
        state = step_pt2(&state);
    }
    state.len()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let input = fs::read_to_string(&args[1]).expect("couldn't read file");
    let lines = input
        .trim()
        .split('\n')
        .map(|s| s.trim())
        .collect::<Vec<_>>();

    let mut board = 0;
    let mut mul = 1;
    for l in lines.iter() {
        for c in l.chars() {
            if c == '#' {
                board += mul;
            }
            mul *= 2;
        }
    }
    println!("{}", iterate_til_seen(board));

    let mut state = HashSet::new();

    for (i, l) in lines.iter().enumerate() {
        for (j, c) in l.chars().enumerate() {
            if c == '#' {
                state.insert((i, j, 0));
            }
        }
    }
    println!("{}", iter_part2(state, 200));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_step() {
        // initial state as shown in https://adventofcode.com/2019/day/24
        let init = 0b00001_00100_11001_01001_10000;
        // state after 1 minute
        let next = 0b00110_11011_10111_01111_01001;
        assert_eq!(step_pt1(init), next);

        // state after 2 minutes
        let two = 0b11101_01000_10000_10000_11111;
        assert_eq!(step_pt1(next), two);
    }
}
