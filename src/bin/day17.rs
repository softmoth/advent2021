#![feature(trait_alias)]

use anyhow::{anyhow, Result};

use combine::{
    eof, many1, optional,
    parser::char::{char, digit, space, spaces, string},
    stream::position,
    EasyParser, Parser, Stream,
};

trait CharStream = Stream<Token = char>;

type Pos = (i32, i32);

#[derive(PartialEq, Eq, Debug, Clone)]
struct Target {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

fn integer<Input: CharStream>() -> impl Parser<Input, Output = i32> {
    (optional(char('-')), many1(digit())).map(|(sign, digits): (_, String)| {
        sign.map(|_| -1).unwrap_or(1) * digits.parse::<i32>().unwrap()
    })
}

fn target_dimension<Input: CharStream>(axis: char) -> impl Parser<Input, Output = Pos> {
    (char(axis), char('='), integer(), string(".."), integer()).map(|(_, _, a, _, b)| (a, b))
}

fn target<Input: CharStream>() -> impl Parser<Input, Output = Target> {
    (
        (string("target area:"), space(), spaces()),
        target_dimension('x'),
        (char(','), space(), spaces()),
        target_dimension('y'),
        (spaces(), eof()),
    )
        .map(|(_, x_range, _, y_range, _)| Target {
            x1: x_range.0,
            y1: y_range.0,
            x2: x_range.1,
            y2: y_range.1,
        })
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        super::process("target area: x=20..30, y=-10..-5").unwrap();
    }
}

fn main() -> Result<()> {
    process(&std::fs::read_to_string("inputs/day17.txt")?)
}

fn process(input: &str) -> Result<()> {
    let (target, _) = target()
        .easy_parse(position::Stream::new(input))
        .map_err(|e| anyhow!("Parse error {e}"))?;

    eprintln!("{:?}", &target);

    Ok(())
}
