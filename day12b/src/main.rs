#![feature(let_chains)]

use std::time::Instant;

const INPUT: &'static str = include_str!("input.txt");

#[derive(Debug)]
struct Region {
    plant: char,
    area: usize,
    corners: usize,
}

impl Region {
    fn price(&self) -> usize {
        self.area * self.corners
    }
}

fn main() {
    let start = Instant::now();

    let plots: Vec<Vec<_>> = INPUT.lines().map(|l| l.chars().collect()).collect();
    let width = plots[0].len();
    let height = plots.len();

    let mut visited: Vec<Vec<bool>> = vec![vec![false; width]; height];

    let mut regions: Vec<Region> = Vec::new();

    for y in 0..height {
        for x in 0..width {
            if visited[y][x] {
                continue;
            }
            let c = plots[y][x];

            let mut region = Region {
                plant: c,
                area: 0,
                corners: 0,
            };

            trace_region(&mut region, &plots, &mut visited, c, x, y, width, height);

            regions.push(region);
        }
    }

    // println!("{:#?}", regions);

    let result = regions.iter().map(|r| r.price()).sum::<usize>();

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}

fn is_same(
    plots: &Vec<Vec<char>>,
    pos: (i32, i32),
    other: (i32, i32),
    width: usize,
    height: usize,
) -> bool {
    let (other_x, other_y) = other;
    if other_x < 0 || other_x >= width as i32 || other_y < 0 || other_y >= height as i32 {
        return false;
    }

    let (other_x, other_y) = (other_x as usize, other_y as usize);
    let (x, y) = (pos.0 as usize, pos.1 as usize);

    plots[y][x] == plots[other_y][other_x]
}

fn is_corner(
    plots: &Vec<Vec<char>>,
    pos: (i32, i32),
    delta: (i32, i32),
    width: usize,
    height: usize,
) -> bool {
    let (x, y) = pos;
    let (dx, dy) = delta;

    let mut total = 0;
    let mut direct = 0;

    if is_same(plots, pos, (x + dx, y), width, height) {
        total += 1;
        direct += 1;
    }

    if is_same(plots, pos, (x, y + dy), width, height) {
        total += 1;
        direct += 1;
    }

    if is_same(plots, pos, (x + dx, y + dy), width, height) {
        total += 1;
    }

    match total {
        // No neighbors in that direction, so it's a corner
        0 => true,
        // Two neighbors, it's a corner. Only count it if those are two direct neigbors
        // otherwise we count inner corners 3x
        2 => direct == 2,
        // 3 neighbors in that direction, so it's not a corner
        3 => false,
        // Only one neighbor in that direction. It's a corner if it's not a direct neighbor
        _ => direct == 0,
    }
}

fn trace_region(
    region: &mut Region,
    plots: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
    plant: char,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) {
    visited[y][x] = true;

    region.area += 1;

    // The number of sides == the number of corners, and corners are easier to count

    // Top left corner?
    if is_corner(plots, (x as i32, y as i32), (-1, -1), width, height) {
        region.corners += 1;
    }
    // Bottom left corner?
    if is_corner(plots, (x as i32, y as i32), (-1, 1), width, height) {
        region.corners += 1;
    }
    // Top right corner?
    if is_corner(plots, (x as i32, y as i32), (1, -1), width, height) {
        region.corners += 1;
    }
    // Bottom right corner?
    if is_corner(plots, (x as i32, y as i32), (1, 1), width, height) {
        region.corners += 1;
    }

    // Follow region left
    if x > 0 && !visited[y][x - 1] && plots[y][x - 1] == plant {
        trace_region(region, plots, visited, plant, x - 1, y, width, height);
    }

    // Follow region right
    if x < width - 1 && !visited[y][x + 1] && plots[y][x + 1] == plant {
        trace_region(region, plots, visited, plant, x + 1, y, width, height);
    }

    // Follow region up
    if y > 0 && !visited[y - 1][x] && plots[y - 1][x] == plant {
        trace_region(region, plots, visited, plant, x, y - 1, width, height);
    }

    // Follow region down
    if y < height - 1 && !visited[y + 1][x] && plots[y + 1][x] == plant {
        trace_region(region, plots, visited, plant, x, y + 1, width, height);
    }
}
