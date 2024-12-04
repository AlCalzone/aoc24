const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let chars: Vec<Vec<_>> = INPUT.lines().map(|line| line.chars().collect()).collect();
    let mut result = 0;

    let lines = chars.len();
    let cols = chars[0].len();

    // BRUTE FORCE!
    for l in 1..lines - 1 {
        for c in 1..cols - 1 {
            if chars[l][c] != 'A' {
                continue;
            }

            // Top-Bottom
            if chars[l - 1][c - 1] == 'M'
                && chars[l - 1][c + 1] == 'M'
                && chars[l + 1][c - 1] == 'S'
                && chars[l + 1][c + 1] == 'S'
            {
                result += 1;
                continue;
            }

            // Bottom-Top
            if chars[l - 1][c - 1] == 'S'
                && chars[l - 1][c + 1] == 'S'
                && chars[l + 1][c - 1] == 'M'
                && chars[l + 1][c + 1] == 'M'
            {
                result += 1;
                continue;
            }

            // Left-Right
            if chars[l - 1][c - 1] == 'M'
                && chars[l + 1][c - 1] == 'M'
                && chars[l - 1][c + 1] == 'S'
                && chars[l + 1][c + 1] == 'S'
            {
                result += 1;
                continue;
            }

            // Right-Left
            if chars[l - 1][c - 1] == 'S'
                && chars[l + 1][c - 1] == 'S'
                && chars[l - 1][c + 1] == 'M'
                && chars[l + 1][c + 1] == 'M'
            {
                result += 1;
                continue;
            }
        }
    }

    println!("Result: {}", result);
}
