use anyhow::Result;

fn expand(data: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(data.len() * 2 + 1);

    let mut chunk = data.to_vec();
    out.extend(chunk.iter());
    out.push(0);

    chunk.reverse();
    for v in chunk.iter_mut() {
        *v = if *v == 0 { 1 } else { 0 }
    }

    out.extend(chunk);

    out
}

fn contract(data: &[u8]) -> Vec<u8> {
    let mut input = data.to_vec();
    let mut out = Vec::with_capacity(data.len() / 2);

    loop {
        for i in 0..(input.len() / 2) {
            let a = input[i * 2];
            let b = input[i * 2 + 1];
            out.push(if a == b { 1 } else { 0 })
        }

        if out.len() % 2 == 1 {
            break;
        }

        input = out;
        out = Vec::with_capacity(input.len() / 2);
    }

    out
}

fn expand_to(input: &[u8], target: usize) -> Vec<u8> {
    let mut data = input.to_vec();

    while data.len() < target {
        data = expand(&data);
    }

    data[0..=target].to_vec()
}

fn format(data: &[u8]) -> String {
    data.iter()
        .map(|i| format!("{}", i))
        .collect::<Vec<String>>()
        .join("")
}

fn main() -> Result<()> {
    let input = &[1, 1, 1, 0, 1, 0, 0, 0, 1, 1, 0, 0, 1, 0, 1, 0, 0];

    let expanded = expand_to(input, 272);
    let contracted = contract(&expanded);
    println!("Part 1: {}", format(&contracted));

    let expanded = expand_to(input, 35651584);
    let contracted = contract(&expanded);
    println!("Part 2: {}", format(&contracted));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand() {
        assert_eq!(expand(&[1]), vec![1, 0, 0]);
        assert_eq!(expand(&[0]), vec![0, 0, 1]);
        assert_eq!(
            expand(&[1, 1, 1, 1, 1]),
            vec![1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0]
        );

        assert_eq!(
            expand(&[1, 0, 0, 0, 0]),
            vec![1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0]
        );
        assert_eq!(
            expand(&[1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0]),
            vec![1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1, 1, 0]
        );
        assert_eq!(
            expand(&[1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0]),
            vec![1, 1, 1, 1, 0, 0, 0, 0, 1, 0, 1, 0, 0, 1, 0, 1, 0, 1, 1, 1, 1, 0, 0, 0, 0]
        );
    }

    #[test]
    fn test_contract() {
        assert_eq!(
            contract(&[1, 1, 0, 0, 1, 0, 1, 1, 0, 1, 0, 0]),
            vec![1, 0, 0]
        )
    }

    #[test]
    fn test_example() {
        let initial = [1, 0, 0, 0, 0];
        let expanded = expand_to(&initial, 20);
        assert_eq!(
            expanded,
            vec![1, 0, 0, 0, 0, 0, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 1, 1]
        );
        let contracted = contract(&expanded);
        assert_eq!(contracted, vec![0, 1, 1, 0, 0]);
    }
}
