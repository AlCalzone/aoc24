use std::{
    cmp::Reverse,
    collections::{BTreeMap, BTreeSet, BinaryHeap},
    fmt::Debug,
    ops::Add,
    path,
    time::Instant,
    usize,
};

const INPUT: &'static str = include_str!("input.txt");

#[derive(PartialEq, Eq, Clone, Copy)]
enum TileKind {
    Start,
    Empty,
    Wall,
    End,
}

impl From<char> for TileKind {
    fn from(value: char) -> Self {
        match value {
            'S' => TileKind::Start,
            '.' => TileKind::Empty,
            '#' => TileKind::Wall,
            'E' => TileKind::End,
            _ => panic!("Invalid tile: {}", value),
        }
    }
}

struct Tile {
    kind: TileKind,
    cost: usize,
    dir: Option<Direction>,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl Add<Direction> for Point {
    type Output = Point;

    fn add(self, rhs: Direction) -> Self::Output {
        match rhs {
            Direction::North => Point {
                x: self.x,
                y: self.y - 1,
            },
            Direction::East => Point {
                x: self.x + 1,
                y: self.y,
            },
            Direction::South => Point {
                x: self.x,
                y: self.y + 1,
            },
            Direction::West => Point {
                x: self.x - 1,
                y: self.y,
            },
        }
    }
}

struct Map {
    tiles: Vec<Vec<Tile>>,
    start_pos: Point,
    end_pos: Point,
    // width: usize,
    // height: usize,
}

impl Map {
    fn parse(input: &str) -> Map {
        let mut tiles = Vec::new();
        let mut start_pos: Point = Default::default();
        let mut end_pos: Point = Default::default();

        for (y, line) in input.lines().enumerate() {
            let mut row = Vec::new();
            for (x, c) in line.chars().enumerate() {
                let kind = TileKind::from(c);

                if kind == TileKind::Start {
                    start_pos = Point { x, y };
                    row.push(Tile {
                        kind,
                        dir: Some(Direction::East),
                        cost: 0,
                    });
                } else {
                    if kind == TileKind::End {
                        end_pos = Point { x, y };
                    }
                    row.push(Tile {
                        kind,
                        dir: None,
                        cost: usize::MAX,
                    });
                }
            }
            tiles.push(row);
        }

        // let width = tiles[0].len();
        // let height = tiles.len();

        Map {
            tiles,
            start_pos,
            end_pos,
            // width,
            // height,
        }
    }

    fn get(&self, pos: Point) -> &Tile {
        let Point { x, y } = pos;
        self.tiles.get(y).unwrap().get(x).unwrap()
    }

