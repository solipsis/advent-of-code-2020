use std::collections::HashSet;

#[derive(Debug)]
enum Direction {
    E,
    SE,
    SW,
    W,
    NW,
    NE,
}

const DAYS: usize = 100;

fn main() {
    let input: Vec<String> = std::fs::read_to_string("input.txt").unwrap()
        .trim()
        .split("\n")
        .map(|x| x.to_string())
        .collect();

    let mut state: HashSet<(isize,isize)> = HashSet::new();

    for line in &input {
        let mut directions: Vec<Direction> = Vec::new();

        let chars: Vec<char> = line.chars().collect();
        let mut idx = 0;
        while idx < line.len() {
            let dir = match chars[idx] {
                'e' => Direction::E, 
                'w' => Direction::W,
                's' => {
                    idx += 1;
                    let next = chars[idx];
                    match next {
                        'w' => Direction::SW,
                        'e' => Direction::SE,
                        _ => panic!("invalid south subdir"),
                    }
                }
                'n' => {
                    idx += 1;
                    let next = chars[idx];
                    match next {
                        'w' => Direction::NW,
                        'e' => Direction::NE,
                        _ => panic!("invalid north subdir"),
                    }
                }
                _ => panic!("invalid dir"),
            };
            directions.push(dir);

            idx += 1;
        }

        walk(&directions, &mut state);
    }
    println!("Part 1: {}", state.len());

    part2(state);
}

fn part2(mut state: HashSet<(isize, isize)>) {

    for _x in 0..DAYS {
        let mut next = state.clone();
        let adjacent: Vec<(isize, isize)> = vec![(2, 0), (-2, 0), (-1, -2), (-1, 2), (1, 2), (1, -2)];

        // all hexes to consider for changes
        let mut candidates: HashSet<(isize, isize)> = HashSet::new();
        for (x, y) in state.iter() {
            candidates.insert((*x,*y));
            for (dx,dy) in &adjacent {
                candidates.insert((x+dx, y+dy));
            }
        }

        for (x, y) in candidates.iter() {
            // tile is black
            if state.contains(&(*x,*y)) {
                let mut adj_black_count = 0;
                for (dx, dy) in &adjacent {
                    if state.contains(&(x+dx, y+dy)) {
                        adj_black_count += 1;
                    }
                }
                if adj_black_count == 0 || adj_black_count > 2 {
                    next.remove(&(*x,*y));
                }
            } else { // tile is white
                let mut adj_black_count = 0;
                for (dx, dy) in &adjacent {
                    if state.contains(&(x+dx, y+dy)) {
                        adj_black_count += 1;
                    }
                }
                if adj_black_count == 2 {
                    next.insert((*x,*y));
                }
            }
        }
        //println!("Day {}: {}", _x, next.len());
        state = next;
    }
    println!("Part 2: {}", state.len());
}

fn walk(dirs: &Vec<Direction>, state: &mut HashSet<(isize, isize)>) {
    let mut x: isize = 0;
    let mut y: isize = 0;

    for dir in dirs {
        match dir {
            Direction::E => x += 2,
            Direction::W => x -= 2,
            Direction::SW => {
                x -= 1;
                y -= 2;
            }
            Direction::NW => {
                x -= 1;
                y += 2;
            }
            Direction::NE => {
                x += 1;
                y += 2;
            }
            Direction::SE => {
                x += 1;
                y -= 2;
            }
        }
    }

    if state.contains(&(x,y)) {
        state.remove(&(x,y));
    } else {
        state.insert((x,y));
    }
}
