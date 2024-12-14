use std::{collections::BTreeSet, ops::Add, time::Instant};

const INPUT: &'static str = include_str!("input.txt");
const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    pub fn parse(str: &str) -> Vector {
        let (x, y) = str.split_once(",").unwrap();
        Vector {
            x: x.parse().unwrap(),
            y: y.parse().unwrap(),
        }
    }
}

impl Add for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: (self.x + other.x + WIDTH) % WIDTH,
            y: (self.y + other.y + HEIGHT) % HEIGHT,
        }
    }
}

struct Robot {
    pos: Vector,
    dir: Vector,
}

impl Robot {
    pub fn step(&mut self) {
        self.pos = self.pos + self.dir;
    }
}

impl Robot {
    pub fn parse(line: &str) -> Robot {
        let (pos, dir) = line.split_once(" ").unwrap();
        let (_, pos) = pos.split_once("=").unwrap();
        let (_, dir) = dir.split_once("=").unwrap();
        Robot {
            pos: Vector::parse(pos),
            dir: Vector::parse(dir),
        }
    }
}

fn generate_pbm(robots: &[Robot]) -> String {
    let mut pbm = String::new();
    pbm.push_str("P1\n");
    pbm.push_str(&format!("{} {}\n", WIDTH, HEIGHT));
    let positions: BTreeSet<Vector> = robots.iter().map(|r| r.pos).collect();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if positions.contains(&Vector { x, y }) {
                pbm.push('0');
            } else {
                pbm.push('1');
            }
            pbm.push(' ');
        }
        pbm.push('\n');
    }
    pbm
}

fn main() {
    let start = Instant::now();
    let mut robots = INPUT.lines().map(Robot::parse).collect::<Vec<_>>();

    if let Err(e) = std::fs::create_dir("images") {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            panic!("Could not create images directory: {}", e);
        }
    }

    for i in 1..10000 {
        for robot in robots.iter_mut() {
            robot.step();
        }

        let image = generate_pbm(&robots);
        std::fs::write(format!("images/{:06}.pbm", i), image).unwrap();
    }

    let elapsed = start.elapsed();

    println!("Created 10000 images - have fun searching for the christmas tree!");

    println!("(took: {:?})", elapsed);
}
