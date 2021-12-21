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

fn main() {
    let input = include_str!("../input.txt");
    let nums = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u32>().unwrap());

    let bumps = nums.fold(FoldOne::default(), FoldOne::fold);

    println!("{}", bumps.count);
}
