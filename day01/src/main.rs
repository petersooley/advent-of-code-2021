fn main() {
    let input = include_str!("../input.txt");
    let nums = input
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|x| x.parse::<u32>().unwrap());

    let mut bumps = 0;
    let mut prev = None;
    for i in nums {
        if let Some(p) = prev {
            if i > p {
                bumps += 1;
            }
        }
        prev = Some(i);
    }

    println!("{}", bumps);
}
