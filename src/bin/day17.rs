//#![feature(trait_alias)]

use anyhow::Result;

type Pos = (i32, i32);

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Target {
    pub x1: i32,
    pub y1: i32,
    pub x2: i32,
    pub y2: i32,
}

peg::parser! {
    grammar target_parser() for str {
        rule _ -> ()
            = quiet!{[' ' | '\t']*} {}

        rule __() -> ()
            = quiet!{[' ' | '\t']+} {}

        rule integer() -> i32
            = sign:"-"? num:['0'..='9']+ {
                num.into_iter().collect::<String>().parse::<i32>().unwrap() * sign.map(|_| -1).unwrap_or(1)
            }

        // Is there a more straightforward way to match a parameter?
        // Is there a way to get the expected character into the error message &str?
        rule axis_label(axis: char)
            = [c] {?
                (c == axis).then_some(()).ok_or("axis label")
            }

        rule target_dim(axis: char) -> Pos
            =  axis_label(axis) _ "=" _ from:integer() _ ".." _ to:integer() {
                (from, to)
            }

        rule eol()
            = "\r"? "\n"

        pub rule target() -> Target
            = "target" __ "area" _ ":" _
                x:target_dim('x') _ "," _
                y:target_dim('y') _ eol()?
            {
                Target {
                    x1: x.0,
                    y1: y.0,
                    x2: x.1,
                    y2: y.1,
                }
            }
    }
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
    let target = target_parser::target(input)?;
    eprintln!("{:?}", &target);

    Ok(())
}
