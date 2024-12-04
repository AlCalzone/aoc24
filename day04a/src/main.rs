const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let chars: Vec<Vec<_>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let mut result = 0;

    let directions: Vec<(i32, i32)> = vec![
        (0, 1),   // RIGHT
        (1, 0),   // DOWN
        (0, -1),  // LEFT
        (-1, 0),  // UP
        (1, 1),   // DOWN RIGHT
        (1, -1),  // DOWN LEFT
        (-1, 1),  // UP RIGHT
        (-1, -1), // UP LEFT
    ];

    let search = vec!['X', 'M', 'A', 'S'];

    let lines = chars.len();
    let cols = chars[0].len();

    // BRUTE FORCE!
    for l in 0..lines {
        for c in 0..cols {
            if chars[l][c] != 'X' {
                continue;
            }

            'search: for dir in directions.iter() {
                for i in 1..search.len() {
                    let (y, x) = (
                        (l as i32) + dir.0 * (i as i32),
                        (c as i32) + dir.1 * (i as i32),
                    );
                    if (x < 0 || y < 0 || x >= cols as i32 || y >= lines as i32)
                        || chars[y as usize][x as usize] != search[i]
                    {
                        continue 'search;
                    }
                }
                result += 1;
            }
        }
    }

    println!("Result: {}", result);
}
