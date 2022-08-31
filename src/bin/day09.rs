#![deny(clippy::all)]
#![warn(clippy::pedantic)]
//#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]

use anyhow::{anyhow, ensure, Result};
use itertools::Itertools;
use std::collections::BTreeSet;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    println!(
        "Answer: {:?}",
        process(&std::fs::read("inputs/day09.txt")?)?
    );

    Ok(())
}

fn process(input: &[u8]) -> Result<(i32, i32)> {
    let hm = Heightmap::new(input)?;
    let low_points = (0..hm.width)
        .cartesian_product(0..hm.height)
        .filter(|&loc| {
            let lowest_neighbor = Heightmap::neighbors(loc)
                .map(|nloc| hm.at(nloc))
                .min()
                .unwrap();
            hm.at(loc) < lowest_neighbor
        })
        .collect::<Vec<Location>>();

    let low_heights = low_points.iter().map(|&p| hm.at(p)).collect::<Vec<_>>();
    //dbg!(&low_points, &low_heights,);
    let part1 = low_heights.iter().map(|v| *v as i32 + 1).sum::<i32>();

    let mut part2 = low_points
        .iter()
        .map(|loc| basin_from_low_point(&hm, *loc).len() as i32)
        .collect::<Vec<_>>();
    part2.sort();
    let part2 = part2.iter().rev().take(3).product::<i32>();

    Ok((part1, part2))
}

fn basin_from_low_point(hm: &Heightmap, low_point: Location) -> BTreeSet<Location> {
    fn advance(
        hm: &Heightmap,
        basin: &mut BTreeSet<Location>,
        points: Vec<Location>,
    ) -> Vec<Location> {
        let mut new_points = Vec::<Location>::new();
        for loc in points {
            for n in Heightmap::neighbors(loc) {
                if hm.at(n) < 9 && !basin.contains(&n) {
                    basin.insert(n);
                    new_points.push(n);
                }
            }
        }
        new_points
    }
    let mut basin = BTreeSet::<Location>::new();
    let mut new_points = vec![low_point];
    loop {
        new_points = advance(hm, &mut basin, new_points);
        if new_points.is_empty() {
            break;
        }
    }
    //dbg!(&low_point, &basin);
    basin
}

type Axis = i32;
type Location = (i32, i32);
struct Heightmap<'a> {
    width: Axis,
    height: Axis,
    // This is the input data, unchanged. So b'0' (ASCII decimal 48) represents 0
    data: &'a [u8],
}

impl<'a> Heightmap<'a> {
    fn new(input: &'a [u8]) -> Result<Self> {
        let width = input
            .iter()
            .position(|&b| b == b'\n')
            .ok_or_else(|| anyhow!("No line ending in input!"))? as Axis;

        let heightmap = Self {
            data: input,
            width,
            height: input.len() as Axis / (width + 1),
        };

        ensure!(
            heightmap.data.len() as Axis == (heightmap.width + 1) * heightmap.height,
            "Impossible combo of (width, len): {:?}",
            (heightmap, input.len())
        );

        Ok(heightmap)
    }

    fn at(&self, (x, y): Location) -> i32 {
        if (0..self.width as Axis).contains(&x) && (0..self.height as Axis).contains(&y) {
            // Valid coordinates, look up data
            let index = ((self.width as Axis + 1) * y + x) as usize;
            // The bytes are all ascii digits
            i32::from(self.data[index].saturating_sub(b'0'))
        } else {
            // Out of range, return top height
            9
        }
    }

    // Takes a point, and iterates over the 4 *points* (not their values!) surrounding it.
    // It *does* return coordinates for entries outside of the grid, because `at()` handles
    // them properly
    fn neighbors((x, y): Location) -> impl Iterator<Item = Location> {
        [(1, 0), (0, 1), (-1, 0), (0, -1)]
            .into_iter()
            .map(move |(dx, dy)| (x + dx, y + dy))
    }
}

impl<'a> std::fmt::Debug for Heightmap<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        writeln!(f)?;
        for b in self.data {
            if *b == b'\n' {
                writeln!(f)?;
            } else {
                write!(f, "{}", b - b'0')?;
            }
        }
        Ok(())
    }
}

/*
Part 1:
Answer: (468, 0)
dhat: Total:     16,037 bytes in 11 blocks
dhat: At t-gmax: 12,980 bytes in 3 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
0.00user 0.00system 0:00.00elapsed 85%CPU (0avgtext+0avgdata 10404maxresident)k
0inputs+16outputs (0major+826minor)pagefaults 0swaps

Part 2:
Answer: (468, 1280496)
dhat: Total:     271,157 bytes in 3,478 blocks
dhat: At t-gmax: 16,020 bytes in 22 blocks
dhat: At t-end:  1,024 bytes in 1 blocks
dhat: The data has been saved to dhat-heap.json, and is viewable with dhat/dh_view.html
0.00user 0.01system 0:00.02elapsed 100%CPU (0avgtext+0avgdata 10824maxresident)k
0inputs+48outputs (0major+874minor)pagefaults 0swaps

*/
