#![deny(clippy::all)]
#![warn(clippy::pedantic)]
//#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
//#![warn(clippy::cargo)]
#![feature(box_syntax)]

#[allow(unused_imports)]
use anyhow::{anyhow, bail, Result};

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() -> Result<()> {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();
    for line in std::fs::read_to_string("inputs/day18.txt")?.lines() {
        let n = line.parse::<SNum>()?;
        eprintln!("{n:?}");
    }
    Ok(())
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct SNum(u64);

struct SNumNybbles<'a> {
    n: &'a SNum,
    pos: u8,
}

impl<'a> Iterator for SNumNybbles<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= 16 {
            return None;
        }

        let shift = 4 * (16 - 1 - self.pos);
        self.pos += 1;

        ((self.n.0 & (0xF << shift)) >> shift).try_into().ok()
    }
}

impl std::fmt::Display for SNum {
    #[allow(clippy::identity_op, clippy::erasing_op)]
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        fn level_for_position(pos: usize) -> usize {
            usize::try_from(pos.trailing_zeros()).unwrap()
        }

        let mut from = 0usize; // Beginning of current slot (inclusive)

        for (i, val) in self.nybbles().enumerate() {
            if val < FLAG {
                let this_lvl = level_for_position(i + 1);
                let from_lvl = if from == 0 {
                    usize::from(Self::num_levels())
                } else {
                    level_for_position(from)
                };

                if from > 0 {
                    write!(f, ",")?;
                }

                for c in std::iter::repeat('[').take(from_lvl.saturating_sub(this_lvl)) {
                    write!(f, "{c}")?;
                }

                write!(f, "{}", &val)?;

                for c in std::iter::repeat(']').take(this_lvl.saturating_sub(from_lvl)) {
                    write!(f, "{c}")?;
                }

                from = i + 1;
            }
        }

        for c in std::iter::repeat(']').take(4usize.saturating_sub(level_for_position(from))) {
            write!(f, "{c}")?;
        }

        Ok(())
    }
}

impl std::str::FromStr for SNum {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self> {
        let mut n = Self(0);

        // Current nesting level
        let mut level = 0;

        // Current position, goes from 0..=15, left to right
        let mut logical_slot = 0;

        for c in s.chars() {
            match c {
                '[' => level += 1,
                ']' => level -= 1,
                '0'..='9' => {
                    assert!(level <= Self::num_levels());
                    let leveled_slot = (Self::max_slot(Self::num_levels()) - logical_slot)
                        / (1 << (Self::num_levels() - level));
                    //dbg!(&logical_slot, &leveled_slot);
                    n.set(
                        level,
                        leveled_slot,
                        c.to_digit(10)
                            .ok_or_else(|| anyhow!("Impossible non-digit {c}"))?
                            .try_into()
                            .unwrap(),
                    );
                    logical_slot += 1 << (Self::num_levels() - level);
                }
                ',' | ' ' | '\n' => {} // Ignore commas
                _ => bail!(anyhow!("Unexpected character {c}")),
            }
        }
        assert!(level == 0);
        Ok(n)
    }
}

// Numbers from 0..=9, or FLAG if covered by lower level slot
const BITS_FOR_VAL: u8 = 4;
const FLAG: u8 = 0xF;
impl SNum {
    // There are 4 levels
    const fn num_levels() -> u8 {
        4
    }

    // There are 2^level slots per level
    const fn max_slot(level: u8) -> u8 {
        1 << level
    }

    // A level 1 slot takes up 32 bits; level 2 = 16, level 3 = 8, level 4 = 4
    // There is no level 0, bare number; the top level is always a pair
    const fn bits_for_level(level: u8) -> u8 {
        1 << (6 - level)
    }


    // Textual representation has slots in right-to-left order; i.e. "left" is higher slot
    // Level 1: [2, 1]
    // Level 2: [[4, 3], [2, 1]]
    // Level 3: [[[8, 7], [6, 5]], [[4, 3], [2, 1]]]
    // Level 4: [[[[16, 15], [14, 13]], [[12, 11], [10, 9]]], [[[8, 7] ... [2, 1]]]]
    fn set(&mut self, level: u8, slot: u8, val: u8) {
        assert!((1..=Self::num_levels()).contains(&level));
        assert!((1..=Self::max_slot(level)).contains(&slot));

        let bits_for_level = Self::bits_for_level(level);

        // A slot is simply shifted over by its index
        let shift_for_slot = bits_for_level * (slot - 1);

        // A slot full of 1s, except the right-most BITS_FOR_VAL are 0
        let slot_mask = ((1 << bits_for_level) - 1) << shift_for_slot;
        let flag_mask = (((1 << bits_for_level) - 1) >> BITS_FOR_VAL) << BITS_FOR_VAL;

        self.0 = (self.0 & !slot_mask) | ((flag_mask | u64::from(val)) << shift_for_slot);
    }

    const fn nybbles(&self) -> SNumNybbles {
        SNumNybbles { n: self, pos: 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_trip(input: &str) -> Result<()> {
        let snum = input.parse::<SNum>()?;
        let s = snum.to_string();
        eprintln!("·{s}");
        assert_eq!(&s, input);
        Ok(())
    }

    #[test]
    fn parsing1() -> Result<()> {
        round_trip("[1,2]")?;
        Ok(())
    }
    #[test]
    fn parsing2() -> Result<()> {
        round_trip("[[1,2],3]")?;
        Ok(())
    }
    #[test]
    fn parsing3() -> Result<()> {
        round_trip("[9,[8,7]]")?;
        Ok(())
    }
    #[test]
    fn parsing4() -> Result<()> {
        round_trip("[[1,9],[8,5]]")?;
        Ok(())
    }
    #[test]
    fn parsing5() -> Result<()> {
        //#[cfg(feature = "dhat-heap")]
        //let _profiler = dhat::Profiler::new_heap();
        round_trip("[[[[1,2],[3,4]],[[5,6],[7,8]]],9]")?;
        Ok(())
    }
    #[test]
    fn parsing6() -> Result<()> {
        round_trip("[[[9,[3,8]],[[0,9],6]],[[[3,7],[4,9]],3]]")?;
        Ok(())
    }
    #[test]
    fn parsing7() -> Result<()> {
        round_trip("[[[[1,3],[5,3]],[[1,3],[8,7]]],[[[4,9],[6,9]],[[8,2],[7,3]]]]")?;
        Ok(())
    }
    #[test]
    fn parsing8() -> Result<()> {
        for line in "\
[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
"
        .lines()
        {
            round_trip(line)?;
        }
        Ok(())
    }
}
