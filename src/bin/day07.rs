#![deny(clippy::all)]
#![warn(clippy::pedantic)]
//#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]

//use ndarray::prelude::*;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

type Coord = i64;

fn cost(n: u64) -> u64 {
    (1..=n).sum()
}

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    assert_eq!(cost(4), 10);

    let nums = include_str!("../../inputs/day07.txt")
        .trim()
        .split(',')
        .map(|v| v.parse().unwrap())
        .collect::<Vec<_>>();

    let max_coord = *nums.iter().max().unwrap();
    let mut cost_per_position = (0..=max_coord)
        .map(|pos| {
            (
                pos,
                nums.iter()
                    .map(|n| cost((n - pos as Coord).unsigned_abs()))
                    .sum::<u64>(),
            )
        })
        .collect::<Vec<_>>();

    cost_per_position.sort_by(|a, b| a.1.cmp(&b.1));
    //dbg!(&cost_per_position);
    println!("Answer: {:?}", cost_per_position[0]);
}

/*
Final result:
Answer: (484, 93214037)
dhat: Total:     62,672 bytes in 13 blocks
dhat: At t-gmax: 53,488 bytes in 4 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
0.00user 0.00system 0:00.00elapsed 100%CPU (0avgtext+0avgdata 9868maxresident)k
0inputs+16outputs (0major+788minor)pagefaults 0swaps
*/
