use std::collections::HashMap;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

pub fn parse(input: &str) -> HashMap<usize, i64> {
    input
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<i64>()
                .unwrap_or_else(|_| panic!("invalid int {}", s))
        })
        .enumerate()
        .collect::<HashMap<_, _>>()
}

pub fn eval(ints: &mut HashMap<usize, i64>) {
    let (sender, reciever) = mpsc::channel();
    eval_with_input(ints, reciever, sender);
}

#[derive(Copy, Clone, PartialEq)]
enum ParamTypes {
    VALUE,
    INDEX,
}

// Gets a param for writing to an index
fn get_param_index(mode: i64, param: i64, relative_base: i64) -> i64 {
    match mode {
        0 => param,
        2 => relative_base + param,
        _ => panic!("bad parameter mode {} for index", mode),
    }
}

fn get_param(mode: i64, param: i64, mem: &mut HashMap<usize, i64>, relative_base: i64) -> i64 {
    match mode {
        0 => *mem.entry(param as usize).or_insert(0),
        1 => param,
        2 => *mem.entry((relative_base + param) as usize).or_insert(0),
        _ => panic!("unknown parameter mode {}", mode),
    }
}

fn get_params(
    pc: usize,
    mem: &mut HashMap<usize, i64>,
    param_types: &[ParamTypes],
    relative_base: i64,
) -> Vec<i64> {
    let mut opcode = *mem.entry(pc).or_insert(0);
    opcode /= 100;
    let mut out = Vec::new();
    for (i, pt) in param_types.iter().enumerate() {
        let mode = opcode % 10;
        opcode /= 10;
        let param = *mem.entry(pc + i + 1).or_insert(0);
        out.push(match pt {
            ParamTypes::INDEX => get_param_index(mode, param, relative_base),
            ParamTypes::VALUE => get_param(mode, param, mem, relative_base),
        });
    }
    out
}

pub fn eval_with_input(
    ints: &mut HashMap<usize, i64>,
    input: Receiver<i64>,
    output: Sender<i64>,
) -> Receiver<i64> {
    eval_with_input_and_requester(ints, input, output, None)
}

pub fn eval_with_input_and_requester(
    ints: &mut HashMap<usize, i64>,
    input: Receiver<i64>,
    output: Sender<i64>,
    requester: Option<Sender<()>>, // indicates that we want a value
) -> Receiver<i64> {
    let mut pc = 0;
    let mut relative_base = 0;
    while pc < ints.len() {
        match *ints.entry(pc).or_insert(0) % 100 {
            1 => {
                // add
                let params = get_params(
                    pc,
                    ints,
                    &[ParamTypes::VALUE, ParamTypes::VALUE, ParamTypes::INDEX],
                    relative_base,
                );
                let idx = params[2] as usize;
                ints.insert(idx, params[0] + params[1]);
                pc += 4;
            }
            2 => {
                // mul
                let params = get_params(
                    pc,
                    ints,
                    &[ParamTypes::VALUE, ParamTypes::VALUE, ParamTypes::INDEX],
                    relative_base,
                );
                let idx = params[2] as usize;
                ints.insert(idx, params[0] * params[1]);
                pc += 4;
            }
            3 => {
                // get input
                let idx = get_params(pc, ints, &[ParamTypes::INDEX], relative_base)[0] as usize;
                let val = match &requester {
                    None => match input.recv() {
                        Err(_) => return input,
                        Ok(x) => x,
                    },
                    Some(s) => {
                        if s.send(()).is_err() {
                            return input;
                        }
                        match input.recv() {
                            Err(_) => return input,
                            Ok(x) => x,
                        }
                    }
                };
                ints.insert(idx, val);
                pc += 2;
            }
            4 => {
                // output
                let param = get_params(pc, ints, &[ParamTypes::VALUE], relative_base)[0];
                output.send(param).unwrap();
                pc += 2;
            }
            5 => {
                // jump if nonzero
                let params = get_params(pc, ints, &[ParamTypes::VALUE; 2], relative_base);
                if params[0] != 0 {
                    pc = params[1] as usize;
                } else {
                    pc += 3;
                }
            }
            6 => {
                // jump if zero
                let params = get_params(pc, ints, &[ParamTypes::VALUE; 2], relative_base);
                if params[0] == 0 {
                    pc = params[1] as usize;
                } else {
                    pc += 3;
                }
            }
            7 => {
                // less than
                let params = get_params(
                    pc,
                    ints,
                    &[ParamTypes::VALUE, ParamTypes::VALUE, ParamTypes::INDEX],
                    relative_base,
                );
                let idx = params[2] as usize;
                ints.insert(idx, if params[0] < params[1] { 1 } else { 0 });
                pc += 4;
            }
            8 => {
                // equal
                let params = get_params(
                    pc,
                    ints,
                    &[ParamTypes::VALUE, ParamTypes::VALUE, ParamTypes::INDEX],
                    relative_base,
                );
                let idx = params[2] as usize;
                ints.insert(idx, if params[0] == params[1] { 1 } else { 0 });
                pc += 4;
            }
            9 => {
                // change relative base
                relative_base += get_params(pc, ints, &[ParamTypes::VALUE], relative_base)[0];
                pc += 2;
            }

            99 => break,
            _ => panic!("invalid opcode {} at index {}", ints[&pc], pc),
        }
    }
    // Let caller continue to read it
    input
}
