#![deny(clippy::all)]
#![warn(clippy::pedantic)]
//#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]

use anyhow::{anyhow, Context, Result};
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
type FishCount = u64;
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
            assert!(
                timer < TIMER_END,
                "Invalid input, fish timer too large: {}",
                timer
            );
            counts[timer] += 1;
        }

        Self {
            counts,
            zero_index: 0,
        }
    }

    fn advance(&mut self) -> Result<()> {
        let of_day = |i| (self.zero_index + i) % TIMER_END;

        // Keep the zero count as is; it will become the new fish with timer 8
        // self.counts[of_day((TIMER_END - 1) + 1)] = self.counts[self.zero_index];

        // And also add the zero count to (the upcoming) day 6
        self.counts[of_day(6 + 1)] = self.counts[of_day(6 + 1)]
            .checked_add(self.counts[self.zero_index])
            .ok_or_else(|| anyhow!("Counter overflowed, need more bits!"))
            .context("Advancing a new generation")?;

        // It's now the next day; 1 -> 0, ..., 8 -> 7, 0 -> 8.
        self.zero_index = of_day(1);

        Ok(())
    }
}

/*
Part 1 (80 days):
Answer: 375482
dhat: Total:     2,657 bytes in 10 blocks
dhat: At t-gmax: 1,624 bytes in 2 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
0.00user 0.00system 0:00.00elapsed 100%CPU (0avgtext+0avgdata 10440maxresident)k
0inputs+48outputs (0major+831minor)pagefaults 0swaps

Part 2 (256 days):
Answer: 1689540415957
dhat: Total:     2,657 bytes in 10 blocks
dhat: At t-gmax: 1,624 bytes in 2 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
0.00user 0.00system 0:00.00elapsed 100%CPU (0avgtext+0avgdata 10728maxresident)k
0inputs+16outputs (0major+837minor)pagefaults 0swaps

Part 3 (Max days that will fit into a u64 final result = 442):
Answer: 18353315898976047013
dhat: Total:     2,657 bytes in 10 blocks
dhat: At t-gmax: 1,624 bytes in 2 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
0.00user 0.00system 0:00.00elapsed 80%CPU (0avgtext+0avgdata 10464maxresident)k
0inputs+16outputs (0major+828minor)pagefaults 0swaps

*/

fn process(input: &str) -> Result<u64> {
    let mut school = School::new(finish(separated_list1(tag(","), u8)(input))?);
    for _day in 0..256 {
        //dbg!(&school);
        school.advance()?;
    }
    dbg!(&school);
    // NB: .sum() isn't checked, so implement it via a fold
    school
        .counts
        .iter()
        .try_fold(0u64, |acc, v| acc.checked_add(*v))
        .ok_or_else(|| anyhow!("Counter overflowed, need more bits!"))
        .context("Final sum")
}
