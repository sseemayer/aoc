use std::collections::HashSet;

use anyhow::Result;

use aoc2018::vm::Vm;

fn fast_part2() -> usize {
    let mut r3 = 0x10000;
    let mut r4 = 12670166;

    let mut prev_r4 = 0;
    let mut seen_r4 = HashSet::new();

    loop {
        let r2 = r3 & 0xff;
        r4 += r2;
        r4 = ((r4 & 0xffffff) * 65899) & 0xffffff;

        if r3 < 256 {
            // r4 is a solution

            if !seen_r4.insert(r4) {
                // r4 has appeared before
                break prev_r4;
            }

            r3 = r4 | 0x10000;
            prev_r4 = r4;
            r4 = 12670166;
            continue;
        }
        r3 /= 256;
    }
}

fn main() -> Result<()> {
    let vm = Vm::parse("data/day21/input")?;

    let mut vm_1 = vm.clone();
    vm_1.run_with_interrupt(|vm| {
        let ip = vm.ip.get(&vm.state);

        if ip == 29 {
            let r4 = vm.state.get(4);
            println!("Part 1: {}", r4);
            return false;
        }
        true
    });

    let mut last_r4 = 0;
    let mut seen_r4 = HashSet::new();
    let mut vm_2 = vm.clone();
    vm_2.debug = false;
    vm_2.run_with_interrupt(|vm| {
        let ip = vm.ip.get(&vm.state);
        if ip == 28 {
            // ip is the check for termination
            // 28: eqrr 4 0 2  <=> if r4 == r0 { break }
            // 29: addr 2 1 1
            let r4 = vm.state.get(4);

            // println!("{} {}", r4, seen_r4.len());
            if seen_r4.contains(&r4) {
                return false;
            }
            last_r4 = r4;

            seen_r4.insert(r4);
        } else if ip == 17 {
            // hijack the inefficient division
            //
            // r2 = 0
            // while (r2 + 1) * 256 <= r3 {
            // 	r2 += 1
            // }
            let r3 = vm.state.get(3);
            let r2 = r3 / 256;

            vm.state.set(2, r2);
            vm.state.set(1, 26);
        }
        true
    });

    println!("Part 2: {}", last_r4);
    println!("Part 2, fast: {}", fast_part2());

    Ok(())
}
