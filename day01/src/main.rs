#[derive(Default)]
struct FoldOne {
    prev: u32, // 0 default causes extra check
    count: u32,
}

impl FoldOne {
    fn fold(f: Self, x: u32) -> Self {
        Self {
            prev: x,
            count: if f.prev != 0 && x > f.prev {
                f.count + 1
            } else {
                f.count
            },
        }
    }
}

//           oldest, old, newest
struct Triad(u32, u32, u32);

impl Triad {
    fn sum(&self) -> u32 {
        self.0 + self.1 + self.2
    }
    fn shift(&self, next: u32) -> Self {
        Self(self.1, self.2, next)
    }
}

// we'd be in trouble if there are less than 3 items in the list
enum FoldThree {
    Init,
    First(u32),
    Second(u32, u32),
    Ready { prev: Triad, count: u32 },
}

impl FoldThree {
    fn fold(f: Self, next: u32) -> Self {
        match f {
            FoldThree::Init => FoldThree::First(next),
            FoldThree::First(p) => FoldThree::Second(p, next),
            FoldThree::Second(oldest, old) => FoldThree::Ready {
                prev: Triad(oldest, old, next),
                count: 0,
            },
            FoldThree::Ready { prev, count } => {
                let shifted = prev.shift(next);
                let count = if prev.sum() < shifted.sum() {
                    count + 1
                } else {
                    count
                };
                FoldThree::Ready {
                    prev: shifted,
                    count,
                }
            }
        }
    }
    fn count(&self) -> u32 {
        match *self {
            Self::Ready { count, .. } => count,
            _ => unreachable!(),
        }
    }
}

fn main() {
    let input = include_str!("../input.txt");
    let nums = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u32>().unwrap());

    let bumps1 = nums.clone().fold(FoldOne::default(), FoldOne::fold);
    let bumps3 = nums.fold(FoldThree::Init, FoldThree::fold);

    println!("fold one {}", bumps1.count);
    println!("fold three {}", bumps3.count());
}
