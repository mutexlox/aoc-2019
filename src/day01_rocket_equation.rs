use std::env;
use std::fs;

fn fuel_to_launch(mass: i32) -> i32 {
    mass / 3 - 2
}

fn total_fuel_requirement(module_masses: Vec<i32>) -> i32 {
    let mut out = 0;
    for mass in module_masses {
       out += fuel_to_launch(mass);
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);

    let input: String = fs::read_to_string(&args[1]).expect("couldn't read file");
    let lines = input.split_whitespace().map(|s| {
        s.parse::<i32>().unwrap_or_else(|_| panic!("invalid int {}", &s))
    }).collect();
    println!("{}", total_fuel_requirement(lines));
}
