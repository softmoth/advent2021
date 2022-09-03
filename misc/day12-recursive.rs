#![deny(clippy::all)]
#![warn(clippy::pedantic)]
//#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]

use apply::Also;
use std::collections::HashSet;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let edges = DATA
        .lines()
        .flat_map(|line| {
            let (a, b) = line.split_once('-').unwrap();
            [(a, b), (b, a)]
        })
        .collect::<Vec<(&str, &str)>>();

    //edges.chunks(2).for_each(|pair| println!("{:?}", pair));

    let solutions = walk(&edges, "start", &[]);

    /*
    solutions.sort();
    for sol in &solutions {
        eprintln!("{:?}", &sol);
    }
    */

    println!("Answer = {}", solutions.len());
}

fn walk<'a, 'b>(
    edges: &'b [(&'a str, &'a str)],
    cave: &'a str,
    path: &[&'a str],
) -> Vec<Vec<&'a str>> {
    let path = path.to_vec().also(|p| p.push(cave));
    assert!(path.len() < 100); // Protect against mistaken recursion

    if cave == "end" {
        return vec![path];
    }

    if !cave.chars().next().unwrap().is_ascii_uppercase() {
        if cave == "start" && path.len() > 1 {
            // No double-visits to starting room
            return vec![];
        }

        let mut seen = HashSet::new();
        let mut have_double_room = false;
        for cave in &path {
            if cave.chars().next().unwrap().is_ascii_uppercase() {
                // Don't weed out any big rooms
                continue;
            }

            if seen.contains(cave) {
                if have_double_room {
                    // This path has too many small room visits; either
                    // - another room has already been visited twice, or
                    // - this room has already been visited twice.
                    // Both of those mean this visit puts us over the limit.
                    return vec![];
                }

                have_double_room = true;
            } else {
                seen.insert(cave);
            }
        }
    }

    // Still here? Good, let's walk this node's exits
    edges
        .iter()
        .filter_map(|(a, b)| if *a == cave { Some(*b) } else { None })
        .flat_map(|exit| walk(edges, exit, &path))
        .collect()
}

const DATA: &str = "\
XW-ed
cc-tk
eq-ed
ns-eq
cc-ed
LA-kl
II-tk
LA-end
end-II
SQ-kl
cc-kl
XW-eq
ed-LA
XW-tk
cc-II
tk-LA
eq-II
SQ-start
LA-start
XW-end
ed-tk
eq-JR
start-kl
ed-II
SQ-tk
";
