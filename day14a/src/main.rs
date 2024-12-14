use std::{ops::Add, time::Instant};

const INPUT: &'static str = include_str!("input.txt");
const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

#[derive(Clone, Copy, Debug)]
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

fn main() {
    let start = Instant::now();
    let mut robots = INPUT.lines().map(Robot::parse).collect::<Vec<_>>();
    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.step();
        }
    }
    let q1 = robots.iter().filter_map(|r| {
        if r.pos.x < WIDTH / 2 && r.pos.y < HEIGHT / 2 {
            Some(r)
        } else {
            None
        }
    });
    let q2 = robots.iter().filter_map(|r| {
        if r.pos.x > WIDTH / 2 && r.pos.y < HEIGHT / 2 {
            Some(r)
        } else {
            None
        }
    });
    let q3 = robots.iter().filter_map(|r| {
        if r.pos.x < WIDTH / 2 && r.pos.y > HEIGHT / 2 {
            Some(r)
        } else {
            None
        }
    });
    let q4 = robots.iter().filter_map(|r| {
        if r.pos.x > WIDTH / 2 && r.pos.y > HEIGHT / 2 {
            Some(r)
        } else {
            None
        }
    });

    let result = q1.count() * q2.count() * q3.count() * q4.count();

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}
