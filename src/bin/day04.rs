#![deny(clippy::all)]
#![warn(clippy::pedantic)]
//#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]

use anyhow::{anyhow, Result};
use ndarray::prelude::*;
use nom::{
    bytes::complete::tag,
    character::complete::{line_ending, space0, u8},
    combinator::{all_consuming, map},
    multi::{fold_many_m_n, many1, many_m_n, separated_list1},
    sequence::{preceded, terminated, tuple},
    IResult,
};
//use rayon::prelude::*;

use std::io::Read;

type Value = u8;
const FLAG: Value = Value::MAX;

fn numbers(input: &str) -> IResult<&str, Vec<Value>> {
    separated_list1(tag(","), u8)(input)
}

const BOARD_COLUMNS: usize = 5;
const BOARD_ROWS: usize = 5;
const BOARD_SIZE: usize = BOARD_COLUMNS * BOARD_ROWS;
type Board = Array2<Value>;

fn board(input: &str) -> IResult<&str, Board> {
    map(
        fold_many_m_n(
            BOARD_ROWS,
            BOARD_ROWS,
            terminated(
                many_m_n(BOARD_COLUMNS, BOARD_COLUMNS, preceded(space0, u8)),
                preceded(space0, line_ending),
            ),
            || Vec::with_capacity(BOARD_SIZE),
            |mut board, mut row| {
                board.append(&mut row);
                board
            },
        ),
        |v| Array::from_shape_vec((5, 5), v).unwrap(),
    )(input)
}

fn bingo(input: &str) -> IResult<&str, (Vec<Value>, Vec<Board>)> {
    tuple((
        terminated(numbers, line_ending),
        many1(preceded(line_ending, board)),
    ))(input)
}

/*
With [Value; BOARD_SIZE]:
dhat: Total:     9,340 bytes in 10 blocks
dhat: At t-gmax: 8,836 bytes in 4 blocks
dhat: At t-end:  8,192 bytes in 1 blocks
(+0 bytes)

With Vec<Value>::with_capacity(BOARD_SIZE):
dhat: Total:     9,411 bytes in 13 blocks
dhat: At t-gmax: 8,907 bytes in 7 blocks
dhat: At t-end:  8,192 bytes in 1 blocks
(+71 bytes)

With many_m_n + buf.append:
dhat: Total:     9,501 bytes in 31 blocks
dhat: At t-gmax: 8,917 bytes in 9 blocks
dhat: At t-end:  8,192 bytes in 1 blocks
(+161/+81 bytes)

With fold_many + many_m_n + append:
dhat: Total:     9,486 bytes in 28 blocks
dhat: At t-gmax: 8,912 bytes in 8 blocks
dhat: At t-end:  8,192 bytes in 1 blocks
(+146/+76 bytes)

With ndarray(fold_many + many_m_n + append):
dhat: Total:     9,661 bytes in 34 blocks
dhat: At t-gmax: 9,072 bytes in 8 blocks
dhat: At t-end:  8,192 bytes in 1 blocks
(+321/+236 bytes)

... And with REAL INPUT:
dhat: Total:     46,420 bytes in 821 blocks
dhat: At t-gmax: 27,209 bytes in 105 blocks
dhat: At t-end:  8,192 bytes in 1 blocks

... And .par_iter_mut() and flagging first number with map_inplace():
dhat: Total:     147,694 bytes in 979 blocks
dhat: At t-gmax: 119,377 bytes in 256 blocks
dhat: At t-end:  100,360 bytes in 151 blocks

... And flagging ALL numbers:
dhat: Total:     149,214 bytes in 980 blocks
dhat: At t-gmax: 120,892 bytes in 255 blocks
dhat: At t-end:  100,360 bytes in 151 blocks


End of Part 1:
Winning score = 51034
dhat: Total:     46,976 bytes in 623 blocks
dhat: At t-gmax: 28,228 bytes in 105 blocks
dhat: At t-end:  9,216 bytes in 2 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
0.00user 0.00system 0:00.01elapsed 90%CPU (0avgtext+0avgdata 10892maxresident)k
0inputs+32outputs (0major+879minor)pagefaults 0swaps

End of Part 2:
(Winner, Loser) = (51034, 5434)
dhat: Total:     64,736 bytes in 676 blocks
dhat: At t-gmax: 41,796 bytes in 149 blocks
dhat: At t-end:  9,216 bytes in 2 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
0.00user 0.00system 0:00.01elapsed 92%CPU (0avgtext+0avgdata 10832maxresident)k
0inputs+32outputs (0major+879minor)pagefaults 0swaps

*/

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn winner_and_loser(numbers: &[Value], boards: &mut Vec<Board>) -> Result<(u32, u32)> {
    fn winning_board(board: &Board) -> bool {
        for axis in 0..2 {
            for row in board.axis_iter(Axis(axis)) {
                if row.iter().filter(|v| **v == FLAG).count() == row.len() {
                    return true;
                }
            }
        }
        false
    }

    fn board_score(board: &Board, val: Value) -> u32 {
        board
            .iter()
            .filter(|v| **v != FLAG)
            .map(|v| u32::from(*v))
            .sum::<u32>()
            * u32::from(val)
    }

    let mut rounds = Vec::<(Value, Vec<Board>)>::new();
    for num in numbers {
        // Mark number as seen on all boards
        for board in boards.iter_mut() {
            board.map_inplace(|v| {
                if *v == *num {
                    *v = FLAG;
                }
            });
        }

        // Could use drain_filter().collect() for this?
        let mut winners = Vec::<Board>::new();
        let mut i = 0;
        while i < boards.len() {
            if winning_board(&boards[i]) {
                winners.push(boards.remove(i));
            } else {
                i += 1;
            }
        }

        // Don't track rounds where no boards won
        if !winners.is_empty() {
            rounds.push((*num, winners));
        }

        if boards.is_empty() {
            break;
        }
    }

    if rounds.is_empty() {
        return Err(anyhow!("Invalid input; no board wins: {:?}", &boards));
    }

    //dbg!(&rounds);
    let first = rounds.first().unwrap();
    let last = rounds.last().unwrap();

    if first.1.len() != 1 {
        return Err(anyhow!("Invalid input; tie for first: {:?}", &first));
    }
    if last.1.len() != 1 {
        return Err(anyhow!("Invalid input; tie for last: {:?}", &last));
    }

    return Ok((
        board_score(first.1.first().unwrap(), first.0),
        board_score(last.1.first().unwrap(), last.0),
    ));
}

fn main() -> Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let (_, (numbers, mut boards)) =
        all_consuming(bingo)(&input).map_err(|e| anyhow!(e.to_string()))?;

    //dbg!(&numbers);

    println!(
        "(Winner, Loser) = {:?}",
        winner_and_loser(&numbers, &mut boards)?
    );

    Ok(())
}
