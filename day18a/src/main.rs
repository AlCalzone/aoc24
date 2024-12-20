use std::{collections::BinaryHeap, time::Instant};

const INPUT: &'static str = include_str!("input.txt");
const SIZE: usize = 71;
const NUM_BYTES: usize = 1024;

#[derive(Clone, Copy)]
struct Tile {
    wall: bool,
    cost: Option<usize>,
}

#[derive(Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Point {
    x: usize,
    y: usize,
}

impl From<(usize, usize)> for Point {
    fn from((x, y): (usize, usize)) -> Self {
        Point { x, y }
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

struct Map {
    tiles: Vec<Vec<Tile>>,
    start_pos: Point,
    end_pos: Point,
}

impl Map {
    fn get(&self, pos: Point) -> &Tile {
        let Point { x, y } = pos;
        self.tiles.get(y).unwrap().get(x).unwrap()
    }

    fn get_mut(&mut self, pos: Point) -> &mut Tile {
        let Point { x, y } = pos;
        self.tiles.get_mut(y).unwrap().get_mut(x).unwrap()
    }
}

fn main() {
    let start = Instant::now();

    let coordinates = INPUT.lines().map(|l| {
        let parts = l.split_once(",").unwrap();
        (parts.0.parse().unwrap(), parts.1.parse().unwrap())
    });

    let mut map = Map {
        tiles: vec![
            vec![
                Tile {
                    wall: false,
                    cost: None
                };
                SIZE
            ];
            SIZE
        ],
        start_pos: Point { x: 0, y: 0 },
        end_pos: Point {
            x: SIZE - 1,
            y: SIZE - 1,
        },
    };

    for coord in coordinates.take(NUM_BYTES) {
        map.get_mut(coord.into()).wall = true;
    }

    let mut todos: BinaryHeap<Node> = BinaryHeap::new();
    todos.push(Node {
        pos: map.start_pos,
        cost: 0,
    });
    map.get_mut(map.start_pos).cost = Some(0);

    let mut result: usize = usize::MAX;

    while let Some(todo) = todos.pop() {
        let pos = todo.pos;
        if pos == map.end_pos {
            result = map.get(pos).cost.unwrap();
            break;
        }
        let cur_cost = map.get(pos).cost.unwrap();
        let next_cost = cur_cost + 1;
        if pos.x < SIZE - 1 {
            // Right
            let next_pos = Point {
                x: pos.x + 1,
                y: pos.y,
            };
            if let Some(node) = check_pos(&mut map, next_pos, next_cost) {
                todos.push(node);
            }
        }
        if pos.y < SIZE - 1 {
            // Bottom
            let next_pos = Point {
                x: pos.x,
                y: pos.y + 1,
            };
            if let Some(node) = check_pos(&mut map, next_pos, next_cost) {
                todos.push(node);
            }
        }
        if pos.x > 0 {
            // Left
            let next_pos = Point {
                x: pos.x - 1,
                y: pos.y,
            };
            if let Some(node) = check_pos(&mut map, next_pos, next_cost) {
                todos.push(node);
            }
        }
        if pos.y > 0 {
            // Top
            let next_pos = Point {
                x: pos.x,
                y: pos.y - 1,
            };
            if let Some(node) = check_pos(&mut map, next_pos, next_cost) {
                todos.push(node);
            }
        }
    }

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}

fn check_pos(map: &mut Map, next_pos: Point, next_cost: usize) -> Option<Node> {
    let next_tile = map.get_mut(next_pos);
    if !next_tile.wall && (next_tile.cost.is_none() || next_tile.cost > Some(next_cost)) {
        next_tile.cost = Some(next_cost);
        Some(Node {
            pos: next_pos,
            cost: next_cost,
        })
    } else {
        None
    }
}
