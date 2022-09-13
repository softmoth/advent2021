//#![feature(trait_alias)]

pub use anyhow::{anyhow, Result};

type Pos = (i32, i32);

#[derive(PartialEq, Eq, Debug, Clone)]
pub struct Target {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

impl Target {
    fn hit_by_launch(&self, (vx0, vy0): (i32, i32)) -> bool {
        for step in 1.. {
            // X velocity never goes negative; after vx0 steps, velocity remains 0
            let kx = step.min(vx0);
            let ky = step;

            let x = kx * vx0 - kx * (kx - 1) / 2;
            let y = ky * vy0 - ky * (ky - 1) / 2;

            if (self.x1..=self.x2).contains(&x) && (self.y1..=self.y2).contains(&y) {
                //println!("HIT the target at step #{step}, pos {:?}", (x, y));
                return true;
            }

            if y < self.y1 || x > self.x2 || step >= vx0 && x < self.x1 {
                //println!("Missed the target at step #{step}, pos {:?}", (x, y));
                return false;
            }
        }
        unreachable!()
    }
}

peg::parser! {
    grammar target_parser() for str {
        rule hspace() -> ()
            = [' ' | '\t'] {}
        rule _ -> ()
            = quiet!{hspace()*} {}

        rule __() -> ()
            = quiet!{hspace()+} {}

        rule sign() -> i32
            = sign:['-' | '+']? {
                if let Some('-') = sign { -1 } else { 1 }
            }
        rule integer() -> i32
            = sign:sign() num:['0'..='9']+ {
                let num = num
                    .into_iter()
                    .collect::<String>()
                    .parse::<i32>()
                    .unwrap();

                num * sign
            }

        // Is there a more straightforward way to match a parameter?
        // Is there a way to get the expected character into the error message &str?
        rule axis_label(axis: char)
            = [c] {?
                (c == axis).then_some(()).ok_or("axis label")
            }

        rule target_dim(axis: char) -> Pos
            =  axis_label(axis) _ "=" _ from:integer() _ ".." _ to:integer() {
                (from, to)
            }

        rule eol()
            = quiet!{"\r"? "\n"} / expected!("EOL")

        pub rule target() -> Target
            = "target" __ "area" _ ":" _
                x:target_dim('x') _ "," _
                y:target_dim('y') _ eol()?
            {
                assert!(x.0 <= x.1);
                assert!(y.0 <= y.1);

                Target {
                    x1: x.0,
                    x2: x.1,
                    y1: y.0,
                    y2: y.1,
                }
            }
    }
}

// x0 = 0                           y0 = 0
// x1 = n                           y1 = n
// x2 = n + n-1 = 2n - 1            y2 = n + n-1
// x3 = n + n-1 + n-2 = 3n - 3      ...
// xk = k'n - (k' * (k'-1) / 2)     yk = kn - (k * (k-1) / 2)
//   : k' = min(k, n)

// Assuming kx == vx0 (i.e., the step at which vx becomes zero):
// targetx <= vx0 * vx0 - vx0 * (vx0 - 1) / 2
// targetx - vx0^2 + (vx0 * (vx0 - 1) / 2) <= 0
// targetx - vx0^2 + vx0^2 / 2 - vx0 / 2 <= 0
// -0.5vx0^2 + -0.5vx0 + targetx <= 0

fn quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let discr = b * b - 4.0 * a * c;
    (discr >= 0.0).then_some((
        (-b + discr.sqrt()) / (2.0 * a),
        (-b - discr.sqrt()) / (2.0 * a),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() -> Result<()> {
        process("target area: x=20..30, y=-10..-5")
    }
}

fn main() -> Result<()> {
    process(&std::fs::read_to_string("inputs/day17.txt")?)
}

fn process(input: &str) -> Result<()> {
    let target = target_parser::target(input)?;
    eprintln!("{:?}", &target);

    // The crucial insight for Part 1 is sorting out how y evolves:
    // - It has initial y velocity vy0 at step = 0, y == 0
    // - It decreases by 1 each step, so after vy0 steps it has velocity 0
    //     - y == 0 + 1 + 2 + .. + (vy0 - 1) == vy0 * (vy0 - 1) / 2
    // - It continues to decrease by 1 each step, so when it reaches y == 0
    //   again, velocity will be -vy0
    // - On the next step, y == -vy0. *This* is the key to solving Part 1.
    //   Put -vy0 at the bottom of the target area, and it's done.

    let max_vy0 = -target.y1;
    // Gauss sum formula for 1 + 2 + ... + n
    let best_y_height = max_vy0 * (max_vy0 - 1) / 2;
    dbg!(&target, &max_vy0, &best_y_height);

    let (qa, qb) = quadratic(-0.5, -0.5, target.x1.into())
        .ok_or_else(|| anyhow!("Impossible no such parabola"))?;
    let min_vx0: i32 = qa.max(qb).ceil() as i32 /* .try_into()? */;

    // Or, you can shoot straight at the target in one step
    let min_vy0 = target.y1;
    let max_vx0 = target.x2;

    // Just brute force all combos
    use itertools::Itertools;
    let hits = (min_vx0..=max_vx0).cartesian_product(min_vy0..=max_vy0)
        .filter(|velocity| target.hit_by_launch(*velocity));

    dbg!(hits.count());

    Ok(())
}
