use std::{ops::Sub, time::Instant};

const INPUT: &'static str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Vector {
    x: i32,
    y: i32,
}

impl Sub for Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

trait Key: Clone + Copy {
    fn position(&self) -> Vector;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Direction {
    Left,
    Down,
    Up,
    Right,
}

#[test]
fn test_direction_order() {
    use Direction::*;
    assert!(Left < Down);
    assert!(Down < Up);
    assert!(Up < Right);

    let a = vec![Left, Down];
    let b = vec![Down, Left];
    assert!(a < b);
}

trait Keypad
where
    for<'a> &'a Self::Key: Sub<&'a Self::Key, Output = Vector>,
    Self::Key: Key,
{
    type Key;
    fn start_key() -> Self::Key;

    fn are_moves_safe(start: Self::Key, end: Self::Key, moves: &[Direction]) -> bool;

    /// Determines the move sequence needed to visit the given keys, starting at A
    fn move_sequence(keys: &[Self::Key]) -> Vec<Vec<Direction>> {
        // The robot arm starts pointing at A
        let keys = [&[Self::start_key()], keys].concat();
        keys.windows(2)
            .map(|pair| {
                let start = pair[0];
                let end = pair[1];
                Self::get_cheapest_moves(start, end)
            })
            .collect()
    }

    fn get_cheapest_moves(start: Self::Key, end: Self::Key) -> Vec<Direction> {
        let path = &end - &start;
        let ret = match (path.x, path.y) {
            (0, 0) => vec![],
            (x, 0) => {
                let dir = if x > 0 {
                    Direction::Right
                } else {
                    Direction::Left
                };
                vec![dir; x.abs() as usize]
            }
            (0, y) => {
                let dir = if y > 0 {
                    Direction::Up
                } else {
                    Direction::Down
                };
                vec![dir; y.abs() as usize]
            }
            (x, y) => {
                let x_dir = if x > 0 {
                    Direction::Right
                } else {
                    Direction::Left
                };
                let y_dir = if y > 0 {
                    Direction::Up
                } else {
                    Direction::Down
                };
                let move_x = vec![x_dir; x.abs() as usize];
                let move_y = vec![y_dir; y.abs() as usize];
                let first = [move_x.clone(), move_y.clone()].concat();
                let second = [move_y, move_x].concat();

                match (
                    Self::are_moves_safe(start, end, &first),
                    Self::are_moves_safe(start, end, &second),
                ) {
                    // If both moves are safe, choose the one that yields the cheapest path
                    (true, true) => first.min(second),
                    (true, false) => first,
                    (false, true) => second,
                    (false, false) => panic!("No safe moves found"),
                }
            }
        };

        ret
    }
}

// Numpad, bottom left is (0, 0)
// +---+---+---+
// | 7 | 8 | 9 |
// +---+---+---+
// | 4 | 5 | 6 |
// +---+---+---+
// | 1 | 2 | 3 |
// +---+---+---+
//     | 0 | A |
//     +---+---+

#[derive(Clone, Copy, PartialEq, Eq)]
enum NumpadKey {
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    A,
}

impl Key for NumpadKey {
    fn position(&self) -> Vector {
        match self {
            Self::N0 => Vector { x: 1, y: 0 },
            Self::A => Vector { x: 2, y: 0 },
            Self::N1 => Vector { x: 0, y: 1 },
            Self::N2 => Vector { x: 1, y: 1 },
            Self::N3 => Vector { x: 2, y: 1 },
            Self::N4 => Vector { x: 0, y: 2 },
            Self::N5 => Vector { x: 1, y: 2 },
            Self::N6 => Vector { x: 2, y: 2 },
            Self::N7 => Vector { x: 0, y: 3 },
            Self::N8 => Vector { x: 1, y: 3 },
            Self::N9 => Vector { x: 2, y: 3 },
        }
    }
}

// Orphan rule :(
impl Sub for &NumpadKey {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        self.position() - rhs.position()
    }
}

impl From<char> for NumpadKey {
    fn from(value: char) -> Self {
        match value {
            '0' => NumpadKey::N0,
            '1' => NumpadKey::N1,
            '2' => NumpadKey::N2,
            '3' => NumpadKey::N3,
            '4' => NumpadKey::N4,
            '5' => NumpadKey::N5,
            '6' => NumpadKey::N6,
            '7' => NumpadKey::N7,
            '8' => NumpadKey::N8,
            '9' => NumpadKey::N9,
            'A' => NumpadKey::A,
            _ => panic!("Invalid key"),
        }
    }
}

struct Numpad {}

impl Keypad for Numpad {
    type Key = NumpadKey;

    fn start_key() -> Self::Key {
        NumpadKey::A
    }

