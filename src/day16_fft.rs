use std::env;
use std::fs;

const PATTERN: [i32; 4] = [0, 1, 0, -1];

fn fft_phase(ints: &[i32]) -> Vec<i32> {
    let mut out_vec = Vec::new();
    for i in 0..ints.len() {
        // produce each output element.
        let mut out: i32 = 0;
        let mut pat_idx = 0;
        let mut int_idx = 0;
        while int_idx < ints.len() {
            let upper = if int_idx == 0 && pat_idx == 0 {
                i
            } else {
                i + 1
            };
            for _j in 0..upper {
                // repeat the element when multiplying i times
                out += ints[int_idx] * PATTERN[pat_idx % PATTERN.len()];
                int_idx += 1;
                if int_idx >= ints.len() {
                    break;
                }
            }
            pat_idx += 1;
        }
        out_vec.push(out.abs() % 10);
    }

    out_vec
}

fn first_eight_digits_after_n_phases(ints: &[i32], phases: usize) -> i32 {
    let mut cur = ints.to_vec();
    for _ in 0..phases {
        cur = fft_phase(&cur);
    }

    let mut out = 0;
    for x in cur.iter().take(8) {
        out *= 10;
        out += x;
    }
    out
}

fn specified_eight_digits_after_n_phases(ints: &[i32], phases: usize) -> i32 {
    let mut idx = 0;
    for x in ints.iter().take(7) {
        idx *= 10;
        idx += *x as usize;
    }

    let desired_len = 10_000 * ints.len();
    // all positions before |idx| will cancel out -- they'll be zero.
    let mut cur = ints
        .iter()
        .cloned()
        .cycle()
        .take(desired_len)
        .skip(idx)
        .collect::<Vec<_>>();

    // after idx, we'll have 1s, but one fewer of them each time (and one more zero).
    // so at |idx| we have the sum, at |idx + 1| we have the sum minus the first, at |idx + 2| the
    // sum minus the first and second, etc.
    for _ in 0..phases {
        let partials = cur
            .iter()
            .scan(0, |state, &x| {
                *state += x;
                Some(*state)
            })
            .collect::<Vec<_>>();
        let sum = *partials.last().unwrap();
        for (i, x) in cur.iter_mut().enumerate() {
            if i == 0 {
                *x = sum % 10;
            } else {
                *x = (sum - partials[i - 1]) % 10;
            }
        }
    }

    let mut out = 0;
    for x in cur.iter().take(8) {
        out *= 10;
        out += x;
    }
    out
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let input = fs::read_to_string(&args[1]).expect("couldn't read file");

    let ints = input
        .trim()
        .chars()
        .map(|c| c.to_digit(10).unwrap() as i32)
        .collect::<Vec<_>>();
    println!("{}", first_eight_digits_after_n_phases(&ints, 100));
    println!("{}", specified_eight_digits_after_n_phases(&ints, 100));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fft_phase() {
        assert_eq!(
            fft_phase(&[1, 2, 3, 4, 5, 6, 7, 8]),
            vec![4, 8, 2, 2, 6, 1, 5, 8]
        );
        assert_eq!(
            fft_phase(&[4, 8, 2, 2, 6, 1, 5, 8]),
            vec![3, 4, 0, 4, 0, 4, 3, 8]
        );
    }
}
