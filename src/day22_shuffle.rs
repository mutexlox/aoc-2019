use itertools::Itertools;
use regex::Regex;
use std::env;
use std::fs;

fn deal_new_stack(cards: &mut [i32]) {
    cards.reverse();
}

fn cut(cards: &mut [i32], n: i32) {
    if n < 0 {
        cards.rotate_right(-n as usize);
    } else {
        cards.rotate_left(n as usize);
    }
}

fn increment(cards: &mut [i32], n: usize) {
    let mut tmp = Vec::new();
    tmp.extend_from_slice(cards);
    let mut dealt = 0;
    let mut pos = 0;
    while dealt < cards.len() {
        tmp[pos] = cards[dealt];
        dealt += 1;
        pos = (pos + n) % cards.len();
    }
    cards.copy_from_slice(&tmp);
}

fn apply_shuffle(lines: &Vec<&str>, cards: &mut [i32]) {
    let inc_regex = Regex::new(r"^deal with increment (\d+)$").unwrap();
    let cut_regex = Regex::new(r"^cut (-?\d+)$").unwrap();
    let rev_regex = Regex::new(r"^deal into new stack$").unwrap();
    for l in lines {
        if let Some(c) = inc_regex.captures(l) {
            increment(cards, c[1].parse::<usize>().unwrap());
        } else if let Some(c) = cut_regex.captures(l) {
            cut(cards, c[1].parse::<i32>().unwrap());
        } else {
            assert!(rev_regex.is_match(l));
            deal_new_stack(cards);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let input = fs::read_to_string(&args[1]).expect("couldn't read file");
    let lines = input
        .trim()
        .split('\n')
        .map(|s| s.trim())
        .collect::<Vec<_>>();

    let mut deck = [0; 10007];
    for i in 0..deck.len() {
        deck[i] = i as i32;
    }

    apply_shuffle(&lines, &mut deck);
    println!("{}", deck.iter().find_position(|x| **x == 2019).unwrap().0);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_deal_new_stack() {
        let mut input = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        deal_new_stack(&mut input);
        assert_eq!(input, [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]);
    }

    #[test]
    fn test_cut() {
        let mut input = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        cut(&mut input, 3);
        assert_eq!(input, [3, 4, 5, 6, 7, 8, 9, 0, 1, 2]);

        input = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        cut(&mut input, -4);
        assert_eq!(input, [6, 7, 8, 9, 0, 1, 2, 3, 4, 5])
    }
    #[test]
    fn test_increment() {
        let mut input = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        increment(&mut input, 3);
        assert_eq!(input, [0, 7, 4, 1, 8, 5, 2, 9, 6, 3])
    }
}
