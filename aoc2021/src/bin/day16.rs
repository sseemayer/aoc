use anyhow::Result;
use aoc2021::io::read_all;
use thiserror::Error;

fn hex_to_bin(data: &str) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    for c in data.trim().chars() {
        let mut d = c.to_digit(16).expect("hex characters");
        let mut l = vec![0; 4];

        for i in 0..4 {
            l[3 - i] = (d % 2) as u8;
            d >>= 1;
        }

        out.extend(l);
    }
    Ok(out)
}

fn bin_to_dec(data: &[u8]) -> usize {
    let mut out = 0;

    for &n in data {
        out <<= 1;
        out += n as usize;
    }

    out
}

#[derive(Debug, PartialEq, Eq)]
struct Packet {
    version: usize,
    payload: Payload,
}

#[derive(Debug, PartialEq, Eq)]
enum Payload {
    Literal {
        number: Vec<u8>,
    },
    Operator {
        opcode: usize,
        sub_packets: Vec<Packet>,
    },
}

#[derive(Error, Debug)]
enum Error {
    #[error("Encountered a bad packet: {:?}", .0)]
    BadPacket(Vec<u8>),
}

impl Packet {
    fn parse(data: &[u8]) -> Result<(Self, &[u8])> {
        let version = bin_to_dec(&data[0..3]);
        let type_id = bin_to_dec(&data[3..6]);
        let next_bit = data[6];

        let (payload, data_after_reading) = match (type_id, next_bit) {
            (4, _) => {
                // parse a literal packet
                let mut number = Vec::new();
                let mut i = 6;

                loop {
                    number.extend(&data[(i + 1)..(i + 5)]);
                    if data[i] != 1 {
                        break;
                    }
                    i += 5;
                }

                (Payload::Literal { number }, &data[i + 5..])
            }
            (opcode, 0) => {
                // parse a 15-bit operator packet

                let length = bin_to_dec(&data[7..22]);
                let mut sub_data = &data[22..(22 + length)];
                let mut sub_packets = Vec::new();
                while sub_data.len() > 0 {
                    let (sp, ss) = Packet::parse(sub_data)?;
                    sub_packets.push(sp);
                    sub_data = ss;
                }

                (
                    Payload::Operator {
                        opcode,
                        sub_packets,
                    },
                    &data[22 + length..],
                )
            }
            (opcode, 1) => {
                // parse a 11-bit operator packet
                let n_packets = bin_to_dec(&data[7..18]);
                let mut sub_data = &data[18..];
                let mut sub_packets = Vec::new();
                for _ in 0..n_packets {
                    let (sp, ss) = Packet::parse(sub_data)?;
                    sub_packets.push(sp);
                    sub_data = ss;
                }

                (
                    Payload::Operator {
                        opcode,
                        sub_packets,
                    },
                    sub_data,
                )
            }
            _ => return Err(Error::BadPacket(data.to_vec()).into()),
        };

        Ok((Packet { version, payload }, data_after_reading))
    }

    fn version_sum(&self) -> usize {
        self.version
            + match &self.payload {
                Payload::Literal { .. } => 0,
                Payload::Operator { sub_packets, .. } => {
                    sub_packets.iter().map(|sp| sp.version_sum()).sum()
                }
            }
    }

    fn value(&self) -> usize {
        match &self.payload {
            Payload::Literal { number } => bin_to_dec(&number),
            Payload::Operator {
                opcode,
                sub_packets,
            } => {
                let sub_values = sub_packets.iter().map(|sp| sp.value());

                match opcode {
                    0 => {
                        // sum
                        sub_values.sum()
                    }
                    1 => {
                        // product
                        sub_values.product()
                    }
                    2 => {
                        // minimum
                        sub_values.min().unwrap_or(0)
                    }
                    3 => {
                        // maximum
                        sub_values.max().unwrap_or(0)
                    }
                    5 => {
                        // greater_than
                        let sv: Vec<_> = sub_values.collect();
                        if sv[0] > sv[1] {
                            1
                        } else {
                            0
                        }
                    }
                    6 => {
                        // less_than
                        let sv: Vec<_> = sub_values.collect();
                        if sv[0] < sv[1] {
                            1
                        } else {
                            0
                        }
                    }
                    7 => {
                        // equal_to
                        let sv: Vec<_> = sub_values.collect();
                        if sv[0] == sv[1] {
                            1
                        } else {
                            0
                        }
                    }
                    _ => panic!("Bad opcode {}", opcode),
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let data = read_all("data/day16/input")?;
    let data = hex_to_bin(&data)?;

    let (parsed, _) = Packet::parse(&data)?;

    println!("Part 1: {}", parsed.version_sum());
    println!("Part 2: {}", parsed.value());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hex_to_bin() -> Result<()> {
        assert_eq!(
            hex_to_bin("0123")?,
            vec![0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 1, 1]
        );

        Ok(())
    }

    #[test]
    fn test_bin_to_dec() {
        assert_eq!(bin_to_dec(&[0]), 0);
        assert_eq!(bin_to_dec(&[1]), 1);
        assert_eq!(bin_to_dec(&[0, 1]), 1);
        assert_eq!(bin_to_dec(&[1, 0]), 2);
        assert_eq!(bin_to_dec(&[1, 1]), 3);
        assert_eq!(bin_to_dec(&[1, 0, 1, 0, 1, 0]), 42);
    }

    #[test]
    fn test_parsing() -> Result<()> {
        assert_eq!(
            Packet::parse(&hex_to_bin("D2FE28")?)?,
            (
                Packet {
                    version: 6,
                    payload: Payload::Literal {
                        number: vec![0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 1]
                    }
                },
                &[0, 0, 0][..]
            )
        );

        assert_eq!(
            Packet::parse(&hex_to_bin("38006F45291200")?)?,
            (
                Packet {
                    version: 1,
                    payload: Payload::Operator {
                        opcode: 6,
                        sub_packets: vec![
                            Packet {
                                version: 6,
                                payload: Payload::Literal {
                                    number: vec![1, 0, 1, 0]
                                },
                            },
                            Packet {
                                version: 2,
                                payload: Payload::Literal {
                                    number: vec![0, 0, 0, 1, 0, 1, 0, 0]
                                },
                            },
                        ]
                    }
                },
                &[0, 0, 0, 0, 0, 0, 0][..]
            )
        );

        assert_eq!(
            Packet::parse(&hex_to_bin("EE00D40C823060")?)?,
            (
                Packet {
                    version: 7,
                    payload: Payload::Operator {
                        opcode: 3,
                        sub_packets: vec![
                            Packet {
                                version: 2,
                                payload: Payload::Literal {
                                    number: vec![0, 0, 0, 1]
                                },
                            },
                            Packet {
                                version: 4,
                                payload: Payload::Literal {
                                    number: vec![0, 0, 1, 0]
                                },
                            },
                            Packet {
                                version: 1,
                                payload: Payload::Literal {
                                    number: vec![0, 0, 1, 1]
                                },
                            },
                        ]
                    }
                },
                &[0, 0, 0, 0, 0][..]
            )
        );

        Ok(())
    }
}
