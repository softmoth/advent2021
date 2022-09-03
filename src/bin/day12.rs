#![deny(clippy::all)]
#![warn(clippy::pedantic)]
//#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]

use std::collections::{HashMap, HashSet};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    println!(
        "Result: {:?}",
        process(&std::fs::read_to_string("inputs/day12.txt").unwrap())
    );
}

fn process(input: &str) -> usize {
    let caves = Caves::new(input);
    //dbg!(&caves);
    let paths = caves.exhaust("start", "end");
    paths.len()
}

type CaveID = u8;

#[derive(Debug)]
struct Caves {
    by_name: HashMap<String, CaveID>,
    data: Vec<Cave>,
    start_id: CaveID,
}

impl Caves {
    fn new(input: &str) -> Self {
        let mut caves = Self {
            by_name: HashMap::new(),
            data: vec![],
            start_id: 0,
        };
        for line in input.lines() {
            let (a, b) = line.split_once('-').unwrap();
            caves.connect(a, b);
        }
        caves.start_id = *caves.by_name.get("start").unwrap();

        caves
    }

    fn get_cave_id(&mut self, name: &str) -> CaveID {
        *self.by_name.entry(name.to_owned()).or_insert_with(|| {
            let id = self.data.len();
            let id = id
                .try_into()
                .expect("Too many caves to fit in u8, at '{name}'");
            self.data.push(Cave {
                id,
                name: name.to_owned(),
                is_small: name.chars().next().unwrap().is_ascii_lowercase(),
                exits: Vec::new(),
            });
            id
        })
    }

    fn connect(&mut self, a: &str, b: &str) {
        let a = self.get_cave_id(a);
        let b = self.get_cave_id(b);
        self.data[a as usize].exits.push(b);
        self.data[b as usize].exits.push(a);
    }

    fn walk(&self, cave: CaveID, mut path: Vec<CaveID>, goal: CaveID) -> Vec<Vec<CaveID>> {
        // Safeguard against recursion mistake
        if path.len() > 1_000 {
            unreachable!("Gah! Recursion busted! {path:?}");
        }

        path.push(cave);

        if cave == goal {
            // Found the end, no need to check the exits
            return vec![path];
        }

        if self.data[cave as usize].is_small {
            /* PART 1
            // Don't re-enter a small room
            if path.contains(&exit) {
                continue;
            }
            */

            // Never return to the starting room
            if cave == self.start_id && path.len() > 1 {
                return vec![];
            }

            // Ensure the path isn't revisiting too many small rooms
            let mut got_a_double = false;
            let mut seen = HashSet::<CaveID>::new();
            for &p in &path {
                if self.data[p as usize].is_small {
                    if seen.contains(&p) {
                        if got_a_double {
                            // Already had a double visit; bail
                            return vec![];
                        }

                        // This is the first double-visit
                        got_a_double = true;
                    } else {
                        // First appearance of this small room
                        seen.insert(p);
                    }
                }
            }
        }

        // Still here? OK, then walk cave's exits
        self.data[cave as usize]
            .exits
            .iter()
            .flat_map(|&e| self.walk(e, path.clone(), goal))
            .collect()
    }

    fn exhaust(&self, start: &str, end: &str) -> Vec<Vec<CaveID>> {
        // TODO Maybe put walk on an iterator struct, and store start & end there?
        assert_eq!(start, "start");
        assert_eq!(end, "end");

        let start = *self.by_name.get(start).unwrap();
        let end = *self.by_name.get(end).unwrap();

        self.walk(start, vec![], end)
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Cave {
    id: CaveID,
    name: String,
    is_small: bool,
    exits: Vec<CaveID>,
}

/*
Perl solution:
107395
2.86user 0.03system 0:02.90elapsed 99%CPU (0avgtext+0avgdata 112984maxresident)k
0inputs+0outputs (0major+27031minor)pagefaults 0swaps


Rust, recursive and mutable reference to solutions vec:
Answer = 107395
dhat: Total:     544,283,056 bytes in 2,819,624 blocks
dhat: At t-gmax: 42,800,364 bytes in 107,411 blocks
dhat: At t-end:  1,024 bytes in 1 blocks

NOTE: Timing is much slower with dhat, because of the many allocations!

Timing with dhat-heap:
5.45user 5.70system 0:11.15elapsed 100%CPU (0avgtext+0avgdata 19056maxresident)k
0inputs+120outputs (0major+4830minor)pagefaults 0swaps

And without:
0.32user 0.01system 0:00.33elapsed 99%CPU (0avgtext+0avgdata 45900maxresident)k
0inputs+0outputs (0major+11074minor)pagefaults 0swaps

Recursive, with return .flat_map(walk).collect():
dhat: Total:     615,999,280 bytes in 3,123,800 blocks
dhat: At t-gmax: 44,696,032 bytes in 107,399 blocks
dhat: At t-end:  1,024 bytes in 1 blocks

Answer = 107395
0.31user 0.00system 0:00.32elapsed 99%CPU (0avgtext+0avgdata 47508maxresident)k
0inputs+0outputs (0major+12268minor)pagefaults 0swaps

So, memory usage did increase using collect() rather than a mut ref, but it doesn't affect the run
time significantly.


Using u8 IDs instead of &str names (still recursive w/ collect()):
Result: 107395
dhat: Total:     149,104,236 bytes in 3,123,879 blocks
dhat: At t-gmax: 7,523,660 bytes in 107,440 blocks
dhat: At t-end:  1,024 bytes in 1 blocks

0.26user 0.00system 0:00.26elapsed 100%CPU (0avgtext+0avgdata 11036maxresident)k
0inputs+0outputs (0major+3012minor)pagefaults 0swaps

*/
