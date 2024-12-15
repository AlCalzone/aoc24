use std::{fmt::Debug, time::Instant};

const INPUT: &'static str = include_str!("input.txt");

#[derive(PartialEq, Eq, Clone, Copy)]
enum Tile {
    Robot,
    Empty,
    Wall,
    // Two halves of a box
    BoxL,
    BoxR,
}

impl From<char> for Tile {
    fn from(c: char) -> Self {
        match c {
            '@' => Tile::Robot,
            '.' => Tile::Empty,
            '#' => Tile::Wall,
            '[' => Tile::BoxL,
            ']' => Tile::BoxR,
            _ => panic!("Invalid tile: {}", c),
        }
    }
}

impl Debug for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Tile::Robot => "🯆",
            Tile::Empty => " ",
            Tile::Wall => "🧱",
            Tile::BoxL => "🎁",
            Tile::BoxR => "",
        };
        write!(f, "{}", c)
    }
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Horizontal(DirectionH),
    Vertical(DirectionV),
}

#[derive(Debug, PartialEq, Eq)]
enum DirectionH {
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum DirectionV {
    Up,
    Down,
}

impl Direction {
    fn from_char(c: char) -> Option<Direction> {
        match c {
            '^' => Some(Direction::Vertical(DirectionV::Up)),
            'v' => Some(Direction::Vertical(DirectionV::Down)),
            '<' => Some(Direction::Horizontal(DirectionH::Left)),
            '>' => Some(Direction::Horizontal(DirectionH::Right)),
            _ => None,
        }
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    robot_pos: (usize, usize),
    width: usize,
    height: usize,
}

impl Map {
    fn parse(input: &str) -> Map {
        let tiles: Vec<Vec<_>> = input
            .lines()
            .map(|line| line.chars().map(Tile::from).collect())
            .collect();

        let height = tiles.len();
        let width = tiles[0].len();

        let robot_pos = tiles
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, &tile)| {
                    if tile == Tile::Robot {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        Map {
            tiles,
            robot_pos,
            width,
            height,
        }
    }

    fn get(&self, x: usize, y: usize) -> Tile {
        self.tiles[y][x]
    }

    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        self.tiles[y][x] = tile;
    }

    fn swap(&mut self, x1: usize, y1: usize, x2: usize, y2: usize) {
        let tmp = self.get(x1, y1);
        self.set(x1, y1, self.get(x2, y2));
        self.set(x2, y2, tmp);
    }

    fn try_move_robot(&mut self, dir: Direction) {
        let (x, y) = self.robot_pos;
        if let Some((x, y)) = self.try_move(x, y, dir) {
            self.robot_pos = (x, y);
        }
    }

    fn try_move(&mut self, x: usize, y: usize, dir: Direction) -> Option<(usize, usize)> {
        match dir {
            Direction::Horizontal(dir) => self.try_move_horizontal(x, y, dir),
            Direction::Vertical(dir) => self.try_move_vertical(x, y, dir),
        }
    }

    fn try_move_horizontal(
        &mut self,
        x: usize,
        y: usize,
        dir: DirectionH,
    ) -> Option<(usize, usize)> {
        if x == 0 && dir == DirectionH::Left {
            return None;
        }
        if x == self.width - 1 && dir == DirectionH::Right {
            return None;
        }
        let (x1, y1) = match dir {
            DirectionH::Left => (x - 1, y),
            DirectionH::Right => (x + 1, y),
        };

        let next_tile = self.get(x1, y1);
        match next_tile {
            Tile::Robot => unreachable!(),
            Tile::Empty => {
                self.swap(x, y, x1, y1);
                Some((x1, y1))
            }
            Tile::Wall => None,
            Tile::BoxL | Tile::BoxR => {
                // Try moving the next box
                if self.try_move_horizontal(x1, y1, dir).is_some() {
                    self.swap(x, y, x1, y1);
                    Some((x1, y1))
                } else {
                    None
                }
            }
        }
    }

    fn try_move_vertical(&mut self, x: usize, y: usize, dir: DirectionV) -> Option<(usize, usize)> {
        if !self.can_move_vertical(x, y, dir) {
            None
        } else {
            Some(self.move_vertical(x, y, dir))
        }
    }

    fn can_move_vertical(&mut self, x: usize, y: usize, dir: DirectionV) -> bool {
        if y == 0 && dir == DirectionV::Up {
            return false;
        }
        if y == self.height - 1 && dir == DirectionV::Down {
            return false;
        }

        let (x1, y1) = match dir {
            DirectionV::Up => (x, y - 1),
            DirectionV::Down => (x, y + 1),
        };
        let next_tile = self.get(x1, y1);

        match next_tile {
            Tile::Robot => unreachable!(),
            Tile::Empty => true,
            Tile::Wall => false,
            // When moving a box, we need to check how far the box can be moved
            Tile::BoxL => {
                self.can_move_vertical(x1, y1, dir) &&
                    // Check the right half too
                    self.can_move_vertical(x1+1, y1, dir)
            }
            Tile::BoxR => {
                self.can_move_vertical(x1, y1, dir) &&
                    // Check the left half too
                    self.can_move_vertical(x1-1, y1, dir)
            }
        }
    }

    fn move_vertical(&mut self, x: usize, y: usize, dir: DirectionV) -> (usize, usize) {
        let (x1, y1) = match dir {
            DirectionV::Up => (x, y - 1),
            DirectionV::Down => (x, y + 1),
        };

        let next_tile = self.get(x1, y1);
        match next_tile {
            Tile::Empty => {
                self.swap(x, y, x1, y1);
            }
            // When pushing a box, push both halves
            Tile::BoxL => {
                self.move_vertical(x1, y1, dir);
                self.move_vertical(x1 + 1, y1, dir);
                self.swap(x, y, x1, y1);
            }
            Tile::BoxR => {
                self.move_vertical(x1, y1, dir);
                self.move_vertical(x1 - 1, y1, dir);
                self.swap(x, y, x1, y1);
            }
            // We have checked that we can move
            _ => unreachable!(),
        }

        (x1, y1)
    }

    fn gps(&self) -> Vec<usize> {
        let box_coordinates = self
            .tiles
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, &tile)| {
                        if tile == Tile::BoxL {
                            Some((x, y))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .flatten();
        box_coordinates.map(|(x, y)| y * 100 + x).collect()
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut str: String = String::with_capacity(self.width * self.height);
        for y in 0..self.height {
            for x in 0..self.width {
                let tile = self.get(x, y);
                if tile != Tile::Wall || x % 2 == 0 {
                    str.push_str(format!("{:?}", tile).as_str());
                }
            }
            str.push('\n');
        }
        writeln!(f, "{str}")?;

        Ok(())
    }
}

fn main() {
    let start = Instant::now();
    let (map_str, moves_str) = INPUT.split_once("\n\n").unwrap();

    let warehouse2 = map_str
        .chars()
        .map(|c| match c {
            '#' => "##".to_owned(),
            'O' => "[]".to_owned(),
            '.' => "..".to_owned(),
            '@' => "@.".to_owned(),
            c => c.to_string(),
        })
        .collect::<String>();

    let mut map = Map::parse(&warehouse2);
    let directions: Vec<Direction> = moves_str.chars().filter_map(Direction::from_char).collect();

    for dir in directions {
        map.try_move_robot(dir);
    }

    let result = map.gps().iter().sum::<usize>();

    let elapsed = start.elapsed();

    println!("{:?}", map);

    println!("");
    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}
