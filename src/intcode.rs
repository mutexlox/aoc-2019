pub fn eval(ints: &mut [usize]) -> usize {
    let mut pc = 0;
    while pc < ints.len() {
        match ints[pc] {
            1 => {
                ints[ints[pc + 3]] = ints[ints[pc + 1]] + ints[ints[pc + 2]];
                pc += 4
            }
            2 => {
                ints[ints[pc + 3]] = ints[ints[pc + 1]] * ints[ints[pc + 2]];
                pc += 4
            }
            99 => break,
            _ => panic!("invalid opcode {} at index {}", ints[pc], pc),
        }
    }

    ints[0]
}
