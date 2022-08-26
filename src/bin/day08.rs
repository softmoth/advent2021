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
    wires: Vec<Segments>,
}

impl Entry {
    fn new(patterns: Vec<Segments>, display: Vec<Segments>) -> Self {
        let digits = vec![Segments::default(); 10];
        let wires = vec![Segments::default(); 7];

        Self {
            patterns,
            display,
            digits,
            wires,
        }
    }

    // Remove and return the pattern (assuming exactly one match) that:
    // - has num_segments bits set, and
    // - satisfies predicate(self, pattern)
    fn grab_pattern(
        &mut self,
        num_segments: usize,
        predicate: fn(&Self, &Segments) -> bool,
    ) -> Segments {
        let pos: usize = self
            .patterns
            .iter()
            .position(|pat| pat.count_ones() == num_segments && predicate(self, pat))
            .unwrap();
        self.patterns.swap_remove(pos)
    }

    fn solve(&mut self) -> u32 {
        // First, the easy ones: each of these digits has a unique number of segments on
        self.digits[1] = self.grab_pattern(2, |_, _| true);
        self.digits[7] = self.grab_pattern(3, |_, _| true);
        self.digits[4] = self.grab_pattern(4, |_, _| true);
        self.digits[8] = self.grab_pattern(7, |_, _| true);

        // 1 & 6 overlap by 1, whereas 1 & 9 and 1 & 0 overlap by 2
        self.digits[6] =
            self.grab_pattern(6, |self_, pat| (*pat & self_.digits[1]).count_ones() == 1);

        // Segment 'b' is known from 8 - 6
        self.wires[1] = self.digits[8] & !self.digits[6];

        // 5 is the only 5-segment number with segment 'b' off
        self.digits[5] =
            self.grab_pattern(5, |self_, pat| (*pat & self_.wires[1]).count_ones() == 0);

        // Segment 'f' is known from 1 - b
        self.wires[5] = self.digits[1] & !self.wires[1];

        // 2 is the only 5-segment number with segment 'f' off
        self.digits[2] =
            self.grab_pattern(5, |self_, pat| (*pat & self_.wires[5]).count_ones() == 0);

        // 9 is the only 6-segment number that completely covers 4
        self.digits[9] = self.grab_pattern(6, |self_, pat| {
            let d4 = self_.digits[4];
            (d4 & *pat).count_ones() == d4.count_ones()
        });

        // 3 is the only 5-segment number remaining
        self.digits[3] = self.grab_pattern(5, |_, _| true);

        // 0 is the only 6-segment number remaining
        self.digits[0] = self.grab_pattern(6, |_, _| true);

        let dlen = self.display.len() as u32;

        (0..dlen)
            .map(|i| {
                10_u32.pow(dlen - 1 - i)
                    * self
                        .digits
                        .iter()
                        .position(|&d| d == self.display[i as usize])
                        .unwrap() as u32
            })
            .sum::<u32>()
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
    //dbg!(&entries.first().unwrap().digits);
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

Part 2:
Answer: (239, 946346)
dhat: Total:     76,696 bytes in 1,010 blocks
dhat: At t-gmax: 49,863 bytes in 802 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
0.00user 0.00system 0:00.01elapsed 100%CPU (0avgtext+0avgdata 11112maxresident)k
0inputs+24outputs (0major+879minor)pagefaults 0swaps

*/
