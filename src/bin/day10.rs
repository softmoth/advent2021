#![deny(clippy::all)]
#![warn(clippy::pedantic)]
//#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]

use anyhow::Result;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    println!("Result: {:?}", process(&std::fs::read("inputs/day10.txt")?));
    Ok(())
}

fn process(input: &[u8]) -> (usize, usize) {
    let (part1, mut part2) = input
        // Inclusive split to avoid extra empty result after final newline
        .split_inclusive(|b| *b == b'\n')
        .map(check_line)
        .fold((0, Vec::<usize>::new()), |(part1, mut part2), check| {
            if let Some(score) = check.score_incomplete() {
                part2.push(score);
            }
            (part1 + check.score_error(), part2)
        });
    let mid = part2.len() / 2;
    (part1, *part2.select_nth_unstable(mid).1)
}

// NB input has trailing '\n'
fn check_line(input: &[u8]) -> SyntaxCheck {
    let check =
        input.iter().try_fold(
            Vec::<SymType>::new(),
            |mut state, in_byte| match Sym::try_from(*in_byte) {
                Ok(sym) => match sym.0 {
                    (symtype, true) => {
                        state.push(symtype);
                        Ok(state)
                    }
                    (symtype, false) => match state.pop() {
                        Some(t) if t == symtype => Ok(state),
                        _ => Err(SyntaxCheck::Error(*in_byte)),
                    },
                },
                Err(b'\n') => Ok(state),
                Err(_e) => panic!("Unexpected input {in_byte}"),
            },
        );
    match check {
        Err(e @ SyntaxCheck::Error(_)) => e,
        Err(e) => panic!("Impossible, resolved {e:?} too soon!"),
        Ok(state) if state.is_empty() => SyntaxCheck::Complete,
        Ok(state) => SyntaxCheck::Incomplete(state),
    }
}

#[derive(Debug, Clone)]
enum SyntaxCheck {
    Complete,
    Incomplete(Vec<SymType>),
    Error(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum SymType {
    Round,
    Square,
    Curly,
    Angle,
}

struct Sym((SymType, bool));

impl TryFrom<u8> for Sym {
    type Error = u8;

    fn try_from(byte: u8) -> Result<Self, Self::Error> {
        use SymType::*;
        match byte {
            b'(' => Ok(Self((Round, true))),
            b'[' => Ok(Self((Square, true))),
            b'{' => Ok(Self((Curly, true))),
            b'<' => Ok(Self((Angle, true))),
            b')' => Ok(Self((Round, false))),
            b']' => Ok(Self((Square, false))),
            b'}' => Ok(Self((Curly, false))),
            b'>' => Ok(Self((Angle, false))),
            _ => Err(byte),
        }
    }
}

impl SyntaxCheck {
    const fn score_error(&self) -> usize {
        match *self {
            Self::Error(b')') => 3,
            Self::Error(b']') => 57,
            Self::Error(b'}') => 1197,
            Self::Error(b'>') => 25137,
            _ => 0,
        }
    }

    fn score_incomplete(&self) -> Option<usize> {
        if let Self::Incomplete(rem) = self {
            Some(
                rem.iter()
                    .rev()
                    .fold(0, |acc, symtype| acc * 5 + (*symtype as usize + 1)),
            )
        } else {
            None
        }
    }
}

/*
Part 1:
Result: 339537
dhat: Total:     13,714 bytes in 207 blocks
dhat: At t-gmax: 11,249 bytes in 2 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
0.00user 0.00system 0:00.00elapsed 100%CPU (0avgtext+0avgdata 10240maxresident)k
0inputs+16outputs (0major+819minor)pagefaults 0swaps

Result: (339537, 2412013412)
dhat: Total:     14,706 bytes in 212 blocks
dhat: At t-gmax: 11,249 bytes in 2 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
0.00user 0.00system 0:00.00elapsed 100%CPU (0avgtext+0avgdata 10308maxresident)k
0inputs+80outputs (0major+819minor)pagefaults 0swaps

*/
