use anyhow::{anyhow, Result};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i32, space1},
    combinator::{map, value},
    sequence::separated_pair,
    IResult,
};

use std::ops::Neg;

#[derive(Clone, Debug, PartialEq, Eq)]
enum Command {
    Forward(i32),
    Down(i32),
    Up(i32),
}

#[derive(Debug)]
struct Submarine {
    x: i32,
    y: i32,
    aim: i32,
}

impl Default for Submarine {
    fn default() -> Self {
        Self { x: 0, y: 0, aim: 0 }
    }
}

impl Submarine {
    fn propel(&self, distance: i32) -> Self {
        Self {
            x: self.x + distance,
            y: self.y + distance * self.aim,
            aim: self.aim,
        }
    }

    fn aim(&self, amount: i32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            aim: self.aim + amount,
        }
    }

    fn finish(&self) -> Result<()> {
        dbg!(self);
        println!(
            "Final answer: {} * {} = {}",
            self.x,
            self.y,
            self.x * self.y
        );
        Ok(())
    }
}

fn command_name(input: &str) -> IResult<&str, fn(i32) -> Command> {
    alt((
        value(Command::Forward as fn(i32) -> Command, tag("forward")),
        value(Command::Down as fn(i32) -> Command, tag("down")),
        value(Command::Up as fn(i32) -> Command, tag("up")),
    ))(input)
}

fn command(input: &str) -> IResult<&str, Command> {
    map(separated_pair(command_name, space1, i32), |(c, v)| c(v))(input)
}

fn main() -> Result<()> {
    let mut submarine = Submarine::default();
    for line in std::io::stdin().lines() {
        let line = line?;
        let command = command(&line)
            // Have to parse then get rid of the nom Error type which holds a reference to the
            // input string (&line) and otherwise would require 'static
            .map_err(|err| anyhow!("{}", err.to_string()))?;
        submarine = match command.1 {
            Command::Forward(v) => submarine.propel(v),
            Command::Down(v) => submarine.aim(v),
            Command::Up(v) => submarine.aim(v.neg()),
        };
    }
    submarine.finish()
}
