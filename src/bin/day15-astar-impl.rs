#![deny(clippy::all)]
#![warn(clippy::pedantic)]
//#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]
#![feature(map_first_last)]
#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

use std::collections::{BTreeSet, HashMap};

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    println!(
        "Result: {:?}",
        process(&std::fs::read_to_string("inputs/day15.txt").unwrap())
    );
}

fn process(input: &str) -> u64 {
    let grid = Grid::new(input, 5, 5);
    //eprintln!("{:?}", grid);
    let path = grid
        .astar((0, 0), (grid.width - 1, grid.height - 1))
        .unwrap();

    path.iter()
        .skip(1)
        .map(|&pos| u64::try_from(grid.at(pos)).unwrap())
        .sum::<u64>()
}

type PosScale = u16;
type Pos = (PosScale, PosScale);
type Risk = u8;

type PathCost = u16;

struct Grid {
    data: Vec<Risk>,
    width: PosScale,
    height: PosScale,
    x_repeat: PosScale,
    y_repeat: PosScale,
}

impl Grid {
    fn new(input: &str, x_repeat: PosScale, y_repeat: PosScale) -> Self {
        let width = input.lines().next().unwrap().len();

        let data: Vec<Risk> = input
            .lines()
            .flat_map(|line| {
                assert_eq!(line.len(), width);
                line.chars().map(|c| {
                    // Guaranteed by problem defn & required by part 2
                    assert!(('1'..='9').contains(&c));
                    c.to_digit(10).unwrap().try_into().unwrap()
                })
            })
            .collect();

        let height = data.len() / width;

        Self {
            data,
            width: PosScale::try_from(width).unwrap() * x_repeat,
            height: PosScale::try_from(height).unwrap() * y_repeat,
            x_repeat,
            y_repeat,
        }
    }

    fn at(&self, (x, y): Pos) -> Risk {
        let real_width = self.width / self.x_repeat;
        let real_height = self.height / self.y_repeat;
        let x_shift = Risk::try_from(x / real_width).unwrap();
        let x_rem = x % real_width;
        let y_shift = Risk::try_from(y / real_height).unwrap();
        let y_rem = y % real_height;

        let val = self.data[usize::from(y_rem) * usize::from(real_width) + usize::from(x_rem)];

        // Rotate value from 10 -> 1, 11 -> 2, etc.
        (val + x_shift + y_shift - 1) % 9 + 1
    }

    const fn manhattan((x1, y1): Pos, (x2, y2): Pos) -> PathCost {
        x1.abs_diff(x2) as PathCost + y1.abs_diff(y2) as PathCost
    }

    #[allow(clippy::unnecessary_lazy_evaluations)]
    fn neighbors(&self, (x, y): Pos) -> impl Iterator<Item = Pos> {
        let left = (x > 0).then(|| (x - 1, y));
        let right = (x < self.width - 1).then(|| (x + 1, y));
        let up = (y > 0).then(|| (x, y - 1));
        let down = (y < self.height - 1).then(|| (x, y + 1));
        [left, right, up, down].into_iter().flatten()
    }

    fn reconstruct_path(came_from: &HashMap<Pos, Pos>, goal: Pos) -> Vec<Pos> {
        //eprintln!("Found solution");
        let mut current = goal;
        let mut path: Vec<Pos> = vec![current];
        loop {
            if !came_from.contains_key(&current) {
                break;
            }
            current = came_from[&current];
            path.push(current);
        }
        path.reverse();
        path
    }

