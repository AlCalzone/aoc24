use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::map,
    multi::many0,
    sequence::tuple,
    IResult,
};
use std::time::Instant;
const INPUT: &'static str = include_str!("input.txt");

#[derive(Debug)]
struct Equation {
    dx_a: u64,
    dy_a: u64,
    dx_b: u64,
    dy_b: u64,
    x: u64,
    y: u64,
}

fn parse_eq(input: &str) -> IResult<&str, Equation> {
    let (input, _) = many0(tag("\n"))(input)?;
    let (input, (_, dx_a, _, dy_a, _)) = tuple((
        tag("Button A: X+"),
        map(digit1, |s: &str| s.parse::<u64>().unwrap()),
        tag(", Y+"),
        map(digit1, |s: &str| s.parse::<u64>().unwrap()),
        tag("\n"),
    ))(input)?;
    let (input, (_, dx_b, _, dy_b, _)) = tuple((
        tag("Button B: X+"),
        map(digit1, |s: &str| s.parse::<u64>().unwrap()),
        tag(", Y+"),
        map(digit1, |s: &str| s.parse::<u64>().unwrap()),
        tag("\n"),
    ))(input)?;
    let (input, (_, x, _, y, _)) = tuple((
        tag("Prize: X="),
        map(digit1, |s: &str| s.parse::<u64>().unwrap()),
        tag(", Y="),
        map(digit1, |s: &str| s.parse::<u64>().unwrap()),
        tag("\n"),
    ))(input)?;

    Ok((
        input,
        Equation {
            dx_a,
            dy_a,
            dx_b,
            dy_b,
            x: x + 10000000000000,
            y: y + 10000000000000,
        },
    ))
}

impl Equation {
    fn solve(&self) -> Option<(u64, u64)> {
        let n0 = self.dy_b * self.x;
        let n1 = self.dx_b * self.y;

        let m0 = self.dx_a * self.y;
        let m1 = self.dy_a * self.x;

        let det0 = self.dx_a * self.dy_b;
        let det1 = self.dx_b * self.dy_a;
        if det1 == det0 {
            // 0 determinant is not solvable
            return None;
        }

        // We're only looking for positive solutions, so either of the following must be true
        if det0 > det1 && n0 > n1 && m0 > m1 {
            let det = det0 - det1;
            let n = n0 - n1;
            let m = m0 - m1;
            // Only integer solutions are valid
            if n % det == 0 && m % det == 0 {
                return Some((n / det, m / det));
            }
        } else if det1 > det0 && n1 > n0 && m1 > m0 {
            let det = det1 - det0;
            let n = n1 - n0;
            let m = m1 - m0;
            // Only integer solutions are valid
            if n % det == 0 && m % det == 0 {
                return Some((n / det, m / det));
            }
        }

        None
    }
}

fn main() {
    let start = Instant::now();

    let (_, equations) = many0(parse_eq)(INPUT).unwrap();

    // println!("Equations: {:#?}", equations);

    let solutions: Vec<(u64, u64)> = equations.iter().filter_map(|eq| eq.solve()).collect();

    // println!("Solutions: {:?}", solutions);

    let result = solutions.iter().map(|(x, y)| x * 3 + y).sum::<u64>();

    let elapsed = start.elapsed();

    println!("Result: {}", result);
    println!("(took: {:?})", elapsed);
}
