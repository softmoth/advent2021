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
    //eprintln!("{:?}", rules);
    //eprintln!("{:?}", template);
    let mut polymer = init_polymer(&template);
    for _step in 0..40 {
        polymer = apply_rules(&polymer, &rules);
        score_polymer(&template, &polymer);
    }

    score_polymer(&template, &polymer)
}

fn score_polymer(template: &str, polymer: &Polymer) -> usize {
    let mut elements = HashMap::<char, usize>::new();

    // This double-counts (almost) every element
    for (&[a, b], n) in polymer {
        *elements.entry(a).or_default() += n;
        *elements.entry(b).or_default() += n;
    }

    // The first and last element in the template weren't counted double; fix that
    assert!(template.len() > 1);
    let mut template = template.chars();
    let (first, last) = (template.next().unwrap(), template.last().unwrap());
    elements.entry(first).and_modify(|v| *v += 1);
    elements.entry(last).and_modify(|v| *v += 1);

    // Get the un-doubled counts, sorted
    let mut elements = elements.iter().map(|(k, v)| (k, v / 2)).collect::<Vec<_>>();
    elements.sort_by_key(|e| e.1);

    //eprintln!("{:?}", elements);
    elements[elements.len() - 1].1 - elements[0].1
}

type Polymer = HashMap<[char; 2], usize>;
type Rules = HashMap<[char; 2], char>;

fn init_polymer(template: &str) -> Polymer {
    let mut polymer: Polymer = HashMap::new();
    for w in template.chars().collect::<Vec<_>>().windows(2) {
        *polymer.entry([w[0], w[1]]).or_default() += 1;
    }
    polymer
}

fn apply_rules(polymer: &Polymer, rules: &Rules) -> Polymer {
    let mut polymer_new = HashMap::new();
    polymer
        .iter()
        .flat_map(|(&pair, &count)| {
            rules
                .get(&pair)
                .map_or([(pair, count), (pair, 0)], |&insert| {
                    [([pair[0], insert], count), ([insert, pair[1]], count)]
                })
        })
        .for_each(|(pair, count)| *polymer_new.entry(pair).or_default() += count);
    //eprintln!("{:?}", &polymer_new);
    polymer_new
}

fn parse(input: &str) -> (String, Rules) {
    let mut lines = input.lines();

    let template: String = lines.next().unwrap().to_owned();
    assert!(template.len() > 1);

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

Part 2:
Result: 4302675529689
dhat: Total:     205,367 bytes in 419 blocks
dhat: At t-gmax: 8,010 bytes in 6 blocks
dhat: At t-end:  1,024 bytes in 1 blocks

0.00user 0.00system 0:00.00elapsed 0%CPU (0avgtext+0avgdata 2100maxresident)k
0inputs+0outputs (0major+95minor)pagefaults 0swaps

*/
