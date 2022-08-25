#![deny(clippy::all)]
#![warn(clippy::pedantic)]
//#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]

use anyhow::{anyhow, Result};
//use ndarray::prelude::*;
use nom::{bytes::complete::tag, character::complete::u8, multi::separated_list1, IResult};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    println!(
        "Answer: {:?}",
        process(&std::fs::read_to_string("inputs/day06.txt")?)?
    );

    Ok(())
}

// Convert nom's IResult to anyhow's Result, discarding any remaining input
fn finish<A>(parsed: IResult<&str, A>) -> Result<A> {
    parsed
        .map_err(|e| anyhow!(e.to_string()))
        .map(|(_input, value)| value)
}

// Hope there's never more than this many fish with any given timer value
type FishCount = u32;
// Fish timer is between 0 and 8
const TIMER_END: usize = 9;

type SchoolStorage = [FishCount; TIMER_END];
#[derive(Debug, Default)]
struct School {
    counts: SchoolStorage,
    zero_index: usize,
}

impl School {
    fn new(fishes: Vec<u8>) -> Self {
        let mut counts = SchoolStorage::default();
        for timer in fishes {
            let timer = usize::from(timer);
            if timer >= TIMER_END {
                panic!("Invalid input, fish timer too large: {}", timer);
            }
            counts[timer] += 1;
        }
        School {
            counts,
            zero_index: 0,
        }
    }

    fn advance(&mut self) {
        let of_day = |i| (self.zero_index + i) % TIMER_END;

        // Keep the zero count as is; it will become the new fish with timer 8
        // self.counts[of_day((TIMER_END - 1) + 1)] = self.counts[self.zero_index];

        // And also add the zero count to day 6
        self.counts[of_day(6 + 1)] += self.counts[self.zero_index];

        // It's now the next day; 1 -> 0, ..., 8 -> 7, 0 -> 8.
        self.zero_index = of_day(1);
    }
}

/*
Part 1:
Answer: 375482
dhat: Total:     2,657 bytes in 10 blocks
dhat: At t-gmax: 1,624 bytes in 2 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
0.00user 0.00system 0:00.00elapsed 100%CPU (0avgtext+0avgdata 10440maxresident)k
0inputs+48outputs (0major+831minor)pagefaults 0swaps

*/

fn process(input: &str) -> Result<u64> {
    let mut school = School::new(finish(separated_list1(tag(","), u8)(input))?);
    for _day in 0..80 {
        //dbg!(&school);
        school.advance();
    }
    dbg!(&school);
    Ok(school.counts.iter().map(|v| u64::from(*v)).sum())
}
