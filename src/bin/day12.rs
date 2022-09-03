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
    dbg!(&caves);
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
                is_big: name.chars().next().unwrap().is_ascii_uppercase(),
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
        assert!(path.len() < 1_000);

        path.push(cave);

        if cave == goal {
            // Found the end, no need to check the exits
            return vec![path];
        }

        if self.data[cave as usize].is_big {
            // Don't follow the same edge in the same direction twice
            if path[..path.len() - 1]
                .windows(2)
                .any(|w| w == [*path.last().unwrap(), cave])
            {
                return vec![];
            }
        } else {
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
                if !self.data[p as usize].is_big {
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
    is_big: bool,
    exits: Vec<CaveID>,
}

/*
Perl solution, part 2:
107395
2.86user 0.03system 0:02.90elapsed 99%CPU (0avgtext+0avgdata 112984maxresident)k
0inputs+0outputs (0major+27031minor)pagefaults 0swaps

*/
