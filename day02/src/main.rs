use std::str::FromStr;

enum Cmd {
    Forward(u32),
    Up(u32),
    Down(u32),
}
impl FromStr for Cmd {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 2 {
            return Err(format!("invalid cmd {}", s));
        }
        let cmd = parts[0];
        let val = parts[1];
        if cmd.starts_with("forward") {
            return Ok(Cmd::Forward(val.parse::<u32>().unwrap()));
        }
        if cmd.starts_with("down") {
            return Ok(Cmd::Down(val.parse::<u32>().unwrap()));
        }
        if cmd.starts_with("up") {
            return Ok(Cmd::Up(val.parse::<u32>().unwrap()));
        }
        Err(format!("unrecognized cmd {}", s))
    }
}

#[derive(Default, Debug)]
struct Position {
    depth: u32,
    horiz: u32,
}

fn main() {
    let cmds = include_str!("../input.txt")
        .split('\n')
        .filter(|s| !s.is_empty())
        .map(|l| l.parse::<Cmd>().unwrap());

    let pos = cmds.fold(Position::default(), |acc, x| match x {
        Cmd::Forward(v) => Position {
            horiz: acc.horiz + v,
            ..acc
        },
        Cmd::Down(v) => Position {
            depth: acc.depth + v,
            ..acc
        },
        Cmd::Up(v) => Position {
            depth: acc.depth - v,
            ..acc
        },
    });

    println!("pos {:?}, calc {}", pos, pos.depth * pos.horiz);
}