    fn astar(&self, start: Pos, end: Pos) -> Option<Vec<Pos>> {
        // Model a priority queue with modifiable priorities as an ordered set of Prio, ID
        // Ideally this would be a binary heap with associated hashmap, according to Wikipedia
        let mut open_prio = BTreeSet::<(PathCost, Pos)>::new();
        let mut open_set = BTreeSet::<Pos>::new();
        let mut f_score = HashMap::<Pos, PathCost>::new();
        let mut g_score = HashMap::<Pos, PathCost>::new();
        let mut came_from = HashMap::<Pos, Pos>::new();

        open_prio.insert((0, start));
        open_set.insert(start);
        g_score.insert(start, 0);
        f_score.insert(start, Self::manhattan(start, end));

        while let Some((_fcost, current)) = open_prio.pop_first() {
            //eprintln!("Current {:?} ${}", current, fcost);

            if current == end {
                return Some(Self::reconstruct_path(&came_from, current));
            }

            open_set.remove(&current);
            for neighbor in self.neighbors(current) {
                let tentative_g_score = g_score[&current] + PathCost::from(self.at(neighbor));

                if g_score.get(&neighbor).map(|&g| g <= tentative_g_score) == Some(true) {
                    continue;
                }

                let old_f_score = f_score.get(&neighbor).copied();

                came_from.insert(neighbor, current);
                f_score
                    .entry(neighbor)
                    .and_modify(|v| *v -= g_score[&neighbor] - tentative_g_score)
                    .or_insert_with(|| tentative_g_score + Self::manhattan(neighbor, end));
                g_score.insert(neighbor, tentative_g_score);

                if let Some(old_f_score) = old_f_score {
                    open_prio.remove(&(old_f_score, neighbor));
                }
                open_prio.insert((f_score[&neighbor], neighbor));
                open_set.insert(neighbor);
            }
        }

        // No path from start to end!
        None
    }
}

impl std::fmt::Debug for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in self.data.chunks(self.width.into()) {
            writeln!(f, "{:?}", row)?;
        }
        Ok(())
    }
}


/*
Part 1:
dhat: Total:     656,953 bytes in 2,136 blocks
dhat: At t-gmax: 314,460 bytes in 75 blocks
dhat: At t-end:  1,024 bytes in 1 blocks

0.00user 0.00system 0:00.00elapsed 100%CPU (0avgtext+0avgdata 2120maxresident)k
0inputs+0outputs (0major+177minor)pagefaults 0swaps


Part 2:
Result: 2893
dhat: Total:     28,346,331 bytes in 51,727 blocks
dhat: At t-gmax: 14,450,404 bytes in 118 blocks
dhat: At t-end:  1,024 bytes in 1 blocks

0.15user 0.00system 0:00.15elapsed 100%CPU (0avgtext+0avgdata 16332maxresident)k
0inputs+0outputs (0major+4893minor)pagefaults 0swaps


Part 2, with 15x15 repeat:

Result: 8393
dhat: Total:     230,140,827 bytes in 462,056 blocks
dhat: At t-gmax: 115,447,908 bytes in 1,029 blocks
dhat: At t-end:  1,024 bytes in 1 blocks

1.50user 0.01system 0:01.51elapsed 100%CPU (0avgtext+0avgdata 115032maxresident)k
0inputs+0outputs (0major+8813minor)pagefaults 0swaps


Part 2, 15x15 repeat, zero heuristic:
dhat: Total:     228,781,523 bytes in 440,370 blocks
dhat: At t-gmax: 115,418,132 bytes in 655 blocks
dhat: At t-end:  1,024 bytes in 1 blocks

1.47user 0.02system 0:01.49elapsed 99%CPU (0avgtext+0avgdata 115052maxresident)k
0inputs+0outputs (0major+8812minor)pagefaults 0swaps


And for fun, a 50x50 repeat:
Result: 28098
22.76user 0.22system 0:22.98elapsed 100%CPU (0avgtext+0avgdata 903384maxresident)k
0inputs+0outputs (0major+142325minor)pagefaults 0swaps

dhat: Total:     1,957,994,827 bytes in 5,152,564 blocks
dhat: At t-gmax: 923,015,340 bytes in 3,083 blocks
dhat: At t-end:  1,024 bytes in 1 blocks

*/
