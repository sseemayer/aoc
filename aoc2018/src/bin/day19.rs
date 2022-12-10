use anyhow::Result;
use aoc2018::vm::Vm;

fn fast_run(r4: usize) -> usize {
    // the decompiled code is just a factorization of r4 and summing up of factors.
    let mut r0 = 0;
    for r3 in 1..r4 {
        if r4 % r3 == 0 {
            r0 += r4 / r3;
        }
    }
    r0 + 1
}

fn main() -> Result<()> {
    let vm = Vm::parse("data/day19/input")?;

    let mut vm_part1 = vm.clone();
    vm_part1.debug = false;
    vm_part1.run_to_end();
    println!("Part 1: {}", vm_part1.state.get(0));

    println!("Part 1, fast: {}", fast_run(958));

    println!("Part 2: {}", fast_run(10551358));

    Ok(())
}
