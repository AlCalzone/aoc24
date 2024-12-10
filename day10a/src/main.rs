#![feature(let_chains)]

use std::{fmt::Display, time::Instant};

const INPUT: &'static str = include_str!("input.txt");

const SIZE: usize = 64;

type Point = (i32, i32);

#[derive(Clone)]
struct Field<const R: usize, const C: usize, T: Default> {
    data: [[T; C]; R],
}

impl<const R: usize, const C: usize, T: Default + Copy> Field<R, C, T> {
    pub fn new() -> Self {
        Field {
            data: [[Default::default(); C]; R],
        }
    }

    pub fn set(&mut self, row: i32, col: i32, value: T) {
        self.data[row as usize][col as usize] = value;
    }

    pub fn get(&self, row: i32, col: i32) -> T {
        self.data[row as usize][col as usize]
    }
}

type Map = Field<SIZE, SIZE, Option<u32>>;
type Scores = Field<SIZE, SIZE, u32>;

fn main() {
    let start = Instant::now();

    let mut map: Map = Field::new();
    let mut bottoms: Vec<Point> = Vec::new();
    let mut tops: Vec<Point> = Vec::new();
    let mut scores: Scores = Field::new();

    for (y, line) in INPUT.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            map.set(
                y as i32,
                x as i32,
                match c {
                    '.' => None,
                    '0' => {
                        bottoms.push((x as i32, y as i32));
                        Some(0)
                    }
                    '9' => {
                        tops.push((x as i32, y as i32));
                        scores.set(y as i32, x as i32, 1);
                        Some(9)
                    }
                    c => Some(c.to_digit(10).unwrap()),
                },
            );
        }
    }

    for &(x, y) in &tops {
        compute_scores((x, y), &map, &mut scores, &mut Field::new());
    }

    let result: u32 = bottoms.iter().map(|&(x, y)| scores.get(y, x)).sum();

    let elapsed = start.elapsed();

    print_scores(&scores);

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}

fn print_scores(scores: &Scores) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            print!("{:02} ", scores.get(y as i32, x as i32));
        }
        println!("");
    }
}

fn compute_scores((x, y): Point, map: &Map, scores: &mut Scores, visited: &mut Field<SIZE, SIZE, bool>) {
    let cur_height = map.get(y, x).unwrap();
    for &(x2, y2) in &[(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
        if x2 < 0 || x2 >= SIZE as i32 || y2 < 0 || y2 >= SIZE as i32 {
            continue;
        }
        if visited.get(y2, x2) {
            continue;
        }

        if let Some(height) = map.get(y2, x2)
            && cur_height > 0
            && height == cur_height - 1
        {
            scores.set(y2, x2, scores.get(y2, x2) + 1);
            visited.set(y2, x2, true);
            compute_scores((x2, y2), map, scores, visited);
        }
    }
}
