use std::{
    ops::{Add, Div, Mul, Sub},
    time::Instant,
};
use rustc_hash::{FxHashMap, FxHashSet};
use num::integer::gcd;

const INPUT: &'static str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector {
    x: i32,
    y: i32,
}

impl Sub for Point {
    type Output = Vector;

    fn sub(self, other: Point) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Add<Vector> for Point {
    type Output = Point;

    fn add(self, other: Vector) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, other: Vector) -> Point {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<i32> for Vector {
    type Output = Vector;

    fn mul(self, scalar: i32) -> Vector {
        Vector {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

impl Div<i32> for Vector {
    type Output = Vector;

    fn div(self, scalar: i32) -> Vector {
        Vector {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

impl Vector {
    pub fn normalize(&self) -> Vector {
        let gcd = gcd(self.x, self.y);
        Vector {
            x: self.x / gcd,
            y: self.y / gcd,
        }
    }
}

struct Rect {
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

impl Rect {
    fn new(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    fn contains(&self, point: Point) -> bool {
        point.x >= self.x
            && point.x < self.x + self.width as i32
            && point.y >= self.y
            && point.y < self.y + self.height as i32
    }
}

type Frequency = char;

fn main() {
    let start = Instant::now();

    let lines = INPUT.lines().collect::<Vec<&str>>();

    let width = lines[0].len();
    let height = lines.len();
    let field = Rect::new(0, 0, width as u32, height as u32);

    // Find all antennas
    let mut locations: FxHashMap<Frequency, Vec<Point>> = FxHashMap::default();
    for (y, line) in lines.iter().enumerate() {
        for (x, freq) in line.chars().enumerate() {
            if freq == '.' {
                continue;
            }

            locations.entry(freq).or_default().push(Point {
                x: x as i32,
                y: y as i32,
            });
        }
    }

    let mut antinodes: FxHashSet<Point> = FxHashSet::default();
    for points in locations.values() {
        for i in 1..points.len() {
            for j in 0..i {
                let p1 = &points[i];
                let p2 = &points[j];
                let delta = (*p2 - *p1).normalize();

                let mut cur: Point = *p1;
                while field.contains(cur) {
                    antinodes.insert(cur);
                    cur = cur + delta;
                }

                let mut cur = *p1 - delta;
                while field.contains(cur) {
                    antinodes.insert(cur);
                    cur = cur - delta;
                }
            }
        }
    }

    let result = antinodes.len();

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}
