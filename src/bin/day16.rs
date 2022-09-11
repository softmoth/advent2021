use anyhow::{anyhow, Result};

fn main() -> Result<()> {
    let input = std::fs::read_to_string("inputs/day16.txt")?;
    let bits = BitEater::new(&input)?;
    eprintln!("{:?}", bits);
    Ok(())
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
                        & (u8::try_from(2u16.pow(n.into()) - 1).unwrap() << (8 - self.bit - n)))
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
}
