use std::env;
use std::fs;

fn gcd(mut x: i32, mut y: i32) -> i32 {
    x = x.abs();
    y = y.abs();
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}

// Counts the number of asteroids visible from (x, y)
fn asteroids_visible_from(asteroids: &[Vec<bool>], x: i32, y: i32) -> i32 {
    let mut count = 0;
    for xdelta in -x..(asteroids.len() - x as usize) as i32 {
        for ydelta in -y..(asteroids[0].len() - y as usize) as i32 {
            if (xdelta == 0 && ydelta == 0) || gcd(xdelta, ydelta) != 1 {
                continue;
            }
            let mut mul = 1;
            let mut newx = x + mul * xdelta;
            let mut newy = y + mul * ydelta;
            while newx >= 0
                && newy >= 0
                && (newx as usize) < asteroids.len()
                && (newy as usize) < asteroids[0].len()
            {
                if asteroids[newx as usize][newy as usize] {
                    count += 1;
                    break;
                }
                mul += 1;
                newx = x + mul * xdelta;
                newy = y + mul * ydelta;
            }
        }
    }
    count
}

fn most_asteroids_visible(asteroids: &[Vec<bool>]) -> i32 {
    let mut max = 0;
    for x in 0..asteroids.len() {
        for y in 0..asteroids[0].len() {
            if asteroids[x][y] {
                let res = asteroids_visible_from(asteroids, x as i32, y as i32);
                if res > max {
                    max = res;
                }
            }
        }
    }
    max
}

pub fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let input: String = fs::read_to_string(&args[1]).expect("couldn't read file");
    let lines = input.split_whitespace();

    // build array of bool indicating asteroid locations.
    let mut asteroids = Vec::new();
    for l in lines {
        asteroids.push(Vec::new());
        let idx = asteroids.len() - 1;
        for c in l.trim().chars() {
            asteroids[idx].push(c == '#');
        }
    }
    println!("{}", most_asteroids_visible(&asteroids));
}
