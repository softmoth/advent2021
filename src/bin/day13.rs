#![deny(clippy::all)]
#![warn(clippy::pedantic)]
//#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    println!(
        "Result: {:?}",
        process(&std::fs::read_to_string("inputs/day13.txt").unwrap())
    );
}

type Coord = u32;

fn process(input: &str) -> usize {
    let mut grid = std::collections::HashSet::new();
    let mut folds = vec![];
    let mut max_x = 0;
    let mut max_y = 0;

    for line in input.lines() {
        #[allow(clippy::option_if_let_else)]
        if let Some(fold) = line.strip_prefix("fold along ") {
            let (axis, coord) = fold.split_once('=').unwrap();
            let axis = axis.chars().next().unwrap();
            let coord = coord.parse::<Coord>().unwrap();
            folds.push((axis, coord));
        } else if !line.is_empty() {
            let (a, b) = line
                .split_once(',')
                .expect("comma-separated pair of integers");
            let (x, y) = (a.parse::<Coord>().unwrap(), b.parse::<Coord>().unwrap());
            max_x = max_x.max(x + 1);
            max_y = max_y.max(y + 1);
            grid.insert((x, y));
        }
    }

    eprintln!(
        "{} entries in grid of size {} ({} X {})",
        grid.len(),
        max_x * max_y,
        max_x,
        max_y
    );

    /*
    for y in 0..max_y {
        for x in 0..max_x {
            eprint!("{}", if grid.contains(&(x, y)) { '#' } else { '.' });
        }
        eprintln!();
    }
    */

    for fold in &folds {
        //eprintln!("FOLD {:?}", fold);
        if fold.0 == 'y' {
            max_y = fold.1;
            let points = grid
                .iter()
                .filter(|p| p.1 > fold.1)
                .copied()
                .collect::<Vec<_>>();
            for (x, y) in points {
                grid.remove(&(x, y));
                grid.insert((x, fold.1.saturating_sub(y - fold.1)));
            }
        } else {
            assert_eq!(fold.0, 'x');
            max_x = fold.1;
            let points = grid
                .iter()
                .filter(|p| p.0 > fold.1)
                .copied()
                .collect::<Vec<_>>();
            for (x, y) in points {
                grid.remove(&(x, y));
                grid.insert((fold.1.saturating_sub(x - fold.1), y));
            }
        }
    }

    for y in 0..max_y {
        for x in 0..max_x {
            eprint!("{}", if grid.contains(&(x, y)) { '#' } else { '.' });
        }
        eprintln!();
    }

    grid.len()
}

/*
928 entries in grid of size 1170723 (1311 X 893)
###...##..#..#.####.###..####...##..##..
#..#.#..#.#..#....#.#..#.#.......#.#..#.
#..#.#....####...#..###..###.....#.#....
###..#.##.#..#..#...#..#.#.......#.#....
#....#..#.#..#.#....#..#.#....#..#.#..#.
#.....###.#..#.####.###..#.....##...##..
Result: 96
dhat: Total:     99,789 bytes in 98 blocks
dhat: At t-gmax: 35,328 bytes in 3 blocks
dhat: At t-end:  1,024 bytes in 1 blocks

0.00user 0.00system 0:00.00elapsed 93%CPU (0avgtext+0avgdata 2108maxresident)k
0inputs+0outputs (0major+102minor)pagefaults 0swaps

*/
