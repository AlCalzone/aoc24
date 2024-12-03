use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::{map, map_res},
    multi::many_till,
    sequence::{delimited, separated_pair},
    Err, IResult,
};

const INPUT: &'static str = include_str!("input.txt");

#[derive(Debug, PartialEq)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn parse_number(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| {
        if s.len() <= 3 {
            match s.parse::<u32>() {
                Ok(n) => Ok(n.into()),
                Err(_) => Err(Err::Error("Invalid number")),
            }
        } else {
            Err(Err::Error("Number too large"))
        }
    })(input)
}

fn parse_tuple(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, (x, y)) = separated_pair(parse_number, tag(","), parse_number)(input)?;
    Ok((input, (x, y)))
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
    let (input, parsed) = delimited(
        tag("mul("),
        map(parse_tuple, |(x, y)| Instruction::Mul(x, y)),
        tag(")"),
    )(input)?;

    Ok((input, parsed))
}

fn parse_do(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("do()")(input)?;
    Ok((input, Instruction::Do))
}

fn parse_dont(input: &str) -> IResult<&str, Instruction> {
    let (input, _) = tag("don't()")(input)?;
    Ok((input, Instruction::Dont))
}

fn parse_one(input: &str) -> IResult<&str, Instruction> {
    // Skip garbage until the parser matches
    let (input, (_, x)) = many_till(take(1usize), alt((parse_mul, parse_do, parse_dont)))(input)?;
    Ok((input, x))
}

fn parse_all(input: &str) -> Vec<Instruction> {
    let mut result = vec![];
    let mut input = input;
    loop {
        match parse_one(input) {
            Ok((new_input, x)) => {
                input = new_input;
                result.push(x);
            }
            Err(_) => break,
        }
    }
    result
}

fn main() {
    let instructions = parse_all(INPUT);
    let mut _do = true;
    let mut result: u32 = 0;
    for instr in instructions {
        match instr {
            Instruction::Mul(x, y) => {
                if _do {
                    result += x * y;
                }
            }
            Instruction::Do => {
                _do = true;
            }
            Instruction::Dont => {
                _do = false;
            }
        }
    }

    println!("Result: {}", result);
}
