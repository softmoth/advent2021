#![deny(clippy::all)]
#![warn(clippy::pedantic)]
//#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]

use anyhow::{anyhow, Context, Result};
use bitvec::prelude::*;
//use ndarray::prelude::*;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, line_ending, space0, space1},
    combinator::{all_consuming, map},
    multi::{many1, separated_list1},
    sequence::{delimited, preceded, separated_pair, terminated},
    IResult,
};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    println!(
        "Answer: {:?}",
        process(&std::fs::read_to_string("inputs/day08.txt")?)?
    );

    Ok(())
}

// Convert nom's IResult to anyhow's Result, discarding any remaining input
fn finish<A>(parsed: IResult<&str, A>) -> Result<A> {
    parsed
        .map_err(|e| anyhow!(e.to_string()))
        .map(|(_input, value)| value)
}

type Segments = BitArr!(for 7, in u8, Msb0);
#[derive(Debug)]
struct Entry {
    patterns: Vec<Segments>,
    display: Vec<Segments>,

    digits: Vec<Segments>,
}

impl Entry {
    fn new(patterns: Vec<Segments>, display: Vec<Segments>) -> Self {
        let digits = vec![Segments::default(); 10];

        Self {
            patterns,
            display,
            digits,
        }
    }

    /*
    length: candidates
    2: 1
    3: 7
    4: 4
    5: 2, 3, 5, 9
    6: 0, 6, 9
    7: 8

    a = 7 & ~1
    */

    fn solve(&mut self) -> u32 {
        0
    }
}

fn segments(input: &str) -> IResult<&str, Segments> {
    map(alpha1, |s: &str| {
        let data = s
            .chars()
            .map(|c| u8::try_from(c).unwrap() - b'a')
            .fold(0u8, |acc, v| acc | 1 << v);
        Segments::new([data; 1])
    })(input)
}

fn entry(input: &str) -> IResult<&str, Entry> {
    map(
        terminated(
            separated_pair(
                separated_list1(space1, segments),
                delimited(space0, tag("|"), space0),
                separated_list1(space1, segments),
            ),
            preceded(space0, line_ending),
        ),
        |(patterns, display)| Entry::new(patterns, display),
    )(input)
}

fn process(input: &str) -> Result<(usize, u64)> {
    let mut entries: Vec<Entry> =
        finish(all_consuming(many1(entry))(input)).context("parsing entries")?;
    //dbg!(&entries);

    // Part 1
    let part1 = entries
        .iter()
        .map(|e| {
            e.display
                .iter()
                .map(|d| d.count_ones())
                .filter(|len| matches!(len, 2 | 3 | 4 | 7))
                .count()
        })
        .sum();

    let part2 = entries
        .iter_mut()
        .map(|e| u64::from(e.solve()))
        .sum::<u64>();
    Ok((part1, part2))
}

/*
At end of part 1:
Answer: 239
dhat: Total:     144,912 bytes in 810 blocks
dhat: At t-gmax: 93,375 bytes in 402 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
0.00user 0.00system 0:00.01elapsed 90%CPU (0avgtext+0avgdata 10676maxresident)k
0inputs+16outputs (0major+852minor)pagefaults 0swaps

Part 1, using bitvec instead of strings:
Answer: 239
dhat: Total:     48,912 bytes in 610 blocks
dhat: At t-gmax: 34,175 bytes in 402 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
0.01user 0.00system 0:00.01elapsed 100%CPU (0avgtext+0avgdata 10828maxresident)k
0inputs+16outputs (0major+861minor)pagefaults 0swaps

*/
