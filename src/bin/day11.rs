#![feature(mixed_integer_ops)]
#![deny(clippy::all)]
#![warn(clippy::pedantic)]
//#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]

use anyhow::{anyhow, Result};
use itertools::Itertools;
use std::collections::VecDeque;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    println!(
        "Result: {:?}",
        process(&std::fs::read_to_string("inputs/day11.txt")?)
    );
    Ok(())
}

fn process(input: &str) -> Result<(usize, Option<usize>)> {
    let mut octopi = Octopi(input.parse()?);
    //dbg!(&octopi);
    let mut flashes = 0;
    let mut first_time = None;
    // For Part 1, just make this range `0..100`
    for step in 0.. {
        let this_time = octopi.step();
        //eprintln!("{step} {this_time} {}", octopi.size());
        flashes += this_time;
        //dbg!(&octopi);
        if this_time == octopi.size() {
            first_time = first_time.or(Some(step + 1));
            break;
        }
    }
    Ok((flashes, first_time))
}

#[derive(Debug)]
struct Octopi(DigitGrid);

impl Octopi {
    fn size(&self) -> usize {
        self.0 .0.len() * self.0 .0[0].len()
    }
    fn step(&mut self) -> usize {
        // Initially, we increment every cell: they're all active
        let mut active: VecDeque<_> = self.0.coords().collect();
        let mut flashes = 0;
        while let Some(pos) = active.pop_front() {
            *self.0.at_mut(pos) += 1;
            if *self.0.at(pos) == 9 + 1 {
                flashes += 1;
                let neighbors = self.0.neighbors8(pos).into_iter().filter_map(identity);
                //dbg!(neighbors.clone().collect::<Vec<_>>());
                active.extend(neighbors);
            }
            //eprintln!("({}, {}) = {} / {}", pos.0, pos.1, *self.0.at(pos), flashes);
        }
        for c in self.0.cells_mut() {
            if *c > 9 {
                *c = 0;
            }
        }
        flashes
    }
}

struct DigitGrid(Vec<Vec<u8>>);

impl DigitGrid {
    fn at(&self, (x, y): (usize, usize)) -> &u8 {
        &self.0[y][x]
    }

    fn at_mut(&mut self, (x, y): (usize, usize)) -> &mut u8 {
        &mut self.0[y][x]
    }

    fn coords(&self) -> impl Iterator<Item = (usize, usize)> {
        (0..self.0[0].len()).cartesian_product(0..self.0.len())
    }

    fn cells_mut(&mut self) -> impl Iterator<Item = &mut u8> {
        self.0.iter_mut().flatten()
    }

    fn neighbors8(&self, (x, y): (usize, usize)) -> [Option<(usize, usize)>; 8] {
        let mut neighbors = [None; 8];

        [
            (-1isize, -1isize),
            (0, -1),
            (1, -1),
            (1, 0),
            (1, 1),
            (0, 1),
            (-1, 1),
            (-1, 0),
        ]
        .iter()
        .map(|&(dx, dy)| {
            x.checked_add_signed(dx)
                .and_then(|x| if x >= self.0[0].len() { None } else { Some(x) })
                .and_then(|x| {
                    y.checked_add_signed(dy)
                        .and_then(|y| if y >= self.0.len() { None } else { Some(y) })
                        .map(|y| (x, y))
                })
        })
        .enumerate()
        .for_each(|(i, n)| neighbors[i] = n);
        neighbors
    }
}

impl std::str::FromStr for DigitGrid {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let data = s
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| {
                        c.to_digit(10)
                            .map(|d| d.try_into().expect("Infallible cast {d} to u8"))
                            .ok_or_else(|| anyhow!("Invalid input char {c}"))
                    })
                    .collect::<Result<Vec<_>>>()
            })
            .collect::<Result<Vec<Vec<_>>>>()?;
        if data.is_empty()
            || data[0].is_empty()
            || !data.iter().all(|row| row.len() == data[0].len())
        {
            panic!("Input is not a MxN grid of digits: {data:#?}");
        }

        Ok(Self(data))
    }
}

impl std::fmt::Display for DigitGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.0 {
            writeln!(
                f,
                "{}",
                row.iter()
                    .map(|digit| (digit + b'0') as char)
                    .collect::<String>()
            )?;
        }
        Ok(())
    }
}

impl std::fmt::Debug for DigitGrid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

const fn identity<A>(a: A) -> A {
    a
}

/*
Part 2:
Result: Ok((6551, Some(418)))
dhat: Total:     2,136,399 bytes in 7,000 blocks
dhat: At t-gmax: 4,942 bytes in 14 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
0.01user 0.02system 0:00.03elapsed 100%CPU (0avgtext+0avgdata 10600maxresident)k
0inputs+32outputs (0major+850minor)pagefaults 0swaps

After filling neighbors array directly rather than with Vec<_>.collect().try_into():
Result: Ok((6551, Some(418)))
dhat: Total:     878,607 bytes in 449 blocks
dhat: At t-gmax: 4,750 bytes in 13 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
0.00user 0.00system 0:00.01elapsed 90%CPU (0avgtext+0avgdata 10668maxresident)k
0inputs+88outputs (0major+852minor)pagefaults 0swaps

*/
