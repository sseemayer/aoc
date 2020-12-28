use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Read},
};

use snafu::{ResultExt, Snafu};

#[derive(Debug, Snafu)]
enum Error {
    #[snafu(display("I/O error: {}", source))]
    Io { source: std::io::Error },

    #[snafu(display("Number parsing error for '{}': {}", data, source))]
    ParseNumber {
        data: String,
        source: std::num::ParseIntError,
    },

    #[snafu(display("Rule parsing error for '{}'", data))]
    ParseRule { data: String },
}

type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone)]
struct Matching<'a> {
    messages: Vec<&'a str>,
}

impl<'a> From<Vec<&'a str>> for Matching<'a> {
    fn from(messages: Vec<&'a str>) -> Self {
        Self { messages }
    }
}

impl<'a> From<&'a str> for Matching<'a> {
    fn from(s: &'a str) -> Self {
        Self { messages: vec![s] }
    }
}

impl<'a> Matching<'a> {
    fn has_complete(&self) -> bool {
        self.messages.iter().filter(|m| m.len() == 0).count() > 0
    }
}

trait Match {
    fn matches<'a>(
        &self,
        messages: Matching<'a>,
        grammar: &Grammar,
        recursion_depth: usize,
    ) -> Matching<'a>;
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Grammar {
    rules: HashMap<usize, Rule>,
}

fn fmt_indent(indent: usize) -> String {
    let mut out = String::new();
    for _ in 0..indent {
        out.extend("  ".chars());
    }
    out
}

fn parse_all<F: Read>(f: &mut F) -> Result<(Grammar, Vec<String>)> {
    enum ParserMode {
        Rules,
        Messages,
    }

    let mut rules = HashMap::new();
    let mut messages = Vec::new();

    let mut current_mode = ParserMode::Rules;
    for line in BufReader::new(f).lines() {
        let line = line.context(Io)?;
        match current_mode {
            ParserMode::Rules => {
                if line.trim().len() == 0 {
                    current_mode = ParserMode::Messages;
                    continue;
                }

                let rule: Rule = line.parse()?;
                rules.insert(rule.id, rule);
            }
            ParserMode::Messages => {
                messages.push(line.to_string());
            }
        }
    }

    let grammar = Grammar { rules };
    Ok((grammar, messages))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rule {
    id: usize,
    sub_rules: Vec<SubRule>,
}

impl Match for Rule {
    fn matches<'a>(
        &self,
        messages: Matching<'a>,
        grammar: &Grammar,
        recursion_depth: usize,
    ) -> Matching<'a> {
        let mut out = Vec::new();
        for sr in &self.sub_rules {
            out.extend(
                sr.matches(messages.clone(), grammar, recursion_depth + 1)
                    .messages,
            );
        }

        out.into()
    }
}

impl std::str::FromStr for Rule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let tokens: Vec<&str> = s.split(":").collect();
        if tokens.len() != 2 {
            return Err(Error::ParseRule {
                data: s.to_string(),
            });
        }

        let id: usize = tokens[0].parse().context(ParseNumber {
            data: tokens[0].to_string(),
        })?;

        let sub_rules: Vec<SubRule> = tokens[1]
            .split("|")
            .map(|r| r.trim().parse())
            .collect::<Result<_>>()?;

        Ok(Rule { id, sub_rules })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SubRule {
    tokens: Vec<Token>,
}

impl Match for SubRule {
    fn matches<'a>(
        &self,
        messages: Matching<'a>,
        grammar: &Grammar,
        recursion_depth: usize,
    ) -> Matching<'a> {
        if messages.messages.is_empty() {
            return Matching { messages: vec![] };
        }

        let mut out = messages.clone();
        let mut next = Vec::new();
        for tkn in &self.tokens {
            next.extend(tkn.matches(out, grammar, recursion_depth + 1).messages);

            out = next.into();
            next = Vec::new();
        }

        // println!(
        //     "{}{:?}: matched {:?} to {:?}",
        //     fmt_indent(recursion_depth),
        //     self,
        //     messages,
        //     out
        // );

        out
    }
}

impl std::str::FromStr for SubRule {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        let tokens: Vec<Token> = s
            .split_whitespace()
            .map(|t| t.parse())
            .collect::<Result<_>>()?;

