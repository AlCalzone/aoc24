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

#[derive(Clone, Debug)]
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

fn plot(robots: &[Robot]) -> String {
    let mut ret = String::new();
    let positions: BTreeSet<Vector> = robots.iter().map(|r| r.pos).collect();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if positions.contains(&Vector { x, y }) {
                ret.push('X');
            } else {
                ret.push(' ');
            }
        }
        ret.push('\n');
    }
    ret
}

fn main() {
    let start = Instant::now();
    let mut robots = INPUT.lines().map(Robot::parse).collect::<Vec<_>>();
    let mut counts: [i32; (WIDTH * HEIGHT) as usize] = [0; (WIDTH * HEIGHT) as usize];
    for robot in robots.iter() {
        counts[(robot.pos.y * WIDTH + robot.pos.x) as usize] += 1;
    }

    let mut min_sum: u32 = u32::MAX;
    let mut min_sum_iter: i32 = 0;
    let mut min_state: Vec<Robot> = vec![];

    for i in 0..WIDTH * HEIGHT {
        for robot in robots.iter_mut() {
            counts[(robot.pos.y * WIDTH + robot.pos.x) as usize] -= 1;
            robot.step();
            counts[(robot.pos.y * WIDTH + robot.pos.x) as usize] += 1;
        }

        // Delta-encode the counts at each location and sum them up
        // When robots are clustered, we expect the sum to be low
        let delta_sum: u32 = counts
            .windows(2)
            .map(|w| match w {
                [a, b] => b.abs_diff(*a),
                _ => unreachable!(),
            })
            .sum();

        if delta_sum < min_sum {
            min_sum = delta_sum;
            min_sum_iter = i;
            min_state = robots.clone();
        }
    }

    let elapsed = start.elapsed();

    println!("{}", plot(&min_state));

    println!("Min sum: {} at iteration {}", min_sum, min_sum_iter);
    println!("(took: {:?})", elapsed);
}
