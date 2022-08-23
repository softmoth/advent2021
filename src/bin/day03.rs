#![deny(clippy::all)]
#![warn(clippy::pedantic)]
//#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]

use std::collections::BTreeSet;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let diags = std::io::stdin()
        .lines()
        //
        .map(|res| u16::from_str_radix(&res.unwrap(), 2).unwrap())
        .collect::<BTreeSet<_>>();
    //dbg!(diags);

    let max_bits = f32::from(*diags.iter().max().unwrap()).log2().floor() as u16 + 1;

    let is_oxygen = |diags: &BTreeSet<u16>, bit: u16| {
        let bit = 1 << bit;
        let set = diags.iter().filter(|v| *v & bit > 0).count();
        let want_set = set >= diags.len() - set;

        move |v: &u16| (*v & bit > 0) == want_set
    };

    // Split diagnostics based on first bit
    let part_func = is_oxygen(&diags, max_bits - 1);
    let (mut oxygen_generator, mut co2_scrubber): (BTreeSet<_>, BTreeSet<_>) =
        diags.into_iter().partition(part_func);
    dbg!(&oxygen_generator);
    dbg!(&co2_scrubber);

    let mut bit = max_bits - 1;
    while oxygen_generator.len() > 1 {
        bit -= 1;
        let filter = is_oxygen(&oxygen_generator, bit);
        oxygen_generator.retain(filter);
    }

    let mut bit = max_bits - 1;
    while co2_scrubber.len() > 1 {
        bit -= 1;
        let filter = is_oxygen(&co2_scrubber, bit);
        co2_scrubber.retain(|v| !filter(v));
    }

    dbg!(&oxygen_generator);
    dbg!(&co2_scrubber);

    println!(
        "Answer = {}",
        *oxygen_generator.iter().next().unwrap() as u32
            * *co2_scrubber.iter().next().unwrap() as u32
    );
}
