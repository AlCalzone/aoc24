#![feature(let_chains)]

use std::{
    collections::{BTreeMap, BinaryHeap},
    fmt::{Debug, Display},
    time::Instant,
};

const INPUT: &'static str = include_str!("input.txt");
const CHEAT_DURATION: usize = 20;

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

impl From<TileKind> for &str {
    fn from(value: TileKind) -> Self {
        match value {
            TileKind::Start => "S",
            TileKind::Empty => ".",
            TileKind::Wall => "#",
            TileKind::End => "E",
        }
    }
}

impl Display for TileKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Into::<&str>::into(*self))
    }
}

#[derive(Clone, Copy)]
struct Tile {
    kind: TileKind,
    cost: Option<usize>,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Point {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point { x, y }
    }
}

impl Point {
    fn above(&self) -> Point {
        Point {
            x: self.x,
            y: self.y - 1,
        }
    }

    fn below(&self) -> Point {
        Point {
            x: self.x,
            y: self.y + 1,
        }
    }

    fn left(&self) -> Point {
        Point {
            x: self.x - 1,
            y: self.y,
        }
    }

    fn right(&self) -> Point {
        Point {
            x: self.x + 1,
            y: self.y,
        }
    }

    fn neighbors(&self) -> [Point; 4] {
        [self.above(), self.below(), self.left(), self.right()]
    }
}

struct Node {
    pos: Point,
    cost: usize,
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

#[derive(Clone)]
struct Map {
    tiles: Vec<Vec<Tile>>,
    start_pos: Point,
    end_pos: Point,
    width: usize,
    height: usize,
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
                        cost: Some(0),
                    });
                } else {
                    if kind == TileKind::End {
                        end_pos = Point { x, y };
                    }
                    row.push(Tile { kind, cost: None });
                }
            }
            tiles.push(row);
        }

        let width = tiles[0].len();
        let height = tiles.len();

        Map {
            tiles,
            start_pos,
            end_pos,
            width,
            height,
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

    fn solve(&mut self) -> Vec<Point> {
        let mut todos: BinaryHeap<Node> = BinaryHeap::new();
        todos.push(Node {
            pos: self.start_pos,
            cost: self.get(self.start_pos).cost.unwrap(),
        });

        while let Some(todo) = todos.pop() {
            let pos = todo.pos;
            let cur_tile = self.get(pos);
            let cur_cost = cur_tile.cost.unwrap();
            if cur_tile.kind == TileKind::End {
                return self.trace_solution();
            }

            let next_cost = cur_cost + 1;
            if pos.x < self.width - 1 {
                // Right
                let next_pos = Point {
                    x: pos.x + 1,
                    y: pos.y,
                };
                if let Some(node) = check_pos(self, next_pos, next_cost) {
                    todos.push(node);
                }
            }
            if pos.y < self.height - 1 {
                // Bottom
                let next_pos = Point {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if let Some(node) = check_pos(self, next_pos, next_cost) {
                    todos.push(node);
                }
            }
            if pos.x > 0 {
                // Left
                let next_pos = Point {
                    x: pos.x - 1,
                    y: pos.y,
                };
                if let Some(node) = check_pos(self, next_pos, next_cost) {
                    todos.push(node);
                }
            }
            if pos.y > 0 {
                // Top
                let next_pos = Point {
                    x: pos.x,
                    y: pos.y - 1,
                };
                if let Some(node) = check_pos(self, next_pos, next_cost) {
                    todos.push(node);
                }
            }
        }

        panic!("No solution found");
    }

    fn trace_solution(&self) -> Vec<Point> {
        let mut result = Vec::new();
        let mut pos = self.end_pos;
        let mut cost = self.get(pos).cost.unwrap();

        while pos != self.start_pos {
            result.push(pos);
            let next_cost = cost - 1;
            let next_pos = pos
                .neighbors()
                .iter()
                .find_map(|&neighbor| {
                    let neighbor_tile = self.get(neighbor);
                    if neighbor_tile.cost == Some(next_cost) {
                        Some(neighbor)
                    } else {
                        None
                    }
                })
                .expect("the map to be solved");

            pos = next_pos;
            cost = next_cost;
        }

        result.push(self.start_pos);
        result.reverse();
        result
    }
}

impl Debug for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in self.tiles.iter() {
            for tile in row.iter() {
                write!(f, "{}", &tile.kind)?;
            }
            writeln!(f, "")?;
        }

        Ok(())
    }
}

