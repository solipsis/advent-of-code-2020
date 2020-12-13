#[derive(Debug)]
enum Command {
    Right(i32),
    Left(i32),
    Forward(i32),
    East(i32),
    West(i32),
    North(i32),
    South(i32),
    Invalid,
}

fn main() {
    let input: Vec<String> = std::fs::read_to_string("input.txt").unwrap()
        .lines()
        .map(|x| x.to_string())
        .collect();

    let mut x = 0;
    let mut y = 0;

    let dirs: Vec<(i32, i32)> = vec![(1,0),(0,-1),(-1,0),(0,1)];
    let mut direction_idx = 0;
    //let mut direction: (i32, i32) = dirs[directio];

    
    let mut cmds: Vec<Command> = Vec::new();
    for line in input {
        let cmd_char = line.chars().next().unwrap();
        let cmd: Command;
        let val: i32 = line[1..].parse().unwrap();
        match cmd_char {
            'R' => cmd = Command::Right(val),
            'L' => cmd = Command::Left(val),
            'F' => cmd = Command::Forward(val),
            'E' => cmd = Command::East(val),
            'W' => cmd = Command::West(val),
            'N' => cmd = Command::North(val),
            'S' => cmd = Command::South(val),
            _ => cmd = Command::Invalid,
        }
        cmds.push(cmd);
    }

    for cmd in cmds {
        match cmd {
            Command::Right(deg) => direction_idx = ((deg/90) + direction_idx) % 4,
            Command::Left(deg) => direction_idx = (4 - (deg/90) + direction_idx) % 4,
            Command::Forward(v) => {
                x += v * dirs[direction_idx as usize].0;
                y += v * dirs[direction_idx as usize].1;
            },
            Command::East(v) => x += v,
            Command::West(v) => x -= v,
            Command::North(v) => y += v,
            Command::South(v) => y -= v,
            _ => println!("FAIL"),
        }
        println!("cmd: {:?}", cmd);
        println!("x: {}, y: {}", x, y);
    }
    println!("x: {}, y: {}", x, y);
    println!("Part 1: {}", x.abs() + y.abs());

}