    fn get_mut(&mut self, pos: Point) -> &mut Tile {
        let Point { x, y } = pos;
        self.tiles.get_mut(y).unwrap().get_mut(x).unwrap()
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.tiles.iter() {
            for tile in row.iter() {
                match tile.kind {
                    TileKind::Start => write!(f, "S")?,
                    TileKind::Empty => match tile.dir {
                        Some(Direction::North) => write!(f, "^")?,
                        Some(Direction::East) => write!(f, ">")?,
                        Some(Direction::South) => write!(f, "v")?,
                        Some(Direction::West) => write!(f, "<")?,
                        None => write!(f, ".")?,
                    },
                    TileKind::Wall => write!(f, "#")?,
                    TileKind::End => write!(f, "E")?,
                }
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn rotate_cw(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::East => Direction::South,
            Direction::South => Direction::West,
            Direction::West => Direction::North,
        }
    }

    fn rotate_ccw(&self) -> Direction {
        match self {
            Direction::North => Direction::West,
            Direction::East => Direction::North,
            Direction::South => Direction::East,
            Direction::West => Direction::South,
        }
    }
}

struct Node {
    pos: Point,
    cost: usize,
    dir: Direction,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl Eq for Node {}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reverse ordering because we want the smallest cost first
        self.cost.cmp(&other.cost).reverse()
    }
}

fn main() {
    let start = Instant::now();

    let mut map = Map::parse(INPUT);
    let mut todos: BinaryHeap<Node> = BinaryHeap::new();
    todos.push(Node {
        pos: map.start_pos,
        cost: 0,
        dir: Direction::East,
    });

    let mut result: usize = usize::MAX;

    while let Some(todo) = todos.pop() {
        let pos = todo.pos;

        {
            let forward = todo.dir;
            let forward_pos = pos + forward;
            let forward_tile = map.get_mut(forward_pos);
            let forward_cost = todo.cost + 1;

            if forward_tile.kind != TileKind::Wall {
                if forward_tile.cost > forward_cost {
                    forward_tile.cost = forward_cost;
                    forward_tile.dir = Some(forward);

                    if forward_tile.kind == TileKind::End {
                        result = result.min(forward_cost);
                    } else {
                        todos.push(Node {
                            pos: forward_pos,
                            cost: forward_tile.cost,
                            dir: forward,
                        });
                    }
                }
            }
        }

        {
            let left = todo.dir.rotate_ccw();
            let left_pos = pos + left;
            let left_tile = map.get_mut(left_pos);
            let left_cost = todo.cost + 1001;

            if left_tile.kind != TileKind::Wall {
                if left_tile.cost > left_cost {
                    left_tile.cost = left_cost;
                    left_tile.dir = Some(left);

                    if left_tile.kind == TileKind::End {
                        result = result.min(left_cost);
                    } else {
                        todos.push(Node {
                            pos: left_pos,
                            cost: left_tile.cost,
                            dir: left,
                        });
                    }
                }
            }
        }

        {
            let right = todo.dir.rotate_cw();
            let right_pos = pos + right;
            let right_tile = map.get_mut(right_pos);
            let right_cost = todo.cost + 1001;

            if right_tile.kind != TileKind::Wall {
                if right_tile.cost > right_cost {
                    right_tile.cost = right_cost;
                    right_tile.dir = Some(right);

                    if right_tile.kind == TileKind::End {
                        result = result.min(right_cost);
                    } else {
                        todos.push(Node {
                            pos: right_pos,
                            cost: right_tile.cost,
                            dir: right,
                        });
                    }
                }
            }
        }
    }

    let elapsed = start.elapsed();
    println!("Result part 1: {}", result);
    println!("(took: {:?})", elapsed);

    let start = Instant::now();

    // Trace backwards through the map to find the path with the best cost
    let end_pos = map.end_pos;
    let end_dir = map.get(end_pos).dir.unwrap();
    let mut paths: BTreeSet<Point> = BTreeSet::new();
    trace_path(
        &mut map,
        &mut paths,
        end_pos,
        end_dir.rotate_ccw().rotate_ccw(),
    );

    let elapsed = start.elapsed();

    println!("Result part 2: {}", paths.len());
    println!("(took: {:?})", elapsed);

    println!();

    for (y, row) in map.tiles.iter().enumerate() {
        for (x, tile) in row.iter().enumerate() {
            match tile.kind {
                TileKind::Start => print!("S"),
                TileKind::Empty => {
                    if paths.contains(&Point { x, y }) {
                        print!("O")
                    } else {
                        print!(".")
                    }
                }
                TileKind::Wall => print!("▒"),
                TileKind::End => print!("E"),
            }
        }
        println!("");
    }
}

fn trace_path(map: &mut Map, paths: &mut BTreeSet<Point>, pos: Point, dir: Direction) {
    paths.insert(pos);

    if map.get(pos).kind == TileKind::Start {
        return;
    }

    let cur_cost = map.get(pos).cost;

    {
        let forward = dir;
        let forward_pos = pos + forward;
        if paths.contains(&forward_pos) {
            return;
        }
        let forward_tile = map.get_mut(forward_pos);
        if forward_tile.cost == cur_cost - 1 {
            trace_path(map, paths, forward_pos, forward);
        } else if forward_tile.cost == cur_cost - 1001 {
            // We arrived here by turning left or right
            trace_path(map, paths, forward_pos, forward.rotate_ccw());
            trace_path(map, paths, forward_pos, forward.rotate_cw());
            // But also consider going forward if we crossed a path
            let forward_twice = forward_pos + forward;
            let forward_twice_tile = map.get_mut(forward_twice);
            if forward_twice_tile.cost == cur_cost - 2 {
                trace_path(map, paths, forward_twice, forward);
            }
        }
    }

    {
        let left = dir.rotate_ccw();
        let left_pos = pos + left;
        let left_tile = map.get_mut(left_pos);
        if left_tile.cost == cur_cost - 1001 {
            trace_path(map, paths, left_pos, left);
        }
    }

    {
        let right = dir.rotate_cw();
        let right_pos = pos + right;
        let right_tile = map.get_mut(right_pos);
        if right_tile.cost == cur_cost - 1001 {
            trace_path(map, paths, right_pos, right);
        }
    }
}
