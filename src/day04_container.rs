use std::env;
use std::fs;

fn matches_rules(mut x: i32, min: i32, max: i32) -> bool {
    if x < 100_000 || x > 999_999 {
        return false;
    }
    if x < min || x > max {
        return false;
    }
    let mut last = x % 10;
    x /= 10;
    let mut has_double = false;
    while x > 0 {
        if x % 10 == last {
            has_double = true;
        }
        if x % 10 > last {
            return false;
        }
        last = x % 10;
        x /= 10;
    }
    has_double
}

fn matches_exactly_one_double(mut x: i32, min: i32, max: i32) -> bool {
    if !matches_rules(x, min, max) {
        false
    } else {
        let mut has_double = false;
        let mut run_length = 1;
        while x > 0 {
            let last = x % 10;
            x /= 10;
            if x % 10 == last {
                run_length += 1
            } else {
                if run_length == 2 {
                    has_double = true;
                    break;
                }
                run_length = 1;
            }
        }
        has_double || run_length == 2
    }
}

fn num_valid_pass(min: i32, max: i32) -> i32 {
    let mut count = 0;
    for i in min..max + 1 {
        if matches_rules(i, min, max) {
            count += 1;
        }
    }
    count
}

fn num_valid_pass_pt2(min: i32, max: i32) -> i32 {
    let mut count = 0;
    for i in min..max + 1 {
        if matches_exactly_one_double(i, min, max) {
            count += 1;
        }
    }
    count
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input: String = fs::read_to_string(&args[1]).expect("couldn't read file");
    let mut range = input.trim().split("-").map(|s| s.parse::<i32>().unwrap());
    let min = range.next().unwrap();
    let max = range.next().unwrap();
    println!("{}", num_valid_pass(min, max));
    println!("{}", num_valid_pass_pt2(min, max));
}
