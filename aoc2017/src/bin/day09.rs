use anyhow::{anyhow, Context, Result};

#[derive(Debug, Clone, PartialEq, Eq)]
enum Content {
    C(String),
    G(Group),
}

impl Content {
    fn score(&self, depth: usize) -> usize {
        match self {
            Content::C(_) => 0,
            Content::G(g) => g.score(depth),
        }
    }

    fn count_garbage_characters(&self) -> usize {
        match self {
            Content::C(content) => {
                let chars: Vec<char> = content.chars().collect();
                let mut count = 0;
                let mut pos = 0;
                let mut in_garbage = false;

                while pos < content.len() {
                    let c = chars[pos];
                    match (in_garbage, c) {
                        (false, '<') => {
                            in_garbage = true;
                        }
                        (true, '!') => {
                            pos += 1;
                        }
                        (true, '>') => {
                            in_garbage = false;
                        }
                        (true, _) => {
                            count += 1;
                        }
                        (false, _) => {
                            // non-garbage characters are not counted
                        }
                    }
                    pos += 1;
                }

                count
            }
            Content::G(g) => g.count_garbage_characters(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Group(Vec<Content>);

impl std::str::FromStr for Group {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let chars: Vec<char> = s.chars().collect();

        // println!("Parsing '{}'", s);

        if chars[0] != '{' {
            return Err(anyhow!("Group must start with '{{': '{}'", s));
        }

        if chars[chars.len() - 1] != '}' {
            return Err(anyhow!("Group must end with '}}': '{}'", s));
        }

        let mut pos = 1;
        let mut in_garbage = false;
        let mut buffer: Vec<char> = Vec::new();
        let mut open_group_pos: Vec<usize> = Vec::new();
        let mut contents: Vec<Content> = Vec::new();
        while pos < chars.len() - 1 {
            let c = chars[pos];

            match (in_garbage, !open_group_pos.is_empty(), c) {
                (false, _, '{') => {
                    // group open
                    // println!("Encountered group opening at {}", pos);
                    open_group_pos.push(pos);
                }
                (false, _, '}') => {
                    // group close
                    if open_group_pos.is_empty() {
                        return Err(anyhow!(
                            "Encountered '}}' without matching group opening: '{}'",
                            s
                        ));
                    }

                    let go = open_group_pos.pop().unwrap();
                    // println!(
                    //     "Encountered group closing at {} (matching opening at {})",
                    //     pos, go
                    // );
                    if open_group_pos.is_empty() {
                        let group = &s[go..=pos];
                        // println!("Parse subgroup: '{}'", group);
                        let group: Group = group.parse()?;
                        contents.push(Content::G(group));
                    }
                }
                (false, _, '<') => {
                    // garbage start
                    // println!("garbage start at {}", pos);
                    in_garbage = true;

                    if open_group_pos.is_empty() {
                        buffer.push(c);
                    }
                }
                (false, false, ',') => {
                    if !buffer.is_empty() {
                        contents.push(Content::C(buffer.iter().collect()));
                        buffer.clear();
                    }
                }
                (true, _, '!') => {
                    // escape within garbage
                    if open_group_pos.is_empty() {
                        buffer.push(c);
                        buffer.push(chars[pos + 1]);
                    }
                    pos += 1;
                }
                (true, _, '>') => {
                    // garbage end
                    // println!("garbage end at {}", pos);
                    in_garbage = false;
                    if open_group_pos.is_empty() {
                        buffer.push(c);
                    }
                }
                (_, false, _) => {
                    // character data
                    buffer.push(c);
                }
                _ => {}
            }

            pos += 1;
        }

        if !buffer.is_empty() {
            contents.push(Content::C(buffer.iter().collect()));
            buffer.clear();
        }

        // println!("Parse result: {:?}", contents);
        Ok(Group(contents))
    }
}

impl Group {
    fn score(&self, depth: usize) -> usize {
        depth + 1 + self.0.iter().map(|c| c.score(depth + 1)).sum::<usize>()
    }

    fn count_garbage_characters(&self) -> usize {
        self.0
            .iter()
            .map(|c| c.count_garbage_characters())
            .sum::<usize>()
    }
}

fn main() -> Result<()> {
    let group: Group = std::fs::read_to_string("data/day09/input")
        .context("Reading input file")?
        .trim()
        .parse()?;

    println!("Part 1: {}", group.score(0));
    println!("Part 2: {}", group.count_garbage_characters());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_parsing() -> Result<()> {
        use Content::{C, G};

        assert_eq!("{}".parse::<Group>()?, Group(vec![])); // 1 group.
        assert_eq!(
            "{{{}}}".parse::<Group>()?,
            Group(vec![G(Group(vec![G(Group(vec![]))]))])
        ); // 3 groups.
        assert_eq!(
            "{{},{}}".parse::<Group>()?,
            Group(vec![G(Group(vec![])), G(Group(vec![]))])
        ); // also 3 groups.

        assert_eq!(
            "{{{},{},{{}}}}".parse::<Group>()?,
            Group(vec![G(Group(vec![
                G(Group(vec![])),
                G(Group(vec![])),
                G(Group(vec![G(Group(vec![]))]))
            ]))])
        ); // 6 groups.

        assert_eq!(
            "{<{},{},{{}}>}".parse::<Group>()?,
            Group(vec![C("<{},{},{{}}>".to_string())])
        ); // 1 group (which itself contains garbage).

        assert_eq!(
            "{<a>,<a>,<a>,<a>}".parse::<Group>()?,
            Group(vec![
                C("<a>".to_string()),
                C("<a>".to_string()),
                C("<a>".to_string()),
                C("<a>".to_string()),
            ])
        ); // 1 group.

        assert_eq!(
            "{{<a>},{<a>},{<a>},{<a>}}".parse::<Group>()?,
            Group(vec![
                G(Group(vec![C("<a>".to_string())])),
                G(Group(vec![C("<a>".to_string())])),
                G(Group(vec![C("<a>".to_string())])),
                G(Group(vec![C("<a>".to_string())])),
            ])
        ); // 5 groups.

        assert_eq!(
            "{{<!>},{<!>},{<!>},{<a>}}".parse::<Group>()?,
            Group(vec![G(Group(vec![C("<!>},{<!>},{<!>},{<a>".to_string())]))])
        ); // 2 groups (since all but the last > are canceled).

        Ok(())
    }

    #[test]
    fn test_scoring() -> Result<()> {
        //{}, score of 1.
        assert_eq!("{}".parse::<Group>()?.score(0), 1);

        //{{{}}}, score of 1 + 2 + 3 = 6.
        assert_eq!("{{{}}}".parse::<Group>()?.score(0), 6);

        //{{},{}}, score of 1 + 2 + 2 = 5.
        assert_eq!("{{},{}}".parse::<Group>()?.score(0), 5);

        //{{{},{},{{}}}}, score of 1 + 2 + 3 + 3 + 3 + 4 = 16.
        assert_eq!("{{{},{},{{}}}}".parse::<Group>()?.score(0), 16);

        //{<a>,<a>,<a>,<a>}, score of 1.
        assert_eq!("{<a>,<a>,<a>,<a>}".parse::<Group>()?.score(0), 1);

        //{{<ab>},{<ab>},{<ab>},{<ab>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
        assert_eq!(
            "{{<ab>},{<ab>},{<ab>},{<ab>}}".parse::<Group>()?.score(0),
            9
        );

        //{{<!!>},{<!!>},{<!!>},{<!!>}}, score of 1 + 2 + 2 + 2 + 2 = 9.
        assert_eq!(
            "{{<!!>},{<!!>},{<!!>},{<!!>}}".parse::<Group>()?.score(0),
            9
        );

        //{{<a!>},{<a!>},{<a!>},{<ab>}}, score of 1 + 2 = 3.
        assert_eq!(
            "{{<a!>},{<a!>},{<a!>},{<ab>}}".parse::<Group>()?.score(0),
            3
        );

        Ok(())
    }
}
