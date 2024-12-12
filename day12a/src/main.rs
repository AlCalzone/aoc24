use std::time::Instant;

const INPUT: &'static str = include_str!("input.txt");

#[derive(Debug)]
struct Region {
    area: usize,
    perimeter: usize,
}

impl Region {
    fn price(&self) -> usize {
        self.area * self.perimeter
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
                area: 0,
                perimeter: 0,
            };

            trace_region(&mut region, &plots, &mut visited, c, x, y, width, height);

            regions.push(region);
        }
    }

    println!("{:#?}", regions);

    let result = regions.iter().map(|r| r.price()).sum::<usize>();

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
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
    // Left perimeter
    if x == 0 || plots[y][x - 1] != plant {
        region.perimeter += 1;
    } else if !visited[y][x - 1] {
        trace_region(region, plots, visited, plant, x - 1, y, width, height);
    }

    // Right perimeter
    if x == width - 1 || plots[y][x + 1] != plant {
        region.perimeter += 1;
    } else if !visited[y][x + 1] {
        trace_region(region, plots, visited, plant, x + 1, y, width, height);
    }

    // Top perimeter
    if y == 0 || plots[y - 1][x] != plant {
        region.perimeter += 1;
    } else if !visited[y - 1][x] {
        trace_region(region, plots, visited, plant, x, y - 1, width, height);
    }

    // Bottom perimeter
    if y == height - 1 || plots[y + 1][x] != plant {
        region.perimeter += 1;
    } else if !visited[y + 1][x] {
        trace_region(region, plots, visited, plant, x, y + 1, width, height);
    }
}
