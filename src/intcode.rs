pub fn parse(input: &str) -> Vec<i32> {
    input
        .split(',')
        .map(|s| {
            s.trim()
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("invalid int {}", s))
        })
        .collect::<Vec<_>>()
}

pub fn eval(ints: &mut [i32]) {
    eval_with_input(ints, &[], &mut Vec::new());
}

pub fn get_param(mode: i32, param: i32, mem: &[i32]) -> i32 {
    match mode {
        0 => mem[param as usize],
        1 => param,
        _ => panic!("unknown parameter mode {}", mode),
    }
}

pub fn get_params(mut opcode: i32, params: &[i32], mem: &[i32]) -> Vec<i32> {
    assert!(params.len() <= 3);
    opcode /= 100;
    let mut out = Vec::new();
    for p in params {
        let mode = opcode % 10;
        opcode /= 10;
        out.push(get_param(mode, *p, mem));
    }
    out
}

pub fn eval_with_input(ints: &mut [i32], inputs: &[i32], outputs: &mut Vec<i32>) {
    eval_with_input_and_pc(ints, inputs, outputs, None);
}
// Returns pc at which to resume execution, if any
pub fn eval_with_input_and_pc(
    ints: &mut [i32],
    inputs: &[i32],
    outputs: &mut Vec<i32>,
    pc_maybe: Option<usize>,
) -> Option<usize> {
    let mut pc = pc_maybe.unwrap_or(0);
    let mut input_idx = 0;
    while pc < ints.len() {
        match ints[pc] % 100 {
            1 => {
                // add
                let params = get_params(ints[pc], &ints[pc + 1..pc + 3], ints);
                assert!(ints[pc] < 10_000, "unexpected immediate output of +");
                ints[ints[pc + 3] as usize] = params[0] + params[1];
                pc += 4;
            }
            2 => {
                // mul
                let params = get_params(ints[pc], &ints[pc + 1..pc + 3], ints);
                assert!(ints[pc] < 10_000, "unexpected immediate output of *");
                ints[ints[pc + 3] as usize] = params[0] * params[1];
                pc += 4;
            }
            3 => {
                // get input
                assert!(ints[pc] < 100, "unexpected immediate output of 3");
                if input_idx >= inputs.len() {
                    return Some(pc);
                }
                ints[ints[pc + 1] as usize] = inputs[input_idx];
                input_idx += 1;
                pc += 2;
            }
            4 => {
                // output
                let param = get_params(ints[pc], &ints[pc + 1..pc + 2], ints)[0];
                outputs.push(param);
                pc += 2;
            }
            5 => {
                // jump if nonzero
                let params = get_params(ints[pc], &ints[pc + 1..pc + 3], ints);
                if params[0] != 0 {
                    pc = params[1] as usize;
                } else {
                    pc += 3;
                }
            }
            6 => {
                // jump if zero
                let params = get_params(ints[pc], &ints[pc + 1..pc + 3], ints);
                if params[0] == 0 {
                    pc = params[1] as usize;
                } else {
                    pc += 3;
                }
            }
            7 => {
                // less than
                let params = get_params(ints[pc], &ints[pc + 1..pc + 3], ints);
                assert!(ints[pc] < 10_000, "unexpected immediate output of <");
                ints[ints[pc + 3] as usize] = if params[0] < params[1] { 1 } else { 0 };
                pc += 4;
            }
            8 => {
                // equal
                let params = get_params(ints[pc], &ints[pc + 1..pc + 3], ints);
                assert!(ints[pc] < 10_000, "unexpected immediate output of <");
                ints[ints[pc + 3] as usize] = if params[0] == params[1] { 1 } else { 0 };
                pc += 4;
            }
            99 => break,
            _ => panic!("invalid opcode {} at index {}", ints[pc], pc),
        }
    }

    None
}
