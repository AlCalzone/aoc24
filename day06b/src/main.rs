use std::time::Instant;

const INPUT: &'static str = include_str!("input.txt");

// X, Y
const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (1, 0), (0, 1), (-1, 0)];

const FIELD_SIZE: usize = 130;

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

fn loops(
    obstacles: &Field<FIELD_SIZE, FIELD_SIZE, bool>,
    _obstacle_pos: (i32, i32),
    mut guard_pos: (i32, i32),
    mut guard_dir_index: usize,
) -> bool {
    let mut directions: Field<FIELD_SIZE, FIELD_SIZE, Option<usize>> = Field::new();

    let mut guard_dir = DIRECTIONS[guard_dir_index];

    while (guard_pos.0 >= 0)
        && (guard_pos.0 < FIELD_SIZE as i32)
        && (guard_pos.1 >= 0)
        && (guard_pos.1 < FIELD_SIZE as i32)
    {
        let next_pos = (guard_pos.0 + guard_dir.0, guard_pos.1 + guard_dir.1);

        if next_pos.0 < 0
            || next_pos.0 >= FIELD_SIZE as i32
            || next_pos.1 < 0
            || next_pos.1 >= FIELD_SIZE as i32
        {
            // Guard is out of bounds, simulation terminates
            return false;
        }

        if obstacles.get(next_pos.1, next_pos.0) {
            // turn right
            guard_dir_index = (guard_dir_index + 1) % DIRECTIONS.len();
            guard_dir = DIRECTIONS[guard_dir_index];
        } else {
            // move forward
            if let Some(dir) = directions.get(guard_pos.1, guard_pos.0) {
                if dir == guard_dir_index {
                    // Guard has been here before, facing in the same direction, we have a loop
                    // // Debug output
                    // println!("");
                    // println!("Loop detected:");
                    // for y in 0..FIELD_SIZE as i32 {
                    //     for x in 0..FIELD_SIZE as i32 {
                    //         if (x, y) == obstacle_pos {
                    //             print!("O");
                    //         } else if obstacles.get(y, x) {
                    //             print!("#");
                    //         } else {
                    //             match directions.get(y, x) {
                    //                 Some(0) => print!("^"),
                    //                 Some(1) => print!(">"),
                    //                 Some(2) => print!("v"),
                    //                 Some(3) => print!("<"),
                    //                 None => print!("."),
                    //                 _ => unreachable!(),
                    //             }
                    //         }
                    //     }
                    //     println!("");
                    // }
                    return true;
                }
            }
            directions.set(guard_pos.1, guard_pos.0, Some(guard_dir_index));

            guard_pos = next_pos;
        }
    }

    false
}

fn main() {
    let start = Instant::now();

    let mut obstacles: Field<FIELD_SIZE, FIELD_SIZE, bool> = Field::new();

    let mut guard_pos: (i32, i32) = (0, 0);
    let mut guard_dir_index: usize = 0;
    let mut guard_dir: (i32, i32);

    // parse
    for (row, line) in INPUT.lines().enumerate() {
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
    let start_pos = guard_pos;
    let start_dir_index = guard_dir_index;

    // simulate
    let mut directions: Field<FIELD_SIZE, FIELD_SIZE, Option<usize>> = Field::new();
    let mut new_obstacles: Field<FIELD_SIZE, FIELD_SIZE, bool> = Field::new();
    let mut result: u32 = 0;

    while (guard_pos.0 >= 0)
        && (guard_pos.0 < FIELD_SIZE as i32)
        && (guard_pos.1 >= 0)
        && (guard_pos.1 < FIELD_SIZE as i32)
    {
        let next_pos = (guard_pos.0 + guard_dir.0, guard_pos.1 + guard_dir.1);

        if next_pos.0 < 0
            || next_pos.0 >= FIELD_SIZE as i32
            || next_pos.1 < 0
            || next_pos.1 >= FIELD_SIZE as i32
        {
            // Guard is going out of bounds, simulation terminates
            break;
        }

        if obstacles.get(next_pos.1, next_pos.0) {
            // turn right
            guard_dir_index = (guard_dir_index + 1) % DIRECTIONS.len();
            guard_dir = DIRECTIONS[guard_dir_index];
        } else {
            // move forward and remember the direction the guard was facing in
            directions.set(guard_pos.1, guard_pos.0, Some(guard_dir_index));
            guard_pos = next_pos;

            // Determine if there would be a loop if an obstacle would be placed at the next position
            if next_pos != start_pos && !new_obstacles.get(next_pos.1, next_pos.0) {
                let mut obstacles = obstacles.clone();
                obstacles.set(next_pos.1, next_pos.0, true);

                if loops(&obstacles, next_pos, start_pos, start_dir_index) {
                    new_obstacles.set(next_pos.1, next_pos.0, true);
                    result += 1;
                }
            }
        }
    }

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}
