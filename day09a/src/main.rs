use std::{
    fmt::{Debug, Display},
    time::Instant,
};

const INPUT: &'static str = include_str!("input.txt");

struct FS {
    blocks: Vec<Option<usize>>,
}

impl FS {
    fn parse(input: &str) -> Self {
        let compressed: Vec<_> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let size: usize = compressed.iter().map(|&x| x as usize).sum();

        let mut blocks = vec![None; size];
        let mut offset: usize = 0;
        for (i, chunk) in compressed.chunks(2).enumerate() {
            match chunk {
                &[file] => {
                    for _ in 0..file {
                        blocks[offset] = Some(i);
                        offset += 1;
                    }
                    continue;
                }
                &[file, space] => {
                    for _ in 0..file {
                        blocks[offset] = Some(i);
                        offset += 1;
                    }
                    offset += space as usize;
                    continue;
                }
                _ => unreachable!(),
            }
        }

        Self { blocks }
    }

    pub fn defrag(&mut self) {
        let mut left: usize = 0;
        let mut right: usize = self.blocks.len() - 1;
        loop {
            while self.blocks[left].is_some() && left < right {
                left += 1;
            }
            while self.blocks[right].is_none() && left < right {
                right -= 1;
            }
            if left >= right {
                break;
            }
            self.blocks.swap(left, right);
        }
    }

    pub fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .enumerate()
            .filter_map(|(i, &x)| x.map(|x| i * x))
            .sum()
    }
}

impl Display for FS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (i, &x) in self.blocks.iter().enumerate() {
            if let Some(x) = x {
                write!(f, "{}", x)?;
            } else {
                write!(f, ".")?;
            }
        }
        Ok(())
    }
}

fn main() {
    let start = Instant::now();

    let mut fs = FS::parse(INPUT);
    fs.defrag();
    let result = fs.checksum();

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}
