use std::collections::HashMap;
use std::env;
use std::fs;

fn count_all_orbits(map: &HashMap<&str, Vec<&str>>, start: &str, parent_bodies: i32) -> i32 {
    match map.get(start) {
        None => parent_bodies,
        Some(bodies) => {
            let mut out = parent_bodies;
            for other in bodies {
                out += count_all_orbits(map, other, parent_bodies + 1)
            }
            out
        }
    }
}

// returns tuple of (dist to SAN from least-common-ancestor, dist to YOU from least-common-ancestor)
// (Add them to get answer)
fn min_distance_to_san(map: &HashMap<&str, Vec<&str>>, start: &str) -> (Option<i32>, Option<i32>) {
    if start == "YOU" {
        (None, Some(-1))
    } else if start == "SAN" {
        (Some(-1), None)
    } else {
        match map.get(start) {
            None => (None, None),
            Some(bodies) => {
                let mut san = None;
                let mut you = None;
                for other in bodies {
                    let (maybe_san, maybe_you) = min_distance_to_san(map, other);
                    match (maybe_san, maybe_you) {
                        (Some(_), Some(_)) => {
                            san = maybe_san;
                            you = maybe_you;
                        }
                        (Some(x), None) => san = Some(x + 1),
                        (None, Some(y)) => you = Some(y + 1),
                        (None, None) => {}
                    }
                }
                (san, you)
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input: String = fs::read_to_string(&args[1]).expect("couldn't read file");
    let lines = input.split_whitespace();
    let mut map = HashMap::<&str, Vec<&str>>::new();
    for l in lines {
        let mut parts = l.split(')');
        let inner = parts.next().unwrap();
        let outer = parts.next().unwrap();
        match map.get_mut(&inner) {
            Some(v) => v.push(outer),
            None => {
                map.insert(inner, vec![outer]);
            }
        };
    }
    println!("{}", count_all_orbits(&map, "COM", 0));
    let (san, you) = min_distance_to_san(&map, "COM");
    println!("{}", san.unwrap() + you.unwrap());
}
