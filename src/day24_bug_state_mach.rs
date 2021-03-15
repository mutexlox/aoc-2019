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

fn step(board: usize) -> usize {
    let mut out = 0;
    for i in 0..WIDTH * HEIGHT {
        let mut count = 0;
        for n in get_neighbors(i) {
            if board & (1 << n) != 0 {
                count += 1;
            }
        }
        if (board & 1 << i) != 0 && count == 1 {
            out |= 1 << i;
        } else if (board & 1 << i) == 0 && (count == 1 || count == 2) {
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
        state = step(state);
    }

    state
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let input = fs::read_to_string(&args[1]).expect("couldn't read file");
    let lines = input.trim().split('\n').map(|s| s.trim());

    let mut board = 0;
    let mut mul = 1;
    for l in lines {
        for c in l.chars() {
            if c == '#' {
                board += mul;
            }
            mul *= 2;
        }
    }
    println!("{}", iterate_til_seen(board));
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
        assert_eq!(step(init), next);

        // state after 2 minutes
        let two = 0b11101_01000_10000_10000_11111;
        assert_eq!(step(next), two);
    }
}
