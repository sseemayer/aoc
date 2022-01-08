fn simulate(input: usize, limit: usize) -> (Vec<usize>, usize) {
    let mut out = Vec::with_capacity(limit);

    let mut pos = 0;
    for len in 0..limit {
        out.insert(pos, len);
        pos = (pos + input) % (len + 1) + 1;

        // println!("{:>9} {:>9} {:?}", len, pos, out);
    }

    out.insert(pos, limit);

    let val_after = out[pos + 1];

    (out, val_after)
}

fn simulate_only_after_zero(input: usize, limit: usize) -> usize {
    let mut pos = 0;
    let mut second = 0;

    for len in 0..limit {
        if pos == 1 {
            second = len;
        }

        pos = (pos + input) % (len + 1) + 1;

        // println!("{:>9} {:>9} {}", len, pos, second);
    }

    second
}

fn main() {
    let (_buf, n) = simulate(371, 2017);
    println!("Part 1: {}", n);

    let v = simulate_only_after_zero(371, 50_000_000);
    println!("Part 2: {}", v);
}
