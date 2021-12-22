use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Default, Debug)]
struct Dozen([u16; 12]);

impl Dozen {
    fn add(&self, oth: &Dozen, val: u16) -> Self {
        let mut out = [0; 12];
        for (i, cur) in out.iter_mut().enumerate() {
            *cur = if oth.0[i] == val {
                self.0[i] + 1
            } else {
                self.0[i]
            };
        }
        Dozen(out)
    }

    fn bin_to_dec(&self) -> u32 {
        u16::from_str_radix(&self.to_string(), 2).unwrap() as u32
    }
}

impl Display for Dozen {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let arr = self.0.map(|x| if x == 0 { "0" } else { "1" });
        f.write_str(arr.concat().as_str())
    }
}

impl FromStr for Dozen {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut d = [0; 12];
        for (i, c) in s.chars().enumerate() {
            if i > 11 {
                return Err(format!("too many chars in input: {}", s));
            }
            if c == '0' {
                d[i] = 0;
            } else if c == '1' {
                d[i] = 1;
            } else {
                return Err(format!("invalid char found in input {}", c));
            }
        }
        Ok(Dozen(d))
    }
}

#[derive(Default, Debug)]
struct FoldBits {
    eps: Dozen,
    gam: Dozen,
}

impl FoldBits {
    fn fold(f: Self, next: &Dozen) -> Self {
        Self {
            eps: f.eps.add(next, 0),
            gam: f.gam.add(next, 1),
        }
    }

    fn express(&self) -> Self {
        let mut gam_rate = [0; 12];
        let mut eps_rate = [0; 12];
        for i in 0..12 {
            if self.gam.0[i] > self.eps.0[i] {
                gam_rate[i] = 1;
                eps_rate[i] = 0;
            } else {
                gam_rate[i] = 0;
                eps_rate[i] = 1;
            }
        }

        Self {
            gam: Dozen(gam_rate),
            eps: Dozen(eps_rate),
        }
    }
}

fn main() {
    let lines: Vec<Dozen> = include_str!("../input.txt")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|l| l.parse::<Dozen>().unwrap())
        .collect();

    let counts = lines.iter().fold(FoldBits::default(), FoldBits::fold);
    let counted = counts.express();

    println!(
        "counts product: {}",
        counted.gam.bin_to_dec() * counted.eps.bin_to_dec()
    );
}
