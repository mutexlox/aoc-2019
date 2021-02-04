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
fn asteroids_visible_from(asteroids: &[Vec<bool>], x: i32, y: i32) -> Vec<(usize, usize)> {
    let mut coords = Vec::new();
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
                    coords.push((newx as usize, newy as usize));
                    break;
                }
                mul += 1;
                newx = x + mul * xdelta;
                newy = y + mul * ydelta;
            }
        }
    }
    coords
}

// Formula from https://math.stackexchange.com/a/3004599
fn compute_sort_key(xdelta: f64, ydelta: f64) -> f64 {
    if xdelta < 0.0 && ydelta == 0.0 {
        std::f64::consts::PI
    } else {
        let arg = ydelta / (xdelta + (xdelta.powf(2.0) + ydelta.powf(2.0)).sqrt());
        let mut res = 2.0 * arg.atan();
        if res < 0.0 {
            res += 2.0 * std::f64::consts::PI;
        }
        res
    }
}

// Sort the given asteroids based on clockwise order starting from straight up.
fn sort_visible_asteroids(asteroids: &mut Vec<(usize, usize)>, x: usize, y: usize) {
    let mut sort_keys = Vec::new();
    for a in asteroids.iter() {
        let xdelta = (x as i32 - a.0 as i32) as f64;
        let ydelta = (a.1 as i32 - y as i32) as f64;
        sort_keys.push((*a, compute_sort_key(xdelta, ydelta)));
    }
    sort_keys.sort_by(|k1, k2| k1.1.partial_cmp(&k2.1).unwrap());
    for (i, k) in sort_keys.iter().enumerate() {
        asteroids[i] = k.0;
    }
}

fn most_asteroids_visible(asteroids: &[Vec<bool>]) -> usize {
    let mut max = Vec::new();
    let (mut max_x, mut max_y) = (0, 0);
    for x in 0..asteroids.len() {
        for y in 0..asteroids[0].len() {
            if asteroids[x][y] {
                let res = asteroids_visible_from(asteroids, x as i32, y as i32);
                if res.len() > max.len() {
                    max = res;
                    max_x = x;
                    max_y = y;
                }
            }
        }
    }
    sort_visible_asteroids(&mut max, max_x, max_y);
    println!("part 2: {}", max[199].1 * 100 + max[199].0);
    max.len()
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

#[cfg(test)]
mod test {
    use crate::*;
    use float_eq::assert_float_eq;

    #[test]
    fn test_compute_sort_key() {
        assert_eq!(compute_sort_key(1.0, 0.0), 0.0);
        assert_float_eq!(
            compute_sort_key(1.0, 1.0),
            std::f64::consts::PI / 4.0,
            abs <= 0.000_01
        );
        assert_float_eq!(
            compute_sort_key(0.0, 1.0),
            std::f64::consts::PI / 2.0,
            abs <= 0.000_01
        );
        assert_float_eq!(
            compute_sort_key(-1.0, 1.0),
            3.0 * std::f64::consts::PI / 4.0,
            abs <= 0.00001
        );
        assert_float_eq!(
            compute_sort_key(-1.0, 0.0),
            std::f64::consts::PI,
            abs <= 0.000_01
        );
        assert_float_eq!(
            compute_sort_key(-1.0, -1.0),
            5.0 * std::f64::consts::PI / 4.0,
            abs <= 0.00001
        );
        assert_float_eq!(
            compute_sort_key(0.0, -1.0),
            3.0 * std::f64::consts::PI / 2.0,
            abs <= 0.00001
        );
        assert_float_eq!(
            compute_sort_key(1.0, -1.0),
            7.0 * std::f64::consts::PI / 4.0,
            abs <= 0.00001
        );
    }

    #[test]
    fn test_sort_visible_asteroids() {
        let x = 38;
        let y = 36;
        let mut asteroids = vec![
            (34, 36),
            (34, 37),
            (37, 35),
            (38, 35),
            (38, 38),
            (39, 35),
            (39, 36),
            (39, 38),
        ];
        let expected_asteroids = vec![
            (34, 36),
            (34, 37),
            (38, 38),
            (39, 38),
            (39, 36),
            (39, 35),
            (38, 35),
            (37, 35),
        ];
        sort_visible_asteroids(&mut asteroids, x, y);
        assert_eq!(asteroids, expected_asteroids);
    }
}
