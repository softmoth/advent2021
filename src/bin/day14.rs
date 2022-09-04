#![deny(clippy::all)]
#![warn(clippy::pedantic)]
//#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]

use std::collections::HashMap;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    println!(
        "Result: {:?}",
        process(&std::fs::read_to_string("inputs/day14.txt").unwrap())
    );
}

fn process(input: &str) -> usize {
    let (template, rules) = parse(input);
    eprintln!("{:?}", rules);
    eprintln!("{:?}", template);
    let mut polymer = template;
    for _step in 0..10 {
        polymer = apply_rules(&polymer, &rules);
        //eprintln!("{:?}", polymer);
    }

    score_polymer(&polymer)
}

fn score_polymer(polymer: &Polymer) -> usize {
    let mut elements = HashMap::<char, usize>::new();
    for &p in polymer {
        *elements.entry(p).or_default() += 1;
    }

    let mut elements = elements.iter().collect::<Vec<_>>();
    elements.sort_by_key(|e| e.1);

    elements[elements.len() - 1].1 - elements[0].1
}

type Polymer = Vec<char>;
type Rules = HashMap<[char; 2], char>;

fn apply_rules(polymer: &Polymer, rules: &Rules) -> Polymer {
    polymer
        .windows(2)
        .flat_map(|w| [Some(w[0]), rules.get(w).copied()])
        .chain(Some(polymer.last().copied()))
        .flatten()
        .collect()
}

fn parse(input: &str) -> (Polymer, Rules) {
    let mut lines = input.lines();

    let template: Polymer = lines.next().unwrap().chars().collect();
    assert!(!template.is_empty());

    let blank = lines.next().unwrap();
    assert!(blank.is_empty());

    let rules = lines
        .map(|line| {
            let (pair, insert) = line.split_once(" -> ").unwrap();
            let mut pair = pair.chars();
            let pair = [pair.next().unwrap(), pair.next().unwrap()];
            let insert = insert.chars().next().unwrap();

            (pair, insert)
        })
        .collect::<Rules>();

    (template, rules)
}

/*
Part 1:
Result: 3284
dhat: Total:     529,703 bytes in 111 blocks
dhat: At t-gmax: 199,110 bytes in 4 blocks
dhat: At t-end:  1,024 bytes in 1 blocks

0.00user 0.00system 0:00.00elapsed 100%CPU (0avgtext+0avgdata 2128maxresident)k
0inputs+0outputs (0major+138minor)pagefaults 0swaps

*/
