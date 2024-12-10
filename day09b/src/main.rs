use std::{
    fmt::{Debug, Display},
    time::Instant,
};

const INPUT: &'static str = include_str!("input.txt");

#[derive(Clone, Copy)]
struct Sector {
    id: Option<usize>,
    length: usize,
}

struct FS {
    blocks: Vec<Sector>,
}

impl Sector {
    pub fn checksum(&self, offset: usize) -> usize {
        match self.id {
            Some(id) => (0..self.length).map(|x| (offset + x) * id).sum(),
            None => 0,
        }
    }
}

impl FS {
    fn parse(input: &str) -> Self {
        let compressed = input.chars().map(|c| c.to_digit(10).unwrap());
        let blocks: Vec<_> = compressed
            .enumerate()
            .flat_map(|(i, chunk)| {
                if i % 2 == 0 {
                    // File
                    vec![Sector {
                        id: Some(i / 2),
                        length: chunk as usize,
                    }]
                } else {
                    // Empty space
                    vec![Sector {
                        id: None,
                        length: chunk as usize,
                    }]
                }
            })
            .filter(|x| x.length > 0)
            .collect();

        Self { blocks }
    }

    fn split_sector(&mut self, i: usize, at: usize) {
        let sector = self.blocks.get_mut(i).unwrap();

        let new_sector = Sector {
            id: sector.id,
            length: sector.length - at,
        };

        sector.length = at;
        self.blocks.insert(i + 1, new_sector);
    }

    pub fn defrag(&mut self) {
        let mut right: usize = self.blocks.len() - 1;
        loop {
            // Find a file we want to move
            while right > 0 && self.blocks[right].id.is_none() {
                right -= 1;
            }
            let file_len = self.blocks[right].length;

            // Find a space to move it to
            let mut left: usize = 0;
            while left < right
                && (self.blocks[left].id.is_some() || self.blocks[left].length < file_len)
            {
                left += 1;
            }
            if left >= right {
                // No space for this one
                if right == 0 {
                    break;
                }
                right -= 1;
                continue;
            }

            // If the sector is too big, split it
            if self.blocks[left].length > file_len {
                self.split_sector(left, file_len);
                // This changes the right pointer
                right += 1;
            }

            self.blocks.swap(left, right);
        }
    }

    pub fn checksum(&self) -> usize {
        let mut offset = 0;
        let mut ret: usize = 0;
        for sector in self.blocks.iter() {
            ret += sector.checksum(offset);
            offset += sector.length;
        }
        ret
    }
}

// impl Display for FS {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         for (i, ref x) in self.blocks.iter().enumerate() {
//             for _ in 0..x.length {
//                 if let Some(x) = x.id {
//                     write!(f, "{}", x)?;
//                 } else {
//                     write!(f, ".")?;
//                 }
//             }
//         }
//         Ok(())
//     }
// }

fn main() {
    let start = Instant::now();

    let mut fs = FS::parse(INPUT);
    fs.defrag();
    let result = fs.checksum();

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}
