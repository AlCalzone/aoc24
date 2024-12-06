use std::{collections::BTreeSet, time::Instant};

const INPUT: &'static str = include_str!("input.txt");

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

fn main() {
    let start = Instant::now();

    let mut num_lines: i32 = 0;
    let mut num_cols: i32 = 0;
    let mut obstacles: Vec<(i32, i32)> = Vec::new();

    let mut guard_pos: (i32, i32) = (0, 0);
    let mut guard_dir_index: usize = 0;
    let mut guard_dir: (i32, i32);

    // parse
    for (row, line) in INPUT.lines().enumerate() {
        num_lines += 1;
        if num_cols == 0 {
            num_cols = line.len() as i32;
        }

        for (col, char) in line.chars().enumerate() {
            if char == '#' {
                obstacles.push((col as i32, row as i32));
            } else if char == '^' {
                guard_pos = (col as i32, row as i32);
                guard_dir_index = 0;
            } else if char == 'v' {
                guard_pos = (col as i32, row as i32);
                guard_dir_index = 2;
            } else if char == '<' {
                guard_pos = (col as i32, row as i32);
                guard_dir_index = 3;
            } else if char == '>' {
                guard_pos = (col as i32, row as i32);
                guard_dir_index = 1;
            }
        }
    }

    guard_dir = DIRECTIONS[guard_dir_index];

    // simulate
    let mut visited: BTreeSet<(i32, i32)> = BTreeSet::new();
    while (guard_pos.0 >= 0)
        && (guard_pos.0 < num_cols)
        && (guard_pos.1 >= 0)
        && (guard_pos.1 < num_lines)
    {
        let next_pos = (guard_pos.0 + guard_dir.0, guard_pos.1 + guard_dir.1);

        if obstacles.contains(&(next_pos.0, next_pos.1)) {
            // turn right
            guard_dir_index = (guard_dir_index + 1) % DIRECTIONS.len();
            guard_dir = DIRECTIONS[guard_dir_index];
        } else {
            // move forward
            visited.insert(guard_pos);
            guard_pos = next_pos;
        }
    }

    let result = visited.len();

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}