    fn are_moves_safe(start: Self::Key, end: Self::Key, moves: &[Direction]) -> bool {
        let Some(&first_move) = moves.first() else {
            return true;
        };
        let Some(&last_move) = moves.last() else {
            return true;
        };

        // On the numpad, moves from and to 0 and A are possibly unsafe.
        match (start, end) {
            (NumpadKey::N0, _) => {
                // Starting at 0, the first move must be up or right
                match first_move {
                    Direction::Up | Direction::Right => true,
                    _ => false,
                }
            }
            (_, NumpadKey::N0) => {
                // Ending at 0, the last move must be down or left
                match last_move {
                    Direction::Down | Direction::Left => true,
                    _ => false,
                }
            }
            (NumpadKey::A, NumpadKey::N1 | NumpadKey::N4 | NumpadKey::N7) => {
                // Going from A to the first column, the first move must be up
                first_move == Direction::Up
            }
            (NumpadKey::N1 | NumpadKey::N4 | NumpadKey::N7, NumpadKey::A) => {
                // Going from the first column to A, the last move must be down
                last_move == Direction::Down
            }
            _ => true,
        }
    }
}

#[test]
fn test_numpad() {
    let keys = [NumpadKey::N1, NumpadKey::N9, NumpadKey::N2, NumpadKey::N1];
    let move_opts = Numpad::move_sequence(&keys);

    use Direction::*;

    assert_eq!(
        move_opts,
        vec![
            vec![Up, Left, Left],
            vec![Up, Up, Right, Right],
            vec![Left, Down, Down],
            vec![Left],
        ]
    );
}

// Dirpad, bottom left is (0, 0)
//     +---+---+
//     | ^ | A |
// +---+---+---+
// | < | v | > |
// +---+---+---+

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DirpadKey {
    Up,
    Left,
    Down,
    Right,
    A,
}

impl Key for DirpadKey {
    fn position(&self) -> Vector {
        match self {
            Self::Up => Vector { x: 1, y: 1 },
            Self::A => Vector { x: 2, y: 1 },
            Self::Left => Vector { x: 0, y: 0 },
            Self::Down => Vector { x: 1, y: 0 },
            Self::Right => Vector { x: 2, y: 0 },
        }
    }
}

// Orphan rule :(
impl Sub for &DirpadKey {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        self.position() - rhs.position()
    }
}

struct Dirpad {}

impl Keypad for Dirpad {
    type Key = DirpadKey;

    fn start_key() -> Self::Key {
        DirpadKey::A
    }

    fn are_moves_safe(start: Self::Key, end: Self::Key, moves: &[Direction]) -> bool {
        let Some(&first_move) = moves.first() else {
            return true;
        };
        let Some(&last_move) = moves.last() else {
            return true;
        };

        // On the dirpad, moves from and to the left key possibly unsafe.
        if start == DirpadKey::Left {
            return first_move == Direction::Right;
        }
        if end == DirpadKey::Left {
            return last_move == Direction::Left;
        }

        true
    }
}

impl Dirpad {
    fn keys_from_moves(moves: &[Direction]) -> Vec<DirpadKey> {
        moves
            .iter()
            .map(|&dir| match dir {
                Direction::Up => DirpadKey::Up,
                Direction::Left => DirpadKey::Left,
                Direction::Down => DirpadKey::Down,
                Direction::Right => DirpadKey::Right,
            })
            .chain(([DirpadKey::A]).into_iter())
            .collect()
    }
}

// Observations:
// - Each move sequence on the dirpad always ends with A
// - Moves have different costs that manifest 2 levels above (or higher?)
//   - < is the most expensive move (needs 1 down and 2 lefts to reach)
//   - v is next (needs 1 left and 1 down)
//   - ^ is next (needs 1 left)
//   - > is the cheapest (needs only one down)
//   - repeated moves are free
// - Because the next-level robot has to confirm each move by going back to A,
//   more expensive moves should be done first and zig-zag moves should be avoided

fn main() {
    let start = Instant::now();
    // const DEPTH: usize = 3;

    let mut result: usize = 0;

    for code in INPUT.trim().lines() {
        let numpad_keys = code.chars().map(NumpadKey::from).collect::<Vec<_>>();

        let numpad_moves = Numpad::move_sequence(&numpad_keys);
        let dirpad_keys: Vec<_> = numpad_moves
            .iter()
            .flat_map(|step| Dirpad::keys_from_moves(&step))
            .collect();

        let dirpad_moves = Dirpad::move_sequence(&dirpad_keys);
        let dirpad2_keys: Vec<_> = dirpad_moves
            .iter()
            .flat_map(|step| Dirpad::keys_from_moves(&step))
            .collect();

        let dirpad2_moves = Dirpad::move_sequence(&dirpad2_keys);
        let dirpad3_keys: Vec<_> = dirpad2_moves
            .iter()
            .flat_map(|step| Dirpad::keys_from_moves(&step))
            .collect();

        let code_num: usize = code.trim_end_matches("A").parse().unwrap();
        result += code_num * dirpad3_keys.len();

        println!("{}, len={}", code, dirpad3_keys.len());
    }

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}
