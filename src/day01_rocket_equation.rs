use std::env;
use std::fs;

fn fuel_to_launch(mass: i32) -> i32 {
    mass / 3 - 2
}

fn fuel_to_launch_counting_fuel(mass: i32) -> i32 {
    let base = fuel_to_launch(mass);
    let mut additional = 0;
    let mut last = base;
    while last > 0 {
        let new = fuel_to_launch(last);
        if new >= 0 {
            additional += new
        }
        last = new;
    }

    base + additional
}

fn total_fuel_requirement(module_masses: &[i32]) -> i32 {
    let mut out = 0;
    for mass in module_masses {
        out += fuel_to_launch(*mass);
    }
    out
}

fn total_fuel_requirement_counting_fuel(module_masses: &[i32]) -> i32 {
    let mut out = 0;
    for mass in module_masses {
        out += fuel_to_launch_counting_fuel(*mass);
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input: String = fs::read_to_string(&args[1]).expect("couldn't read file");
    let lines = input
        .split_whitespace()
        .map(|s| {
            s.parse::<i32>()
                .unwrap_or_else(|_| panic!("invalid int {}", &s))
        })
        .collect::<Vec<_>>();
    println!("{}", total_fuel_requirement(&lines));
    println!("{}", total_fuel_requirement_counting_fuel(&lines));
}
