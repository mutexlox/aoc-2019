use std::cmp::Ordering;
use std::collections::HashSet;
use std::env;
use std::fs;

fn update_velocities(positions: &[Vec<i64>], velocities: &mut Vec<Vec<i64>>) {
    for (i, p1) in positions.iter().enumerate() {
        for p2 in positions {
            for (k, val) in p1.iter().enumerate() {
                velocities[i][k] += match val.cmp(&p2[k]) {
                    Ordering::Less =>  1,
                    Ordering::Greater => -1,
                    Ordering::Equal => 0,
                }
            }
        }
    }
}

fn update_positions(positions: &mut Vec<Vec<i64>>, velocities: &[Vec<i64>]) {
    for (p, v) in positions.iter_mut().zip(velocities.iter()) {
        for (i, coord) in p.iter_mut().enumerate() {
            *coord += v[i];
        }
    }
}

fn compute_energy(positions: &[Vec<i64>], velocities: &[Vec<i64>]) -> i64 {
    let mut energy = 0;
    for (i, p) in positions.iter().enumerate() {
        let mut potential_energy = 0;
        for x in p {
            potential_energy += x.abs();
        }
        let mut kinetic_energy = 0;
        for x in velocities[i].iter() {
            kinetic_energy += x.abs();
        }
        energy += potential_energy * kinetic_energy;
    }
    energy
}

fn do_steps(positions: &mut Vec<Vec<i64>>, nsteps: i64) -> i64 {
    let mut velocities = Vec::new();
    for _ in positions.iter() {
        velocities.push(vec![0, 0, 0]);
    }
    for _ in 0..nsteps {
        update_velocities(positions, &mut velocities);
        update_positions(positions, &velocities);
    }
    compute_energy(positions, &velocities)
}

fn gcd(mut x: i64, mut y: i64) -> i64 {
    x = x.abs();
    y = y.abs();
    while y != 0 {
        let temp = y;
        y = x % y;
        x = temp;
    }
    x
}

fn lcm(x: i64, y: i64) -> i64 {
    (x * y).abs() / gcd(x, y)
}

fn lcm_arr(vels: &[i64]) -> i64 {
    let mut acc = vels[0];
    for v in vels.iter().skip(1) {
        acc = lcm(acc, *v);
    }
    acc
}

fn steps_til_repeat(positions: &mut Vec<Vec<i64>>) -> i64 {
    // compute periods of each axis, then take least common multiple of those.

    let mut velocities = Vec::new();

    // periods for each axis.
    let mut seen: [HashSet<(Vec<i64>, Vec<i64>)>; 3] =
        [HashSet::new(), HashSet::new(), HashSet::new()];
    let mut periods = [0, 0, 0];

    for _ in positions.iter() {
        velocities.push(vec![0, 0, 0]);
    }

    let mut i = 0;
    loop {
        let mut done = true;
        for j in 0..3 {
            if periods[j] != 0 {
                continue;
            }
            // build current view of each axis
            let mut pos = Vec::new();
            let mut vel = Vec::new();
            for p in positions.iter() {
                pos.push(p[j]);
            }
            for v in velocities.iter() {
                vel.push(v[j]);
            }
            if seen[j].contains(&(pos.clone(), vel.clone())) {
                periods[j] = i;
            } else {
                done = false;
                seen[j].insert((pos.clone(), vel.clone()));
            }
        }
        if done {
            break;
        }
        i += 1;

        update_velocities(positions, &mut velocities);
        update_positions(positions, &velocities);
    }
    lcm_arr(&periods)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let input = fs::read_to_string(&args[1]).expect("couldn't read file");
    let lines = input.trim().split('\n').map(|s| s.trim());

    // parse
    let mut positions = Vec::new();
    for l in lines {
        let trimmed = l.trim_start_matches('<').trim_end_matches('>');
        let parts = trimmed.split(',').map(|s| s.trim());
        let mut coords = Vec::new();
        for p in parts {
            let coord = p.split('=').nth(1).unwrap().parse::<i64>().unwrap();
            coords.push(coord);
        }
        positions.push(coords);
    }
    let mut pos2 = positions.clone();
    println!("{}", do_steps(&mut positions, 1000));
    println!("{}", steps_til_repeat(&mut pos2));
}
