use failure::{format_err, Error};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
type Result<T> = std::result::Result<T, Error>;

#[derive(Clone, Debug)]
struct Layer {
    data: Vec<Vec<u8>>,
}

impl std::fmt::Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for row in self.data.iter() {
            for c in row {
                let p = match c {
                    0 => "░",
                    1 => "█",
                    2 => " ",
                    _ => "?",
                };
                write!(f, "{}", p)?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

impl Layer {
    fn from_buffer(buf: &[u8], width: usize, height: usize) -> Self {
        let mut pos = 0;
        let mut data = Vec::new();
        while pos < buf.len() {
            let row: Vec<u8> = buf[pos..(pos + width)]
                .into_iter()
                .filter(|v| **v > 47 && **v < 58)
                .map(|v| {
                    println!("{}", v);
                    *v - 48
                })
                .collect();
            data.push(row);

            pos += width;
        }

        Layer { data }
    }

    fn count_pixels(&self) -> HashMap<u8, usize> {
        let mut out = HashMap::new();

        for row in self.data.iter() {
            for pix in row {
                out.entry(*pix).and_modify(|count| *count += 1).or_insert(1);
            }
        }

        out
    }

    /// Blend this layer with another layer that will be considered below the own image
    fn blend(&self, other: &Layer) -> Layer {
        let data: Vec<Vec<u8>> = self
            .data
            .iter()
            .zip(other.data.iter())
            .map(|(row_a, row_b)| {
                row_a
                    .iter()
                    .zip(row_b.iter())
                    .map(|(a, b)| if *a == 2 { *b } else { *a })
                    .collect()
            })
            .collect();

        Layer { data }
    }
}

#[derive(Debug)]
struct Spif {
    layers: Vec<Layer>,
}

impl Spif {
    fn read(f: &mut dyn Read, width: usize, height: usize) -> Result<Self> {
        let layer_size = width * height;
        println!("One layer is {} bytes", layer_size);

        let mut buf = vec![0; layer_size];
        let mut out = Vec::new();
        while f.read(&mut buf)? > 0 {
            out.push(Layer::from_buffer(&buf[..], width, height));
        }

        Ok(Spif { layers: out })
    }

    fn flatten(&self) -> Layer {
        self.layers[1..]
            .iter()
            .fold(self.layers[0].clone(), |acc, x| acc.blend(x))
    }
}

impl std::fmt::Display for Spif {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        for layer in self.layers.iter() {
            write!(f, "{}\n\n", layer)?;
        }
        Ok(())
    }
}

fn main() -> Result<()> {
    let mut f = File::open("data/day08/input")?;
    let img = Spif::read(&mut f, 25, 6)?;

    println!("Got image:\n{}", img);

    let mut i_min = 0;
    let mut n_min = std::usize::MAX;
    for (i, layer) in img.layers.iter().enumerate() {
        let counts = layer.count_pixels();

        if counts[&0] < n_min {
            n_min = counts[&0];
            i_min = i;
        }
    }

    let counts_min = img.layers[i_min].count_pixels();

    println!("Checksum: {}", counts_min[&1] * counts_min[&2]);

    let final_layer = img.flatten();

    println!("Flattened image:\n{}", final_layer);

    Ok(())
}