fn check_pos(map: &mut Map, next_pos: Point, next_cost: usize) -> Option<Node> {
    let next_tile = map.get_mut(next_pos);
    if next_tile.kind != TileKind::Wall
        && (next_tile.cost.is_none() || next_tile.cost > Some(next_cost))
    {
        next_tile.cost = Some(next_cost);
        Some(Node {
            pos: next_pos,
            cost: next_cost,
        })
    } else {
        None
    }
}

fn main() {
    let start = Instant::now();

    let mut map = Map::parse(INPUT);
    // Determine the normal path through the racetrack
    let original_path = map.solve();

    // How much time each cheat saves
    let mut saves: BTreeMap<usize, usize> = BTreeMap::new();

    // Along the normal path, check if there is another point we can reach quicker by cheating
    for point in original_path.iter() {
        let tile = map.get(*point);
        if tile.kind == TileKind::End {
            break;
        }

        find_saves_at_pos(&map, &mut saves, *point);
    }

    let result: usize = saves
        .iter()
        .filter_map(|(&save, count)| if save >= 100 { Some(count) } else { None })
        .sum();

    let elapsed = start.elapsed();

    // println!("Un-cheated time: {} picoseconds", original_path.len());
    // for (save, count) in saves {
    //     println!("{} cheats save {} picoseconds", count, save);
    // }

    println!("Cheats >= 100 picoseconds: {}", result);
    println!("(took: {:?})", elapsed);
}

fn compute_save_at_pos(
    map: &Map,
    saves: &mut BTreeMap<usize, usize>,
    cur_cost: usize,
    pos: Point,
    cheat_duration: usize,
) -> Option<usize> {
    let cost = map.get(pos).cost;
    if let Some(cost) = cost
        && cost > cur_cost + cheat_duration
    {
        let save = cost - (cur_cost + cheat_duration);
        let count = saves.entry(save).or_insert(0);
        *count += 1;
        Some(save)
    } else {
        None
    }
}

fn find_saves_at_pos(map: &Map, saves: &mut BTreeMap<usize, usize>, pos: Point) {
    let cur_cost = map.get(pos).cost.unwrap();

    for duration in 2..=CHEAT_DURATION {
        for dx in 0..=duration {
            let dy = duration - dx;
            if dx <= pos.x && dy <= pos.y && dy > 0 && dx > 0 {
                // we can go left/up
                let point = Point {
                    x: pos.x - dx,
                    y: pos.y - dy,
                };
                compute_save_at_pos(map, saves, cur_cost, point, duration);
            }
            if dx <= pos.x && dy < map.height - pos.y && dx > 0 {
                // we can go left/down
                let point = Point {
                    x: pos.x - dx,
                    y: pos.y + dy,
                };
                compute_save_at_pos(map, saves, cur_cost, point, duration);
            }
            if dx < map.width - pos.x && dy <= pos.y && dy > 0 {
                // we can go right/up
                let point = Point {
                    x: pos.x + dx,
                    y: pos.y - dy,
                };
                compute_save_at_pos(map, saves, cur_cost, point, duration);
            }
            if dx < map.width - pos.x && dy < map.height - pos.y {
                // we can go right/down
                let point = Point {
                    x: pos.x + dx,
                    y: pos.y + dy,
                };
                compute_save_at_pos(map, saves, cur_cost, point, duration);
            }
        }
    }
}
