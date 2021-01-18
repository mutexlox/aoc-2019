use std::env;
use std::fs;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn find_fewest_zeros(layers: &[Vec<u32>]) -> usize {
    let mut min_zeros = None;
    let mut ones_times_twos = 0;
    for l in layers.iter() {
        let zeros = l.iter().filter(|x| **x == 0).count();
        let ones = l.iter().filter(|x| **x == 1).count();
        let twos = l.iter().filter(|x| **x == 2).count();
        if let Some(x) = min_zeros {
            if x > zeros {
                min_zeros = Some(zeros);
                ones_times_twos = ones * twos;
            }
        } else {
            min_zeros = Some(zeros);
            ones_times_twos = ones * twos;
        }
    }
    ones_times_twos
}

fn get_frontmost_pixels(layers: &[Vec<u32>]) -> Vec<u32> {
    let mut out = Vec::new();
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let mut k: usize = 0;
            while k < layers.len() && layers[k][i * WIDTH + j] == 2 {
                k += 1;
            }
            if k == layers.len() {
                panic!("all transparent");
            }
            out.push(layers[k][i * WIDTH + j]);
        }
    }
    out
}

fn print_picture(frontmost: &[u32]) {
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            print!(
                "{}",
                match frontmost[i * WIDTH + j] {
                    1 => 'X',
                    0 => ' ',
                    _ => panic!("unexpected digit {}", frontmost[i * WIDTH + j]),
                }
            );
        }
        println!();
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let input_big = fs::read_to_string(&args[1]).expect("couldn't read file");
    let input = input_big.trim();

    let mut layers = Vec::new();
    let mut layer = Vec::new();
    for c in input.chars() {
        layer.push(c.to_digit(10).unwrap());
        if layer.len() == (WIDTH * HEIGHT) {
            layers.push(layer);
            layer = Vec::new();
        }
    }

    println!("{}", find_fewest_zeros(&layers));
    let front = get_frontmost_pixels(&layers);
    print_picture(&front);
}
