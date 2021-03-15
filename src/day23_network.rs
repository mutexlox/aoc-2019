use std::collections::{HashMap, VecDeque};
use std::env;
use std::fs;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

const NUM_NICS: usize = 50;

fn run_nics(ints: &HashMap<usize, i64>, part_one: bool) -> i64 {
    let mut senders = Vec::new();
    let mut receivers = Vec::new();
    let mut meta_receivers = Vec::new();
    let mut children = Vec::new();
    let mut pending = Vec::new();

    for i in 0..NUM_NICS {
        let (in_sender, in_receiver) = mpsc::channel();
        let (out_sender, out_receiver) = mpsc::channel();
        let (meta_sender, meta_receiver) = mpsc::channel();
        in_sender.send(i as i64).unwrap();
        senders.push(in_sender);
        receivers.push(out_receiver);
        meta_receivers.push(meta_receiver);
        pending.push(VecDeque::new());

        let mut mem = ints.to_owned();
        children.push(thread::spawn(move || {
            intcode::eval_with_input_and_requester(
                &mut mem,
                in_receiver,
                out_sender,
                Some(meta_sender),
            );
        }));
    }

    let mut nat_x = 0;
    let mut nat_y = 0;
    let mut last_sent_y = -1;
    let mut waiting_count = 0;
    loop {
        let mut got_any = false;
        let mut all_waiting = true;
        for i in 0..NUM_NICS {
            if let Ok(dest) = receivers[i].try_recv() {
                got_any = true;
                let x = receivers[i].recv().unwrap();
                let y = receivers[i].recv().unwrap();
                if dest == 255 {
                    if part_one {
                        return y;
                    }
                    nat_x = x;
                    nat_y = y;
                } else {
                    pending[dest as usize].push_back(x);
                    pending[dest as usize].push_back(y);
                }
            }

            if meta_receivers[i].try_recv().is_ok() {
                if let Some(x) = pending[i].pop_front() {
                    senders[i].send(x).unwrap();
                    all_waiting = false;
                } else {
                    senders[i].send(-1).unwrap();
                }
            }
        }

        // check to see if all are waiting and none are sending for "a little while"
        if !got_any && all_waiting {
            if waiting_count >= 1 {
                if !part_one && nat_y == last_sent_y {
                    return nat_y;
                }
                last_sent_y = nat_y;

                // None are sending. send nat_x and nat_y
                pending[0].push_back(nat_x);
                pending[0].push_back(nat_y);
            }
            thread::sleep(Duration::from_millis(10));
            waiting_count += 1;
        } else {
            waiting_count = 0;
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(args.len(), 2);
    let input = fs::read_to_string(&args[1]).expect("couldn't read file");
    let ints = intcode::parse(&input);

    println!("{}", run_nics(&ints, true));
    println!("{}", run_nics(&ints, false));
}
