#![deny(clippy::all)]
#![warn(clippy::pedantic)]
//#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]

use anyhow::{anyhow, Result};
use ndarray::prelude::*;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space0, u16},
    combinator::{all_consuming, map},
    multi::many1,
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
        process(&std::fs::read_to_string("inputs/day05.txt")?)?
    );

    Ok(())
}

// Convert nom's IResult to anyhow's Result, discarding any remaining input
fn finish<A>(parsed: IResult<&str, A>) -> Result<A> {
    parsed
        .map_err(|e| anyhow!(e.to_string()))
        .map(|(_input, value)| value)
}

/*
Parsing input only (sample):
dhat: Total:     1,375 bytes in 6 blocks
dhat: At t-gmax: 1,262 bytes in 3 blocks
dhat: At t-end:  1,024 bytes in 1 blocks

Grid with sums in each cell (sample):
dhat: Total:     2,152 bytes in 9 blocks
dhat: At t-gmax: 1,134 bytes in 2 blocks
dhat: At t-end:  1,024 bytes in 1 blocks

At end of Part 1 (full input):
Answer: 7438
dhat: Total:     1,023,058 bytes in 12 blocks
dhat: At t-gmax: 1,005,761 bytes in 3 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
0.00user 0.00system 0:00.00elapsed 100%CPU (0avgtext+0avgdata 10656maxresident)k
0inputs+16outputs (0major+1087minor)pagefaults 0swaps

At end of Part 2:
Answer: 21406
dhat: Total:     1,023,058 bytes in 12 blocks
dhat: At t-gmax: 1,005,761 bytes in 3 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
0.00user 0.00system 0:00.00elapsed 100%CPU (0avgtext+0avgdata 10720maxresident)k
0inputs+16outputs (0major+1091minor)pagefaults 0swaps

*/

fn process(input: &str) -> Result<usize> {
    let lines: Vec<Line> = finish(all_consuming(lines)(input))?;

    let grid_size = lines
        .iter()
        .fold((0, 0), |(max_x, max_y), &Line { x1, y1, x2, y2 }| {
            (
                // Coord starts from zero
                max_x.max(x1.max(x2) + 1),
                max_y.max(y1.max(y2) + 1),
            )
        });
    let mut grid = Array::<u8, _>::zeros(grid_size);

    for Line { x1, y1, x2, y2 } in lines {
        if x1 == x2 {
            let yy = y1.min(y2)..=y1.max(y2);
            grid.slice_mut(s![x1, yy]).mapv_inplace(|v| v + 1);
        } else if y1 == y2 {
            let xx = x1.min(x2)..=x1.max(x2);
            grid.slice_mut(s![xx, y1]).mapv_inplace(|v| v + 1);
        } else {
            // Part 1
            //continue;

            // ndarray::SliceInfoElem doesn't do diagonals

            let (mut x, mut y) = (x1, y1);
            loop {
                grid[[x, y]] += 1;
                if x == x2 || y == y2 {
                    break;
                }
                x = if x < x2 { x + 1 } else { x - 1 };
                y = if y < y2 { y + 1 } else { y - 1 };
            }
        }
    }

    let answer = grid.iter().filter(|v| **v > 1).count();

    // ndarray slices are [row, column], not [x, y]
    //dbg!(grid.reversed_axes(), answer);

    Ok(answer)
}

type Coord = usize;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Line {
    x1: Coord,
    y1: Coord,
    x2: Coord,
    y2: Coord,
}

fn point(input: &str) -> IResult<&str, (Coord, Coord)> {
    map(separated_pair(u16, tag(","), u16), |(x, y)| {
        (usize::from(x), usize::from(y))
    })(input)
}

fn line(input: &str) -> IResult<&str, Line> {
    map(
        separated_pair(point, delimited(space0, tag("->"), space0), point),
        |((x1, y1), (x2, y2))| Line { x1, y1, x2, y2 },
    )(input)
}

fn lines(input: &str) -> IResult<&str, Vec<Line>> {
    many1(terminated(line, preceded(space0, line_ending)))(input)
}