        Ok(SubRule { tokens })
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Token {
    Rule(usize),
    Terminal(String),
}

impl Match for Token {
    fn matches<'a>(
        &self,
        messages: Matching<'a>,
        grammar: &Grammar,
        recursion_depth: usize,
    ) -> Matching<'a> {
        match self {
            Token::Rule(rule_id) => {
                grammar.rules[rule_id].matches(messages, grammar, recursion_depth + 1)
            }
            Token::Terminal(txt) => messages
                .messages
                .iter()
                .filter_map(|m| {
                    if m.starts_with(txt) {
                        Some(&m[txt.len()..])
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>()
                .into(),
        }
    }
}

impl std::str::FromStr for Token {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if &s[..1] == "\"" && &s[s.len() - 1..] == "\"" {
            let text = s[1..s.len() - 1].to_string();
            Ok(Token::Terminal(text))
        } else {
            let rule_id: usize = s.parse().context(ParseNumber {
                data: s.to_string(),
            })?;
            Ok(Token::Rule(rule_id))
        }
    }
}

fn main() -> Result<()> {
    let (grammar, messages) = parse_all(&mut File::open("data/day19/input").context(Io)?)?;

    let matched_messages = messages
        .iter()
        .filter(|m| {
            grammar.rules[&0]
                .matches((&m[..]).into(), &grammar, 0)
                .has_complete()
        })
        .count();

    println!("Part 1: got {} matching messages", matched_messages);

    let mut grammar = grammar;
    grammar.rules.insert(8, "8: 42 | 42 8".parse::<Rule>()?);
    grammar
        .rules
        .insert(11, "11: 42 31 | 42 11 31".parse::<Rule>()?);

    let matched_messages = messages
        .iter()
        .filter(|m| {
            grammar.rules[&0]
                .matches((&m[..]).into(), &grammar, 0)
                .has_complete()
        })
        .count();

    println!("Part 2: got {} matching messages", matched_messages);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_token() -> Result<()> {
        assert_eq!(
            "\"hi\"".parse::<Token>()?,
            Token::Terminal("hi".to_string())
        );

        assert_eq!("42".parse::<Token>()?, Token::Rule(42));

        Ok(())
    }

    #[test]
    fn test_parse_subrule() -> Result<()> {
        assert_eq!(
            "\"hello\" 1 2 \"world\" 3 4".parse::<SubRule>()?,
            SubRule {
                tokens: vec![
                    Token::Terminal("hello".to_string()),
                    Token::Rule(1),
                    Token::Rule(2),
                    Token::Terminal("world".to_string()),
                    Token::Rule(3),
                    Token::Rule(4),
                ]
            }
        );

        Ok(())
    }

    #[test]
    fn test_parse_rule() -> Result<()> {
        assert_eq!(
            "2: 1 3 | \"abc\" 1".parse::<Rule>()?,
            Rule {
                id: 2,
                sub_rules: vec![
                    SubRule {
                        tokens: vec![Token::Rule(1), Token::Rule(3)],
                    },
                    SubRule {
                        tokens: vec![Token::Terminal("abc".to_string()), Token::Rule(1)]
                    }
                ]
            }
        );

        Ok(())
    }

    fn assert_compare_match(grammar: &Grammar, messages: &[String], expected: &[bool]) {
        let r0 = &grammar.rules[&0];
        for (i, (msg, exp_match)) in messages.iter().zip(expected).enumerate() {
            println!("CASE #{}: \"{}\"", i, msg);
            let matches = r0.matches((&msg[..]).into(), &grammar, 0);
            if matches.has_complete() && !exp_match {
                panic!("Message '{}'; matched cleanly when it shouldn't have", msg);
            } else if !matches.has_complete() && *exp_match {
                panic!(
                    "Message '{}' should have matched but got leftovers: {:?}",
                    msg, matches
                );
            }
        }
    }

    #[test]
    fn test_matching() -> Result<()> {
        let (grammar, messages) = parse_all(&mut "0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"\n\nababbb\nbababa\nabbbab\naaabbb\naaaabbb".as_bytes())?;
        let expected = vec![true, false, true, false, false];

        assert_compare_match(&grammar, &messages[..], &expected);
        Ok(())
    }

    #[test]
    fn test_extended_matching() -> Result<()> {
        let (mut grammar, messages) =
            parse_all(&mut File::open("data/day19/example").context(Io)?)?;

        let expected = vec![
            false, // abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
            true,  // bbabbbbaabaabba
            false, // babbbbaabbbbbabbbbbbaabaaabaaa
            false, // aaabbbbbbaaaabaababaabababbabaaabbababababaaa
            false, // bbbbbbbaaaabbbbaaabbabaaa
            false, // bbbababbbbaaaaaaaabbababaaababaabab
            true,  // ababaaaaaabaaab
            true,  // ababaaaaabbbaba
            false, // baabbaaaabbaaaababbaababb
            false, // abbbbabbbbaaaababbbbbbaaaababb
            false, // aaaaabbaabaaaaababaa
            false, // aaaabbaaaabbaaa
            false, // aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
            false, // babaaabbbaaabaababbaabababaaab
            false, // aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
        ];

        assert_compare_match(&grammar, &messages[..], &expected);

        println!("\n====== patched rules ======");
        grammar.rules.insert(8, "8: 42 | 42 8".parse()?);
        grammar.rules.insert(11, "11: 42 31 | 42 11 31".parse()?);

        let expected = vec![
            false, // abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
            true,  // bbabbbbaabaabba
            true,  // babbbbaabbbbbabbbbbbaabaaabaaa
            true,  // aaabbbbbbaaaabaababaabababbabaaabbababababaaa
            true,  // bbbbbbbaaaabbbbaaabbabaaa
            true,  // bbbababbbbaaaaaaaabbababaaababaabab
            true,  // ababaaaaaabaaab
            true,  // ababaaaaabbbaba
            true,  // baabbaaaabbaaaababbaababb
            true,  // abbbbabbbbaaaababbbbbbaaaababb
            true,  // aaaaabbaabaaaaababaa
            false, // aaaabbaaaabbaaa
            true,  // aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
            false, // babaaabbbaaabaababbaabababaaab
            true,  // aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba
        ];
        assert_compare_match(&grammar, &messages[..], &expected);

        Ok(())
    }

    #[test]
    fn test_extended_matching_2() -> Result<()> {
        let (grammar, messages_and_expected) =
            parse_all(&mut File::open("data/day19/example2").context(Io)?)?;
        // let (grammar, messages) = parse_all(
        //     &mut "0: 1\n1: \"a\" | \"a\" \"b\" 1 \"c\"\n\nb\na\naa\naaa\nabac\naaba\nabababaccc\nabab"
        //         .as_bytes(),
        // )?;

        let mut messages: Vec<String> = Vec::new();
        let mut expected: Vec<bool> = Vec::new();

        for mae in messages_and_expected {
            let tokens: Vec<&str> = mae.split("#").collect();
            messages.push(tokens[0].trim().to_string());
            expected.push(tokens[1].trim().parse().unwrap());
        }

        assert_compare_match(&grammar, &messages[..], &expected);

        Ok(())
    }
}
