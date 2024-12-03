use nom::{
    bytes::complete::{tag, take},
    character::complete::digit1,
    combinator::map_res,
    multi::many_till,
    sequence::{delimited, separated_pair},
    Err, IResult,
};

const INPUT: &'static str = include_str!("input.txt");

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

fn parse_mul(input: &str) -> IResult<&str, (u32, u32)> {
    let (input, parsed) = delimited(tag("mul("), parse_tuple, tag(")"))(input)?;

    Ok((input, parsed))
}

fn parse_one(input: &str) -> IResult<&str, (u32, u32)> {
    // Skip garbage until the parser matches
    let (input, (_, x)) = many_till(take(1usize), parse_mul)(input)?;
    Ok((input, x))
}

fn parse_all(input: &str) -> Vec<(u32, u32)> {
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

#[test]
fn test_parse() {
    assert_eq!(parse_one("mul(1,2)"), Ok(("", (1, 2))));
    assert_eq!(parse_one("mul(111,222)"), Ok(("", (111, 222))));
    assert!(parse_one("mul(111,2222)").is_err());

    assert_eq!(parse_one("garbagemul(111,333)"), Ok(("", (111, 333))));
    assert_eq!(
        parse_one("garbagemul(111,333)moregarbage"),
        Ok(("moregarbage", (111, 333)))
    );

    assert_eq!(parse_all("mul(1,2)"), vec![(1, 2)]);
    assert_eq!(parse_all("mul(1,2)mul(3,4)"), vec![(1, 2), (3, 4)]);
    assert_eq!(parse_all("mul(1,2)mul(*mul(5,6)"), vec![(1, 2), (5, 6)]);
}

fn main() {
    let tuples = parse_all(INPUT);
    let result = tuples.iter().map(|(x, y)| x * y).sum::<u32>();
    println!("Result: {}", result);
}
