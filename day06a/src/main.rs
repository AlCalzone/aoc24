use std::time::Instant;

const INPUT: &'static str = include_str!("input.txt");

// X, Y
const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

#[derive(Clone)]
struct Field<const R: usize, const C: usize> {
    data: [[bool; C]; R],
}

impl<const R: usize, const C: usize> Field<R, C> {
    pub fn new() -> Self {
        Field {
            data: [[false; C]; R],
        }
    }

    pub fn set(&mut self, row: i32, col: i32, value: bool) {
        self.data[row as usize][col as usize] = value;
    }

    pub fn get(&self, row: i32, col: i32) -> bool {
        self.data[row as usize][col as usize]
    }
}

fn main() {
    let start = Instant::now();

    let mut num_lines: i32 = 0;
    let mut num_cols: i32 = 0;

    let mut obstacles: Field<130, 130> = Field::new();

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
                obstacles.set(row as i32, col as i32, true);
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
    let mut visited: Field<130, 130> = Field::new();
    let mut result: u32 = 0;

    while (guard_pos.0 >= 0)
        && (guard_pos.0 < num_cols)
        && (guard_pos.1 >= 0)
        && (guard_pos.1 < num_lines)
    {
        let next_pos = (guard_pos.0 + guard_dir.0, guard_pos.1 + guard_dir.1);

        if next_pos.0 < 0 || next_pos.0 >= num_cols || next_pos.1 < 0 || next_pos.1 >= num_lines {
            result += 1;
            break;
        }

        if obstacles.get(next_pos.1, next_pos.0) {
            // turn right
            guard_dir_index = (guard_dir_index + 1) % DIRECTIONS.len();
            guard_dir = DIRECTIONS[guard_dir_index];
        } else {
            // move forward
            if !visited.get(guard_pos.1, guard_pos.0) {
                result += 1;
            }
            visited.set(guard_pos.1, guard_pos.0, true);
            guard_pos = next_pos;
        }
    }

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}
