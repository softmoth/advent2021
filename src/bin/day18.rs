#![deny(clippy::all)]
#![warn(clippy::pedantic)]
//#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]
#![feature(box_syntax)]

use anyhow::{anyhow, Result};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    for line in std::fs::read_to_string("inputs/day18.txt")?.lines() {
        let n = parse_snailfish(line)?;
        eprintln!("{n:?}");
    }
    Ok(())
}

// [[[[4,4],[4,4]],[[4,4],[4,4]]], [[[4,4],[4,4]],[[4,4],[4,4]]]]

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SNum {
    Term(i32),
    Pair(Box<Self>, Box<Self>),
}

/// Parse a Snail num
/// # Errors
/// Fails on mal-formed input
pub fn parse_snailfish(line: &str) -> Result<SNum> {
    peg::parser! {
        grammar snailfish_number() for str {
            pub rule number() -> SNum
                = term() / pair()

            rule integer() -> i32
                = a:['0'..='9']+ {?
                    a.into_iter()
                        .collect::<String>()
                        .parse()
                        .ok()
                        .ok_or("i32")
                }

            pub rule term() -> SNum
                = n:integer() {
                    SNum::Term(n)
                }

            pub rule pair() -> SNum
                = "[" a:number() "," b:number() "]" {
                    SNum::Pair(box a, box b)
                }
        }
    }

    snailfish_number::pair(line).map_err(|e| anyhow!("Parse {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parsing() -> Result<()> {
        assert_eq!(
            super::parse_snailfish("[1,2]")?,
            SNum::Pair(box SNum::Term(1), box SNum::Term(2))
        );

        assert_eq!(
            super::parse_snailfish("[[1,2],3]")?,
            SNum::Pair(
                box SNum::Pair(box SNum::Term(1), box SNum::Term(2)),
                box SNum::Term(3)
            )
        );

        super::parse_snailfish("[9,[8,7]]")?;

        super::parse_snailfish("[[1,9],[8,5]]")?;

        assert_eq!(
            super::parse_snailfish("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]")?,
            SNum::Pair(
                box SNum::Pair(
                    box SNum::Pair(
                        box SNum::Pair(box SNum::Term(1), box SNum::Term(2)),
                        box SNum::Pair(box SNum::Term(3), box SNum::Term(4))
                    ),
                    box SNum::Pair(
                        box SNum::Pair(box SNum::Term(5), box SNum::Term(6)),
                        box SNum::Pair(box SNum::Term(7), box SNum::Term(8))
                    )
                ),
                box SNum::Term(9)
            )
        );

        super::parse_snailfish("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]")?;

        super::parse_snailfish("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]")?;

        Ok(())
    }
}
