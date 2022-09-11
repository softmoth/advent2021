use anyhow::{anyhow, bail, Result};
use strum_macros::FromRepr;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day16.txt")?;
    let mut bits = BitEater::new(&input)?;
    let packet = get_packet(&mut bits)?;
    println!("Packet value: {}", packet.value());
    println!("Version sum: {}", packet.version_sum());
    Ok(())
}

#[derive(Debug, Clone, Copy, FromRepr)]
enum Op {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan = 5,
    LessThan,
    EqualTo,
}

type Version = u8;
enum Packet {
    Value(Version, u64),
    Operator(Version, Op, Vec<Packet>),
}

impl Packet {
    fn value(&self) -> u64 {
        match self {
            Packet::Value(_, v) => *v,
            Packet::Operator(_, Op::Sum, vs) => vs.iter().map(|v| v.value()).sum(),
            Packet::Operator(_, Op::Product, vs) => vs.iter().map(|v| v.value()).product(),
            Packet::Operator(_, Op::Minimum, vs) => vs.iter().map(|v| v.value()).min().unwrap(),
            Packet::Operator(_, Op::Maximum, vs) => vs.iter().map(|v| v.value()).max().unwrap(),
            Packet::Operator(_, Op::GreaterThan, vs) => {
                if vs[0].value() > vs[1].value() {
                    1
                } else {
                    0
                }
            }
            Packet::Operator(_, Op::LessThan, vs) => {
                if vs[0].value() < vs[1].value() {
                    1
                } else {
                    0
                }
            }
            Packet::Operator(_, Op::EqualTo, vs) => {
                if vs[0].value() == vs[1].value() {
                    1
                } else {
                    0
                }
            }
        }
    }

    fn version_sum(&self) -> u64 {
        match self {
            Packet::Value(v, _) => u64::from(*v),
            Packet::Operator(v, _, vs) => {
                u64::from(*v) + vs.iter().map(|v| v.version_sum()).sum::<u64>()
            }
        }
    }
}

fn get_packet(bits: &mut BitEater) -> Result<Packet> {
    let vrs = u8::try_from(bits.eat(3)?).unwrap();
    let typ = bits.eat(3)?;
    match dbg!(typ) {
        4 => Ok(Packet::Value(vrs, get_value(bits)?)),
        (0..=7) => Ok(Packet::Operator(
            vrs,
            dbg!(Op::from_repr(usize::from(typ)).unwrap()),
            get_subpackets(bits)?,
        )),
        _ => bail!("Invalid packet type {typ}"),
    }
}

fn get_value(bits: &mut BitEater) -> Result<u64> {
    let mut val = 0;
    loop {
        let go_on = bits.eat(1)?;
        val = val << 4 | u64::from(bits.eat(4)?);
        if go_on == 0 {
            break;
        }
    }
    Ok(val)
}

fn get_subpackets(bits: &mut BitEater) -> Result<Vec<Packet>> {
    let mut res = vec![];
    let length_id = bits.eat(1)?;
    if length_id == 0 {
        let length = bits.eat(15)?;
        let end = bits.position() + usize::from(length);
        while bits.position() < end {
            res.push(get_packet(bits)?);
        }
    } else {
        let length = bits.eat(11)?;
        for _ in 0..length {
            res.push(get_packet(bits)?);
        }
    }

    Ok(res)
}

struct BitEater {
    // The raw data
    data: Vec<u8>,
    byte: usize,
    bit: u8,
}

impl std::fmt::Debug for BitEater {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "+{},{}:", self.byte, self.bit)?;
        for byte in &self.data {
            write!(f, " {:08b}", byte)?;
        }
        writeln!(f, "\t{:?}", &self.data)
    }
}

fn hex_pair(bytes: &[u8]) -> Result<u8> {
    let (high, low) = (bytes[0], bytes[1]);
    char::from(high)
        .to_digit(16)
        .and_then(|high| char::from(low).to_digit(16).map(|low| (high, low)))
        .ok_or_else(|| anyhow!("not hex input {:?}", bytes))
        .and_then(|(high, low)| {
            u8::try_from((high << 4) | low)
                .map_err(|e| anyhow!("Impossible u8::from({high},{low}): {e}"))
        })
}

impl BitEater {
    // Turn a string of 2n hex digits into a Vec<u8> of length n
    fn new(input: &str) -> Result<Self> {
        let chunks = input.trim_end().as_bytes().chunks_exact(2);
        let last_one = chunks.remainder().first().map(|b| hex_pair(&[*b, b'0']));
        let data = chunks
            .map(hex_pair)
            .chain(last_one)
            .collect::<Result<Vec<u8>>>()?;

        Ok(BitEater {
            data,
            byte: 0,
            bit: 0,
        })
    }

    fn position(&self) -> usize {
        self.byte * 8 + usize::from(self.bit)
    }

    // Convert the next count bits into a u16 and return them
    fn eat(&mut self, count: usize) -> Result<u16> {
        if count > 16 {
            return Err(anyhow!("Can't eat {count} bits at a time, 16 max!"));
        }

        if count > (self.data.len() - self.byte) * 8 - usize::from(self.bit) {
            return Err(anyhow!("Not {count} bits remaining in {self:?}"));
        }

        let mut res = 0u16;
        let mut rem = count;
        while rem > 0 {
            let n = u8::try_from(rem.min(usize::from(8 - self.bit))).unwrap();
            debug_assert!((1..=8).contains(&n));

            res = (res << n)
                | u16::from(
                    (self.data[self.byte]
                        & (u8::try_from((1u16 << n) - 1).unwrap() << (8 - self.bit - n)))
                        >> (8 - self.bit - n),
                );

            self.bit += n;
            debug_assert!(self.bit <= 8);
            if self.bit == 8 {
                self.byte += 1;
                self.bit = 0;
            }

            rem -= usize::from(n);
        }

        Ok(res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn biteater_new() -> Result<()> {
        let mut bits = BitEater::new("D2FE28")?;
        eprintln!("{:?}", bits);
        assert_eq!(bits.eat(3)?, 6);
        assert_eq!(bits.eat(3)?, 4);
        assert_eq!(bits.eat(3)?, 5);
        assert_eq!(bits.eat(7)?, 126);
        assert_eq!(bits.eat(8)?, 40);
        assert!(bits.eat(1).is_err());
        Ok(())
    }

    #[test]
    fn simple_number() -> Result<()> {
        // Example 1 from Part 1
        let mut bits = BitEater::new("D2FE28")?;
        if let Packet::Value(vrs, val) = get_packet(&mut bits)? {
            assert_eq!(vrs, 6);
            assert_eq!(val, 2021);
        } else {
            bail!("Did not get a Value packet");
        }
        Ok(())
    }

    #[test]
    fn two_subpackets() -> Result<()> {
        // Example 2 from Part 1
        let mut bits = BitEater::new("38006F45291200")?;
        if let ref packet @ Packet::Operator(vrs, _op, ref val) = get_packet(&mut bits)? {
            assert_eq!(vrs, 1);
            assert_eq!(val.len(), 2);
            assert_eq!(val[0].value(), 10);
            assert_eq!(val[1].value(), 20);
            assert_eq!(packet.value(), 1);
        } else {
            bail!("Did not get a Op packet");
        }
        Ok(())
    }
}
