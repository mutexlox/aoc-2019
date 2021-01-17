pub fn eval(ints: &mut [i32]) -> i32 {
    eval_with_input(ints, &[])
}
pub fn eval_with_input(ints: &mut [i32], _inputs: &[i32]) -> i32 {
    let mut pc = 0;
    while pc < ints.len() {
        match ints[pc] {
            1 => {
                ints[ints[pc + 3] as usize] =
                    ints[ints[pc + 1] as usize] + ints[ints[pc + 2] as usize];
                pc += 4
            }
            2 => {
                ints[ints[pc + 3] as usize] =
                    ints[ints[pc + 1] as usize] * ints[ints[pc + 2] as usize];
                pc += 4
            }
            99 => break,
            _ => panic!("invalid opcode {} at index {}", ints[pc], pc),
        }
    }

    ints[0]
}
