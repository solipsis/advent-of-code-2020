use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}
fn main() {
    let mut state: HashSet<Point> = HashSet::new();
    let mut next_state: HashSet<Point>;
    let mut to_consider: HashSet<Point> = HashSet::new();

    let input = std::fs::read_to_string("input.txt").unwrap();
    for (idx, line) in input.lines().enumerate() {
        for (c_idx, c) in line.chars().enumerate() {
            if c == '#' {
                state.insert(Point {
                    x: c_idx as i32,
                    y: idx as i32,
                    z: 0,
                    w: 0,
                });
            }
        }
    }

    for _cycle in 0..6 {
        for point in &state {
            for x in &[-1, 0, 1] {
                for y in &[-1, 0, 1] {
                    for z in &[-1, 0, 1] {
                        for w in &[-1, 0, 1] {
                            to_consider.insert(Point {
                                x: point.x + *x,
                                y: point.y + *y,
                                z: point.z + *z,
                                w: point.w + *w,
                            });
                        }
                    }
                }
            }
        }

        next_state = HashSet::new();
        for point in &to_consider {
            let mut active_cnt = 0;
            for x in &[-1, 0, 1] {
                for y in &[-1, 0, 1] {
                    for z in &[-1, 0, 1] {
                        for w in &[-1, 0, 1] {
                            if *x == 0 && *y == 0 && *z == 0 && *w == 0 {
                                continue;
                            }
                            if state.contains(&Point {
                                x: point.x + *x,
                                y: point.y + *y,
                                z: point.z + *z,
                                w: point.w + *w,
                            }) {
                                active_cnt += 1;
                            }
                        }
                    }
                }
            }

            // this point is already active
            if state.contains(&point) {
                if active_cnt == 2 || active_cnt == 3 {
                    next_state.insert(*point);
                }
            } else {
                if active_cnt == 3 {
                    next_state.insert(*point);
                }
            }
        }

        println!("Cycle: {}", _cycle + 1);
        println!("next_state {}", next_state.len());
        state = next_state.clone();
    }

}


