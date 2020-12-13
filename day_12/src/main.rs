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

    for cmd in &cmds {
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
    }
    println!("x: {}, y: {}", x, y);
    println!("Part 1: {}", x.abs() + y.abs());

    part2(&cmds);
}

fn part2(cmds: &Vec<Command>) {
    
    let mut x = 0;
    let mut y = 0;
    let mut way_x = 10;
    let mut way_y = 1;

    for cmd in cmds {
        match cmd {
            Command::Right(deg) => {
                let rot = rot_right(deg/90, way_x, way_y);
                way_x = rot.0;
                way_y = rot.1;
            },
            Command::Left(deg) => {
                let rot = rot_left(deg/90, way_x, way_y);
                way_x = rot.0;
                way_y = rot.1;
            },
            Command::Forward(v) => {
                x += v * way_x;
                y += v * way_y;
            },
            Command::East(v) => way_x += v,
            Command::West(v) => way_x -= v,
            Command::North(v) => way_y += v,
            Command::South(v) => way_y -= v,
            _ => println!("FAIL"),
        }
    }
    println!("Part 2: {}", x.abs() + y.abs());
}

fn rot_right(mut times: i32, mut x: i32, mut y: i32) -> (i32, i32) {
    while times > 0 {
        let tmp = x;
        x = y;
        y = tmp;
        y *= -1;
        times -= 1;
    }
    return (x, y);
}

fn rot_left(mut times: i32, mut x: i32, mut y: i32) -> (i32, i32) {
    while times > 0 {
        let tmp = x;
        x = y;
        y = tmp;
        x *= -1;
        times -= 1;
    }
    return (x, y);
}
