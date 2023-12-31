pub fn gcd(mut a: usize, mut b: usize) -> usize {
    while b > 0 {
        (a, b) = (b, a % b);
    }
    a
}

pub fn gcd_multiple(n: &[usize]) -> usize {
    n.iter().cloned().reduce(gcd).unwrap_or(0)
}

pub fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

pub fn lcm_multiple(n: &[usize]) -> usize {
    n.iter().cloned().reduce(lcm).unwrap_or(0)
}
